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

bool compile(const char *source, Chunk *chunk) {
  initScanner(source);
  parser.hadError = false;
  parser.panicMode = false;
  // req next token from scanner
  advance();
  expression();
  consume(TOKEN_EOF, "Expect end of expression");
  return !parser.hadError;
}
