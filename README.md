# NOTES

# RESOURCES:

[Crafting Interpreters - Robert Nystrom](https://craftinginterpreters.com/) 


# Crafting Interpresters: Robert Nystrom

source code -> scanning -> tokens -> parsing -> AST -> traverse and interpret AST


# Compilation Flow
## Source Code:

The original code created by a programmer.
## Scanning (Lexical Analysis):

The scanner processes a linear stream of characters, grouping them into __tokens__ based on predefined rules.
A __lexeme__ is the smallest substring of source code conveying meaning. A __token__ consists of a lexeme along with additional useful data.

## Parsing into AST - Context-Free Grammar:

|-----------------------------------------------------------------------------|
|   Level of Grammar   |        Alphabet (Lexemes)   |        Strings         |
|------------------------------------------------------------------------------
| Lexical Analysis     | Individual characters       | Valid lexemes (tokens) |
|                      | (e.g., 'a', '1', '+', etc.) | (e.g., "if", "123", "+")|
|-----------------------------------------------------------------------------|
| Syntactic Analysis   | Entire tokens               | Sequences of tokens    |
| (Parser's Grammar)    | (e.g., 'if', '123', '+')    | (e.g., "if (x > 0)")   |
-------------------------------------------------------------------------------

The scanner breaks down the source code into lexemes during the lexical phase. The parser analyzes sequences of tokens during the syntactic analysis phase, creating an __Abstract Syntax Tree (AST)__ based on token relationships.

## AST Construction and Grammar Rules:

The AST is built based on grammar rules (productions), representing the hierarchical structure of the language. Grammar rules define how valid sequences of tokens can be assembled into meaningful expressions and statements.
Terminal: lexeme.
Non-Terminal: reference to another production rule.
The AST serves as an intermediate representation of the source code for further analysis.

#### STATEMENT GRAMAR RULES
| Production     | Expansion                                      |
|----------------|------------------------------------------------|
| program        | declaration* EOF ;                             |
| declaration    | varDecel | statement;                          |
| statement      | exprStmt                                       |
|                | printStmt                                      |
| exprStmt       | expression ";"                                 |
| printStmt      | "print" expression ";"                         |

#### EXPRESSION PRODUCTION RULES
| Production Rule | Syntax                                             | Description                                           |
|------------------|----------------------------------------------------|-------------------------------------------------------|
| expression       | `literal \| unary \| binary \| grouping`          | An expression can be a literal, unary, binary, or grouped expression. |
| literal          | `NUMBER \| STRING \| "true" \| "false" \| "nil"` | A literal can be a number, string, true, false, or nil. |
| grouping         | `("(" expression ")")`                             | A grouping is an expression enclosed in parentheses.   |
| unary            | `("-" \| "!") expression`                          | A unary operation is negation or logical NOT applied to an expression. |
| binary           | `expression operator expression`                   | A binary operation is an expression with an operator and another expression. |
| operator         | `"==" \| "!=" \| "<" \| "<=" \| ">" \| ">=" \| "+" \| "-" \| "*" \| "/"` | An operator can be equal, not equal, less than, less than or equal to, greater than, greater than or equal to, addition, subtraction, multiplication, or division. |


| Grammar notation | Code representation                                      |
|-------------------|----------------------------------------------------------|
| Terminal          | Code to match and consume a token                        |
| Nonterminal       | Call to that rule’s function                             |
| \|                | if or switch statement                                   |
| * or +            | while or for loop                                       |
| ?                 | if statement                                            |

# Parsing

## Recursive Descent Parsing

Top-down parser where each nonterminal in the grammar corresponds to a function, and parsing involves calling these functions recursively.

Recursive descent is considered a top-down parser because it starts from the top or outermost grammar rule (here expression) and works its way down into the nested subexpressions before finally reaching the leaves of the syntax tree.
The descent is described as “recursive” because when a grammar rule refers to itself—directly or indirectly—that translates to a recursive function call.

Top-down grammar rules in order of increasing precedence. It’s called “recursive descent” because it walks down the grammar. Confusingly, we also use direction metaphorically when talking about “high” and “low” precedence, but the orientation is reversed. In a top-down parser, you reach the lowest-precedence expressions first because they may in turn contain subexpressions of higher precedence.

__Expression__ produce values
__Statements__ produce side effects. Preform actions or control the flow of a program

In parser.rs you'll see that we'll keep passing the tokens until primary which are the leaves of the AST, we'll go deeper whilst respecting the rulst of precedence and association.

## Precedence and Associativity:

Rules determine the order of operator evaluation.
Ambiguity resolution for expressions like 6/3-1 involves rules of precedence and associativity.

__Precedence__ determines which operator is evaluated first in an expression containing a mixture of different operators. Precedence rules tell us that we evaluate the / before the - in the above example. Operators with higher precedence are evaluated before operators with lower precedence. Equivalently, higher precedence operators are said to “bind tighter”.

__Associativity__ determines which operator is evaluated first in a series of the same operator. When an operator is __left-associative__ (think “left-to-right”), operators on the left evaluate before those on the right. Since - is left-associative, this expression:

#### Association rules 

| Name       | Operators      | Associates |
|------------|-----------------|------------|
| Equality   | ==, !=          | Left       |
| Comparison | >, >=, <, <=    | Left       |
| Term       | -, +            | Left       |
| Factor     | /, *            | Left       |
| Unary      | !, -            | Right      |

#### Precedence rules

STATEMENT

| Rule            | Production                                                |
|-----------------|-----------------------------------------------------------|
| varDecl         |  → "var" IDENTIFIER  ("=" expression )? ";";  
| exprStmt        |  → expression ";" ;         |
| printStmt       |  → "print" expression ";" ;|

EXPRESSION:

| Rule      | Production                                            |
|-----------|-------------------------------------------------------|
| expression| → equality ;                                         |
| equality  | → comparison ( ( "!=" \| "==" ) comparison )* ;     |
| comparison| → term ( ( ">" \| ">=" \| "<" \| "<=" ) term )* ;    |
| term      | → factor ( ( "-" \| "+" ) factor )* ;                |
| factor    | → unary ( ( "/" \| "\*" ) unary )* ;                 |
| unary     | → ( "!" \| "-" ) unary \| primary ;                 |
| primary   | → NUMBER \| STRING \| "true" \| "false" \| "nil"    |
|           | \| "(" expression ")" ;                               |


### Panic mode error handling:
We need to report as many seperate errors whilst avoiding reporting errors that are the consequence of an earlier reported error - cascading errors. We do this through discarding the tokens until we find a production rule that matches the token stream - *synchronization*. Synchronization involves discarding input tokens until a recognizable point in the input stream is reached, allowing the parser to reestablish a valid state for parsing. This helps avoid the propagation of errors throughout the parsing process.

# INTERPRETER

## Evaluating Expressions
evaluate an expression and prodcure a value. Values are created by literals, computed by expressions and stored in a variable.
We need to be able to discern which type of value we're dealing with at runtime. + operator can add to numbers or concatenate two strings.
__Literal__ is a fixed value in the source code, this produces a value. A __value__ is a representation of data during the excution of a program - __runtime__. Values are dynamic and can change during the runtime of a program

## Binding or Resulotion
- identify what each identifier in the source code refers to.
  - local variable, function,...
  - `Scope` also ~ an important role in this stage
  - match `identifier` with the corresponding `declariation`

## Type checking 
- if the language is statically typed.

## Semantic information storage: 
- The AST is analyzed and all the info (variables, functions, scope,...) is stored into an `Intermediate Representation`
  - This is a lower level representation of the code (with still a higher level structure)
    - maintains control flow, data flow and relationships between different parts of the code.
  - more simplified and generic than the actual machine code
  - allows various `Optimizations`
  - THE BACKEND THAT FOCUSES ON TRANSFORMING THE IR INTO MACHINE CODE FOR THE TARGET ARCHITECTURE

## Code Generation
- `Native machine code` binary code that a specific machine can execute
- `Bytecode for a VM`: 
  - VM interpreters the bytecode a runtime
    - `Ahead of time compilation`
    - `just in time interpretation`
    - Java's JVM includes a Just-in-Time compiler that compiles frequently-executed bytecode into native code for improved performance, while less frequently executed code is interpreted. 


# RANDOM - cleanup later
  
### Compiler vs Interpreter
- A compiler takes an entire program and converts it into object code, which is a low-level, machine language version of the source code. The object code is stored on the disk and can be executed independently of the source code. Since the entire code is analyzed before execution, error detection and optimization can be more thorough, leading to efficient code execution.
  - Examples of compiled languages include C, C++, Rust, and Go.
- An interpreter translates and executes the source code line by line, not requiring a separate compilation step. As soon as the interpreter encounters an error, it stops translating and reports the error, which allows for easier debugging. However, it also means interpreted programs generally run slower than compiled ones, as translation occurs at runtime.
  - Examples of interpreted languages include Python, Ruby, and JavaScript.
- `JIT`:
  - In the context of languages like Java and JavaScript, when a program is run, its code is initially interpreted, which allows for quicker startup times as you don't have to wait for the whole program to be compiled. This process is similar to how a traditional interpreter works, and the code is typically converted into an intermediate representation (not quite machine code, but a lower-level format than the source code).
  - However, the JIT compiler monitors the program as it runs and identifies "hot spots" in the code — these are portions of the code that are executed frequently.
  - When such hot spots are identified, the JIT compiler kicks in and compiles those parts of the code directly to machine code. Since machine code executes more quickly than interpreted code, this results in a significant performance boost for those parts of the program. The machine code is also cached for future use, so if the same part of the code is encountered again, the already-compiled machine code can be reused.
