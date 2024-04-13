#ifndef clox_vm_h
#define clox_vm_h

#include "chunk.h"
#include <stdint.h>

#define STACK_MAX 256

typedef struct {
  Chunk *chunk;
  uint8_t *ip;
  Value stack[STACK_MAX];
  // NOTE: we use a pointer, to track the top of the stack, because it's faster
  // to dereference a pointer than to calculate the offset from the index each
  // time we need it. The top points to where the next value in the stack will
  // go
  Value *stackTop;
  Obj *objects;
} VM;

typedef enum {
  INTERPRET_OK,
  INTERPRET_COMPILE_ERROR,
  INTERPRET_RUNTIME_ERROR
} InterpretResult;

extern VM vm;

void initVM();
void freeVM();
InterpretResult interpret(const char *source);
void push(Value value);
Value pop();

#endif
