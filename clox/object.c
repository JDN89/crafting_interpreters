#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "memory.h"
#include "object.h"
#include "value.h"
#include "vm.h"

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
  object->next = vm.objects;
  vm.objects = object;

  return object;
}

static ObjString *allocateString(char *chars, int length, uint32_t hash) {
  ObjString *string = ALLOCATE_OBJ(ObjString, OBJ_STRING);
  string->length = length;
  string->chars = chars;
  string->hash = hash;
  return string;
}

static uint32_t hashString(const char *key, int length) {
  uint32_t hash = 2166136261u;
  for (int i = 0; i < length; i++) {
    hash ^= (uint8_t)key[i];
    hash *= 16777619;
  }
  return hash;
}

// NOTE: When we created a new String we made a copy and placed it on the heap
// with concatenation we take owenership of the previous created character
// arrays on the heap. No need to create and copy a new one
//

ObjString *takeString(char *chars, int length) {
  uint32_t hash = hashString(chars, length);
  return allocateString(chars, length, hash);
}

ObjString *copyString(const char *chars, int length) {
  uint32_t hash = hashString(chars, length);
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
  return allocateString(heapChars, length, hash);
}

void printObject(Value value) {
  switch (OBJ_TYPE(value)) {
  case OBJ_STRING:
    printf("%s", AS_CSTRING(value));
    break;
  }
}
