#include "vm.h"
#include "chunk.h"
#include "common.h"
#include "compiler.h"
#include "debug.h"
#include "value.h"
#include <stdarg.h>
#include <stdio.h>

// Author decided to declare global vm instead of declaring a pointer to the vm
// as to save lines of code
// NOTE: Taking a VM pointer and passing it around would have been a better
// design choice, because it allows for more flexibility see:
// http://gameprogrammingpatterns.com/singleton.html

VM vm;

// The top points to the beginning of the array
static void resetStack() { vm.stackTop = vm.stack; }

static void runtimeError(const char *format, ...) {
  va_list args;
  va_start(args, format);
  vfprintf(stderr, format, args);
  va_end(args);
  fputs("\n", stderr);

  // Index -1 because the interpreter advances past each instruction before
  // executing it
  size_t instruction = vm.ip - vm.chunk->code - 1;
  int line = vm.chunk->lines[instruction];
  fprintf(stderr, "[line %d] in script\n", line);
  resetStack();
}

void initVM() { resetStack(); }

void freeVM() {}

// NOTE: value is for now a double, because we only do arithmetics for the
// moment and so we only need to store numbers

void push(Value value) {
  // NOTE: we dereference the stackTop pointer to put a value on the vm.stack
  // array
  *vm.stackTop = value;
  vm.stackTop++;
}

Value pop() {
  vm.stackTop--;
  return *vm.stackTop;
}

static Value peek(int distance) { return vm.stackTop[-1 - distance]; }

static bool isFalsey(Value value) {
  return IS_NIL(value) || (IS_BOOL(value) && !AS_BOOL(value));
}

static InterpretResult run() {
  // *vm.ip dereferences the pointer, returns the value stored in memory
  // This is a fundamental operation in pointer manipulation
  // READ BYTE moves the pointer to the next byteCode in the Chunk.code array
  // In this case the index of the constant in the valueArray
#define READ_BYTE() (*vm.ip++)
#define READ_CONSTANT() (vm.chunk->constants.values[READ_BYTE()])

  // NOTE: whe use to do while loop to keep the statements in the same scope ->
  // C tricky trick
  // we evaluate left to right -> right is top of stack so we assign the first
  // pop to value : b

#define BINARY_OP(valueType, op)                                               \
  do {                                                                         \
    if (!IS_NUMBER(peek(0)) || !IS_NUMBER(peek(1))) {                          \
      runtimeError("Operands must be numbers.");                               \
      return INTERPRET_RUNTIME_ERROR;                                          \
    }                                                                          \
    double b = AS_NUMBER(pop());                                               \
    double a = AS_NUMBER(pop());                                               \
    push(valueType(a op b));                                                   \
  } while (false)

  for (;;) {
    // NOTE: To get visibility into the stack: we’ll show the current contents
    // of the stack before we interpret each instruction.
#ifdef DEBUG_TRACE_EXECUTION

    printf("          ");
    // NOTE: pointer points to beginning of stack and we move it up to the top
    for (Value *slot = vm.stack; slot < vm.stackTop; slot++) {
      printf("[ ");
      printValue(*slot);
      printf(" ]");
    }
    printf("\n");
    disassembleInstruction(vm.chunk, (int)(vm.ip - vm.chunk->code));
#endif

    uint8_t instruction;
    switch (instruction = READ_BYTE()) {
    case OP_CONSTANT: {
      Value constant = READ_CONSTANT();
      push(constant);
      break;
    }

    case OP_NIL:
      push(NIL_VAL);
      break;
    case OP_TRUE:
      push(BOOL_VAL(true));
      break;
    case OP_FALSE:
      push(BOOL_VAL(false));
      break;

    case OP_EQUAL: {
      Value b = pop();
      Value a = pop();
      push(BOOL_VAL(valuesEqual(a, b)));
      break;
    }

      // We wrap the result before pushing it on the stack by passing the
      // wrapping macro as a paramter
    case OP_ADD:
      BINARY_OP(NUMBER_VAL, +);
      break;
    case OP_SUBTRACT:
      BINARY_OP(NUMBER_VAL, -);
      break;
    case OP_MULTIPLY:
      BINARY_OP(NUMBER_VAL, *);
      break;
    case OP_DIVIDE:
      BINARY_OP(NUMBER_VAL, /);
      break;
    case OP_NOT:
      push(BOOL_VAL(isFalsey(pop())));
      break;
    case OP_GREATER:
      BINARY_OP(BOOL_VAL, >);
      break;
    case OP_LESS:
      BINARY_OP(BOOL_VAL, <);
      break;
    case OP_NEGATE:
      if (!IS_NUMBER(peek(0))) {
        runtimeError("Operand must be a number.");
        return INTERPRET_RUNTIME_ERROR;
      }
      push(NUMBER_VAL(-AS_NUMBER(pop())));
      break;

    case OP_RETURN: {
      printValue(pop());
      printf("\n");
      return INTERPRET_OK;
    }
    }
  }

#undef READ_BYTE
#undef READ_CONSTANT
#undef BINARY_OP
}

InterpretResult interpret(const char *source) {
  Chunk chunk;
  initChunk(&chunk);

  if (!compile(source, &chunk)) {
    freeChunk(&chunk);
    return INTERPRET_COMPILE_ERROR;
  }

  vm.chunk = &chunk;
  vm.ip = vm.chunk->code;

  printf("CHUNK CONSTANT ARRAY:  ");
  for (int i = 0; i < vm.chunk->constants.count; i++) {
    printf("[ ");
    printValue(vm.chunk->constants.values[i]);
    printf(" ]\n");
  }

  InterpretResult result = run();

  freeChunk(&chunk);
  return result;
}
