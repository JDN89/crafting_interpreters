#ifndef clox_memory_h
#define clox_memory_h

#include "common.h"
#include "object.h"

// sizeof(type) gives the size of each element of the array being allocated.
// (count) is the number of elements in the array.
// sizeof(type) * (count) calculates the total size required for the array in
// bytes.
#define ALLOCATE(type, count)                                                  \
  (type *)reallocate(NULL, 0, sizeof(type) * (count))

#define FREE(type, pointer) reallocate(pointer, sizeof(type), 0)

// Grow capacity grows the capacity field and keeps track of the number of
// elements the array can store
#define GROW_CAPACITY(capacity) ((capacity) < 8 ? 8 : (capacity) * 2)

// Grow array does the actual memory allocation or resizing based on the type it
// hodls and the count
#define GROW_ARRAY(type, pointer, oldCount, newCount)                          \
  (type *)reallocate(pointer, sizeof(type) * (oldCount),                       \
                     sizeof(type) * (newCount))

#define FREE_ARRAY(type, pointer, oldCount)                                    \
  reallocate(pointer, sizeof(type) * (oldCount), 0)

void *reallocate(void *pointer, size_t oldSize, size_t newSize);

void freeObjects();

#endif
