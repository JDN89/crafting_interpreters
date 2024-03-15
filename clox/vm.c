#include "vm.h"
#include "chunk.h"
#include "common.h"
#include "debug.h"
#include "value.h"
#include <stdio.h>

// Author decided to declare global vm instead of declaring a pointer to the vm
// as to save lines of code
// NOTE: Taking a VM pointer and passing it around would have been a better
// design choice, because it allows for more flexibility see:
// http://gameprogrammingpatterns.com/singleton.html

VM vm;

// The top points to the beginning of the array
static void resetStack() { vm.stackTop = vm.stack; }

void initVM() { resetStack(); }

void freeVM() {}

// NOTE: value is for now a double, because we only do arithmetics for the
// moment and so we only need to store numbers
void push(Value value) {
  *vm.stackTop = value;
  vm.stackTop++;
}

Value pop() {
  vm.stackTop--;
  return *vm.stackTop;
}

static InterpretResult run() {
  // *vm.ip dereferences the pointer, returns the value stored in memory
  // This is a fundamental operation in pointer manipulation
  // READ BYTE moves the pointer to the next byteCode in the Chunk.code array
#define READ_BYTE() (*vm.ip++)
#define READ_CONSTANT() (vm.chunk->constants.values[READ_BYTE()])

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
    case OP_NEGATE:
      push(-pop());
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
}

InterpretResult interpret(Chunk *chunk) {
  vm.chunk = chunk;
  // NOTE: vm.ip is a pointer that points to the bytecode being executed.
  // Sets the ip field of the VM structure to the beginning of the bytecode
  // array (chunk->code).
  vm.ip = vm.chunk->code;
  return run();
}
