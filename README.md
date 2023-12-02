# NOTES

# RESOURCES:

[Discussion Jonathan Blow](https://www.youtube.com/watch?v=MnctEW1oL-E&t=3153s)
[Crafting Interpreters - Robert Nystrom](https://craftinginterpreters.com/) 


# Crafting Interpresters: Robert Nystrom

## Scanning or Lexing - regular language
The __scanner__ processes a linear stream of characters, grouping them into __tokens__ based on predefined rules. A __lexeme__ is the smallest substring of source code, representing the most basic sequence of characters that conveys meaning. A __token__ comprises a lexeme along with additional useful data.
  - Identifier: min
  - Keyword: if
  - Literal: 123
  - Operator: + 
  - Separator: ,

## Parsing into AST - context free grammar

+----------------------+-----------------------------+------------------------+
|   Level of Grammar   |        Alphabet (Lexemes)   |        Strings         |
+----------------------+-----------------------------+------------------------+
| Lexical Analysis     | Individual characters       | Valid lexemes (tokens) |
|                      | (e.g., 'a', '1', '+', etc.) | (e.g., "if", "123", "+")|
+----------------------+-----------------------------+------------------------+
| Syntactic Analysis   | Entire tokens               | Sequences of tokens    |
| (Parser's Grammar)    | (e.g., 'if', '123', '+')    | (e.g., "if (x > 0)")   |
+----------------------+-----------------------------+------------------------+

The lexical phase the scanner broked down the source code and transformed them into lexemes. In the syntactic analysis phase the parser analyzes sequence of tokens and puts them into an AST based on the relationship between the tokens.

The AST is constructed based on Grammar rules called productions and represents the hierarchical structure of the language. The grammar rules define how valid sequences of tokens (lexemes) can be assembled into meaningful expressions and statements

- Terminal: lexeme.
- Non Terminal: reference to another production rule.

The AST is an intermediate representation of the source code and is used for further analysis.


| Production Rule | Syntax                                             | Description                                           |
|------------------|----------------------------------------------------|-------------------------------------------------------|
| expression       | `literal \| unary \| binary \| grouping`          | An expression can be a literal, unary, binary, or grouped expression. |
| literal          | `NUMBER \| STRING \| "true" \| "false" \| "nil"` | A literal can be a number, string, true, false, or nil. |
| grouping         | `("(" expression ")")`                             | A grouping is an expression enclosed in parentheses.   |
| unary            | `("-" \| "!") expression`                          | A unary operation is negation or logical NOT applied to an expression. |
| binary           | `expression operator expression`                   | A binary operation is an expression with an operator and another expression. |
| operator         | `"==" \| "!=" \| "<" \| "<=" \| ">" \| ">=" \| "+" \| "-" \| "*" \| "/"` | An operator can be equal, not equal, less than, less than or equal to, greater than, greater than or equal to, addition, subtraction, multiplication, or division. |

## Parsing Expressions
Given a valid sequence of tokens, produce a corresponding syntax tree.
Given an invalid sequence of tokens, detact any errors and tell the user about their mistakes.

Given a series of tokens we map the tokens to a terminal to figure out which rule could have generated this token.
We have one issues, right now there is som ambiguity as to how the series of tokens were produced.
- 6/3-1
Starting at expression, pick binary.
For the left-hand expression, pick NUMBER, and use 6.
For the operator, pick "/".
For the right-hand expression, pick binary again.
In that nested binary expression, pick 3 - 1.

Another is:

Starting at expression, pick binary.
For the left-hand expression, pick binary again.
In that nested binary expression, pick 6 / 3.
Back at the outer binary, for the operator, pick "-".
For the right-hand expression, pick NUMBER, and use 1.

To avoid this we have rules of precedence and Associativity. Precedence determines which operator is evaluated first in an expression containing a mixture of different operators. Associativity determines which operator is evaluated first in a series of the same operator.

| Rule            | Production                                                |
|-----------------|-----------------------------------------------------------|
| expression      | → equality ;                                             |
| equality        | → comparison ( ( "!=" \| "==" ) comparison )* ;         |
| comparison      | → term ( ( ">" \| ">=" \| "<" \| "<=" ) term )* ;        |
| term            | → factor ( ( "-" \| "+" ) factor )* ;                    |
| factor          | → unary ( ( "/" \| "*" ) unary )* ;                      |
| unary           | → ( "!" \| "-" ) unary \| primary ;                     |
| primary         | → NUMBER \| STRING \| "true" \| "false" \| "nil"         |
|                 | \| "(" expression ")" ;                                   |

#### Recursive Descent Parsing

Top-down parser where each nonterminal in the grammar corresponds to a function, and parsing involves calling these functions recursively.

Recursive descent is considered a top-down parser because it starts from the top or outermost grammar rule (here expression) and works its way down into the nested subexpressions before finally reaching the leaves of the syntax tree.

Top-down grammar rules in order of increasing precedence. It’s called “recursive descent” because it walks down the grammar. Confusingly, we also use direction metaphorically when talking about “high” and “low” precedence, but the orientation is reversed. In a top-down parser, you reach the lowest-precedence expressions first because they may in turn contain subexpressions of higher precedence.


| Grammar notation | Code representation                                      |
|-------------------|----------------------------------------------------------|
| Terminal          | Code to match and consume a token                        |
| Nonterminal       | Call to that rule’s function                             |
| \|                | if or switch statement                                   |
| * or +            | while or for loop                                       |
| ?                 | if statement                                            |

The descent is described as “recursive” because when a grammar rule refers to itself—directly or indirectly—that translates to a recursive function call.



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
