#ifndef clox_chunk_h
#define clox_chunk_h

#include "common.h"
#include <cstdint>
#include <functional>

typedef enum {
  OP_RETURN,
} OpCode;

typedef struct {
  uint8_t *code;
} Chunk;

#endif
