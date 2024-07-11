#include <stdio.h>

// Define our bytecode instructions (opcodes)
typedef enum {
  OP_CONSTANT,
  OP_ADD,
  OP_SUBTRACT,
  OP_MULTIPLY,
  OP_DIVIDE,
  OP_RETURN,
} OpCode;

// Example bytecode representing a simple arithmetic operation: 2 + 3
unsigned char bytecode[] = {
    OP_CONSTANT, 0, // Load constant 0 (value 2)
    OP_CONSTANT, 1, // Load constant 1 (value 3)
    OP_ADD,         // Add the top two values on the stack
    OP_RETURN       // Return the result
};

// Simple bytecode interpreter function
void interpret(unsigned char *bytecode) {
  int stack[256]; // Simple stack for operands
  int sp = -1;    // Stack pointer

  int constants[] = {2, 3}; // Constants used in the bytecode

  // Loop through the bytecode
  for (int ip = 0;; ip++) {
    switch (bytecode[ip]) {
    case OP_CONSTANT: {
      int constantIndex = bytecode[ip + 1];
      sp++;
      stack[sp] = constants[constantIndex];
      ip += 1; // Move instruction pointer to next byte (skip constant index)
      break;
    }
    case OP_ADD: {
      int b = stack[sp--];
      int a = stack[sp--];
      sp++;
      stack[sp] = a + b;
      break;
    }
    case OP_RETURN: {
      printf("Result: %d\n", stack[sp]);
      return;
    }
    default:
      printf("Unknown opcode %d\n", bytecode[ip]);
      return;
    }
  }
}

int main() {
  interpret(bytecode);
  return 0;
}
