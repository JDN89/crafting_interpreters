# JLox, A Tree Walk Interpreter

## Project
- 
- javac com/craftinginterpreters/tool/GenerateAst.java
- java com/craftinginterpreters/tool/GenerateAst .
  - change . to the right destination folder

## Side notes
- Lox is a scripting language -> executes directly from source
  - interpreted : code is executed directly by an interpreter without the need for compilation.
- Interactive prompt =  REPL 
  - Read line of input, Evaluate it, Print the result, Loop

## Lox Expressions

### Non-terminal symbols:
expression
literal
grouping
unary
binary
operator

### Production rules:
expression can be a literal, unary, binary, or grouping.
literal can be a NUMBER, STRING, "true", "false", or "nil".
grouping is an expression enclosed in parentheses.
unary is either a unary minus - or unary not ! followed by an expression.
binary is an expression followed by an operator and another expression.
operator can be "==", "!=", "<", "<=", ">", ">=", "+", "-", "*", or "/".