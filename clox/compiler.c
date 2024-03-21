#include <stdio.h>
#include <stdlib.h>

#include "common.h"
#include "compiler.h"
#include "scanner.h"

typedef struct {
  Token current;
  Token previous;
  bool hadError;
  bool panicMode;
} Parser;

typedef enum {
  PREC_NONE,
  PREC_ASSIGNMENT, // =
  PREC_OR,         // or
  PREC_AND,        // and
  PREC_EQUALITY,   // == !=
  PREC_COMPARISON, // < > <= >=
  PREC_TERM,       // + -
  PREC_FACTOR,     // * /
  PREC_UNARY,      // ! -
  PREC_CALL,       // . ()
  PREC_PRIMARY
} Precedence;

// we have a single global variable of this struct type so we don’t need to pass
// the state around from function to function in the compiler.
//
// if we pass the state we do it like via a pointer
// static void advance(Parser *parser) {
// Example usage
// Accessing current token: parser->current
// Accessing previous token: parser->previous
// For demonstration purposes, let's print current token value
// printf("Current token value: %s\n", parser->current.value);
// }

Parser parser;
Chunk *compilingChunk;

static Chunk *currentChunk() { return compilingChunk; }

static void errorAt(Token *token, const char *message) {
  if (parser.panicMode)
    return;
  parser.panicMode = true;
  fprintf(stderr, "[line %d] Error", token->line);

  if (token->type == TOKEN_EOF) {
    fprintf(stderr, " at end");
  } else if (token->type == TOKEN_ERROR) {
    // Nothing.
  } else {
    fprintf(stderr, " at '%.*s'", token->length, token->start);
  }

  fprintf(stderr, ": %s\n", message);
  parser.hadError = true;
}

static void error(const char *message) { errorAt(&parser.previous, message); }

static void errorAtCurrent(const char *message) {
  errorAt(&parser.current, message);
}

static void advance() {
  parser.previous = parser.current;

  for (;;) {
    parser.current = scanToken();
    if (parser.current.type != TOKEN_ERROR)
      break;

    errorAtCurrent(parser.current.start);
  }
}
// It’s similar to advance() in that it reads the next token. But it also
// validates that the token has an expected type. If not, it reports an error.
// This function is the foundation of most syntax errors in the compiler.
static void consume(TokenType type, const char *message) {
  if (parser.current.type == type) {
    advance();
    return;
  }

  errorAtCurrent(message);
}

// currentChunk is a wrapper around chunk pointer that we pass to compile
static void emitByte(uint8_t byte) {
  writeChunk(currentChunk(), byte, parser.previous.line);
}

// for the OP_CODE followed by one byte operands
static void emitBytes(uint8_t byte1, uint8_t byte2) {
  emitByte(byte1);
  emitByte(byte2);
}

static void emitReturn() { emitByte(OP_RETURN); }

static uint8_t makeConstant(Value value) {
  int constant = addConstant(currentChunk(), value);
  if (constant > UINT8_MAX) {
    error("Too many constants in one chunk.");
    return 0;
  }

  return (uint8_t)constant;
}

static void emitConstant(Value value) {
  emitBytes(OP_CONSTANT, makeConstant(value));
}

static void parsePrecedence(Precedence precedence) {
  // What goes here?
}

static void endCompiler() { emitReturn(); }

static void expression() {
  parsePrecedence(PREC_ASSIGNMENT);
  // What goes here?
}

// NOTE: Again, we assume the initial ( has already been consumed. We
// recursively call back into expression() to compile the expression between the
// parentheses, then parse the closing ) at the end.
//
static void grouping() {
  expression();
  consume(TOKEN_RIGHT_PAREN, "Expect ')' after expression.");
}

static void number() {
  double value = strtod(parser.previous.start, NULL);
  emitConstant(value);
}

// NOTE: we evaluate the operand first which leaves it value on the stack
// then we po the value -> negate it and pus the result back on the stack
// OP_NEGATE | OPERAND VALUE ==> LIFO stack -> negate instruciton pop value push
// result back
static void unary() {
  TokenType operatorType = parser.previous.type;

  // Compile the operand.

  parsePrecedence(PREC_UNARY);

  // Emit the operator instruction.
  switch (operatorType) {
  case TOKEN_MINUS:
    emitByte(OP_NEGATE);
    break;
  default:
    return; // Unreachable.
  }
}

bool compile(const char *source, Chunk *chunk) {
  initScanner(source);
  // store pointer in local variable, so we can pass it to emitByteCode
  compilingChunk = chunk;
  parser.hadError = false;
  parser.panicMode = false;
  // req next token from scanner
  advance();
  expression();
  consume(TOKEN_EOF, "Expect end of expression");
  endCompiler();
  return !parser.hadError;
}
