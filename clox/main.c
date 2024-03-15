#include "chunk.h"
#include "common.h"
#include "debug.h"
#include "vm.h"

// NOTE: GENERAL SETUP
// Chunk contains code array and value array
// code array: OP_CODES + index of the value store in de Values array
// VM -> we pass the chunk to the vm where we interpret the chunk -> swithc on
// OP_code and lookup the corresponding value if necessary

int main(int argc, const char *argv[]) {
  initVM();
  Chunk chunk;
  initChunk(&chunk);
  int constant = addConstant(&chunk, 1.2);
  writeChunk(&chunk, OP_CONSTANT, 123);
  // add constatnt return the index
  // so we store the index in de code array instead of the actual constatn
  // value!
  writeChunk(&chunk, constant, 123);
  writeChunk(&chunk, OP_NEGATE, 123);

  writeChunk(&chunk, OP_RETURN, 123);

  disassembleChunk(&chunk, "test chunk");
  interpret(&chunk);
  freeVM();
  freeChunk(&chunk);
  return 0;
}
