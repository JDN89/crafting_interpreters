#include <stdio.h>
#include <string.h>

#include "memory.h"
#include "object.h"
#include "value.h"

// NOTE: Like the previous macro, this exists mainly to avoid the need to
// redundantly cast a void* back to the desired type. The actual functionality
// is here: realocate is a wrapper around c's realloc which returns null,
// With this we don't have to case each time the return type void to the
// appropriate type
#define ALLOCATE_OBJ(type, objectType)                                         \
  (type *)allocateObject(sizeof(type), objectType)

static Obj *allocateObject(size_t size, ObjType type) {
  Obj *object = (Obj *)reallocate(NULL, 0, size);
  object->type = type;
  return object;
}

static ObjString *allocateString(char *chars, int length) {
  ObjString *string = ALLOCATE_OBJ(ObjString, OBJ_STRING);
  string->length = length;
  string->chars = chars;
  return string;
}

ObjString *copyString(const char *chars, int length) {
  char *heapChars = ALLOCATE(char, length + 1);

  // memcpy is a standard C function that copies length bytes from the memory
  // location pointed to by chars to the memory location pointed to by
  // heapChars.

  // WARNING: In many programming languages, including C, string literals (such
  // as "hello") are typically stored in read-only memory. This means that the
  // memory locations where these string literals are stored cannot be modified
  // at runtime. Any attempt to modify them will result in a runtime error or
  // undefined behavior.
  // NOTE: We copy over the characters because we want to
  // own them. if we keep referencing the source code via a pointer we create
  // issues when we dereference the objStringArray in the future and we're
  // freeing read only memory. To avoid these issues, it's safer to preemptively
  // copy the characters from the original source code string (such as string
  // literals) to dynamically allocated memory (the heap) when creating an
  // ObjString
  memcpy(heapChars, chars, length);
  heapChars[length] = '\0';
  return allocateString(heapChars, length);
}
