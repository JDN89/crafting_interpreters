#include "vm.h"
#include "chunk.h"
#include "common.h"
#include "debug.h"
#include "value.h"
#include <stdio.h>

// Author decided to declare global vm instead of declaring a pointer to the vm
// as to save lines of code
// Taking a VM pointer and passing it around would have been a better design
// choice, because it allows for more flexibility
// see: http://gameprogrammingpatterns.com/singleton.html

VM vm;

void initVM() {}

void freeVM() {}

static InterpretResult run() {
  // *vm.ip dereferences the pointer, returns the value stored in memory
  // This is a fundamental operation in pointer manipulation
  // READ BYTE moves the pointer to the next byteCode in the Chunk.code array
#define READ_BYTE() (*vm.ip++)
#define READ_CONSTANT() (vm.chunk->constants.values[READ_BYTE()])

  for (;;) {
#ifdef DEBUG_TRACE_EXECUTION
    disassembleInstruction(vm.chunk, (int)(vm.ip - vm.chunk->code));
#endif

    uint8_t instruction;
    switch (instruction = READ_BYTE()) {
    case OP_CONSTANT: {
      Value constant = READ_CONSTANT();
      printValue(constant);
      printf("\n");
      break;
    }
    case OP_RETURN: {
      return INTERPRET_OK;
    }
    }
  }

#undef READ_BYTE
#undef READ_CONSTANT
}

InterpretResult interpret(Chunk *chunk) {
  vm.chunk = chunk;
  // vm.ip is a pointer that points to the bytecode being executed.
  // Sets the ip field of the VM structure to the beginning of the bytecode
  // array (chunk->code).
  vm.ip = vm.chunk->code;
  return run();
}
