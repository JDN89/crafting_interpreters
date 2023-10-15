# JLox, A Tree Walk Interpreter

## Scanning

Convert raw source code in `tokens` (keyword, identifier, operator, separator, literal)

- keyword: if, while, for,...
- Identifiers: name of var, functions, obj,...
- Operator: + / -
- Separators: ; , { ...
- Literals: values that are written directly into the source code. These are fixed unchanging values. 

## Side notes
- Lox is a scripting language -> executes directly from source
  - interpreted : code is executed directly by an interpreter without the need for compilation.
- Interactive prompt =  REPL 
  - Read line of input, Evaluate it, Print the result, Loop