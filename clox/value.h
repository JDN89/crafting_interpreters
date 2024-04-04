#ifndef clox_value_h
#define clox_value_h

#include "common.h"

typedef struct Obj Obj;

typedef struct ObjString ObjString;

typedef enum { VAL_BOOL, VAL_NIL, VAL_NUMBER, VAL_OBJ } ValueType;

typedef struct {
  ValueType type;
  // tagged union. The size of the union is the size of it largest field
  // the fields overlap in memory. But caution, don't have boolean and later
  // access it as a number!
  union {
    bool boolean;
    double number;
    Obj *obj;
  } as;
} Value;

// typechecking to execute everytime we call AS_ macros
#define IS_BOOL(value) ((value).type == VAL_BOOL)
#define IS_NIL(value) ((value).type == VAL_NIL)
#define IS_NUMBER(value) ((value).type == VAL_NUMBER)
#define IS_OBJ(value) ((value).type == VAL_OBJ)

// Convert Clox value to a native C value
#define AS_BOOL(value) ((value).as.boolean)
#define AS_NUMBER(value) ((value).as.number)
#define AS_OBJ(value) ((value).as.obj)

// convert native C value to a Clox value
#define BOOL_VAL(value) ((Value){VAL_BOOL, {.boolean = value}})
#define NIL_VAL ((Value){VAL_NIL, {.number = 0}})
#define NUMBER_VAL(value) ((Value){VAL_NUMBER, {.number = value}})

// (Obj*) object: This casts object, which is assumed to be a pointer, to type
// Obj*. It essentially converts the pointer to a pointer of type Obj.*:
// The asterisk (*) is the dereference operator. It is used to access the value
// pointed to by a pointer. In this context, it's used to dereference the
// pointer object, accessing the actual Obj value it points to.

#define OBJ_VAL(object) ((Value){VAL_OBJ, {.obj = (Obj *)object}})

typedef struct {
  int capacity;
  int count;
  Value *values;
} ValueArray;

bool valuesEqual(Value a, Value b);

void initValueArray(ValueArray *array);
void writeValueArray(ValueArray *array, Value value);
void freeValueArray(ValueArray *array);
void printValue(Value value);

#endif
