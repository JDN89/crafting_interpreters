# NOTES

# RESOURCES:

[Discussion Jonathan Blow](https://www.youtube.com/watch?v=MnctEW1oL-E&t=3153s)
[Crafting Interpreters - Robert Nystrom] (https://craftinginterpreters.com/) 


# Crafting Interpresters: Robert Nystrom

## CHAPTER 1: INTRODUCTION
### Scanning or Lexing
The __scanner__ processes a linear stream of characters, grouping them into _tokens__ based on predefined rules. A __lexeme__ is the smallest substring of source code, representing the most basic sequence of characters that conveys meaning. A __token__ comprises a lexeme along with additional useful data.
  - Identifier: min
  - Keyword: if
  - Literal: 123
  - Operator: + 
  - Separator: ,

#### Parsing
- Put the tokens and build an Abstract Syntax Tree that represents the program's syntax.
- The tree is then used by the compiler to produce the next steps of the process

3 + 4 * 5
- first multiplication then addition
        +
       / \
      3   *
         / \
        4   5
- The parser gives structure and meaning to the flat tokens

#### Binding or Resulotion
- identify what each identifier in the source code refers to.
  - local variable, function,...
  - `Scope` also ~ an important role in this stage
  - match `identifier` with the corresponding `declariation`
#### Type checking 
- if the language is statically typed.

#### Semantic information storage: 
- The AST is analyzed and all the info (variables, functions, scope,...) is stored into an `Intermediate Representation`
  - This is a lower level representation of the code (with still a higher level structure)
    - maintains control flow, data flow and relationships between different parts of the code.
  - more simplified and generic than the actual machine code
  - allows various `Optimizations`
  - THE BACKEND THAT FOCUSES ON TRANSFORMING THE IR INTO MACHINE CODE FOR THE TARGET ARCHITECTURE

#### Code Generation
- `Native machine code` binary code that a specific machine can execute
- `Bytecode for a VM`: 
  - VM interpreters the bytecode a runtime
    - `Ahead of time compilation`
    - `just in time interpretation`
    - Java's JVM includes a Just-in-Time compiler that compiles frequently-executed bytecode into native code for improved performance, while less frequently executed code is interpreted. 
  
### Compiler vs Interpreter
- A compiler takes an entire program and converts it into object code, which is a low-level, machine language version of the source code. The object code is stored on the disk and can be executed independently of the source code. Since the entire code is analyzed before execution, error detection and optimization can be more thorough, leading to efficient code execution.
  - Examples of compiled languages include C, C++, Rust, and Go.
- An interpreter translates and executes the source code line by line, not requiring a separate compilation step. As soon as the interpreter encounters an error, it stops translating and reports the error, which allows for easier debugging. However, it also means interpreted programs generally run slower than compiled ones, as translation occurs at runtime.
  - Examples of interpreted languages include Python, Ruby, and JavaScript.
- `JIT`:
  - In the context of languages like Java and JavaScript, when a program is run, its code is initially interpreted, which allows for quicker startup times as you don't have to wait for the whole program to be compiled. This process is similar to how a traditional interpreter works, and the code is typically converted into an intermediate representation (not quite machine code, but a lower-level format than the source code).
  - However, the JIT compiler monitors the program as it runs and identifies "hot spots" in the code — these are portions of the code that are executed frequently.
  - When such hot spots are identified, the JIT compiler kicks in and compiles those parts of the code directly to machine code. Since machine code executes more quickly than interpreted code, this results in a significant performance boost for those parts of the program. The machine code is also cached for future use, so if the same part of the code is encountered again, the already-compiled machine code can be reused.
