## TODO
- format BinExpr,... so you can print a nice AST and paste it in the readme
- test al scenarios: does the interpreter work 
- see if you can replace match in some places with __if let__, better to use in combination with enums where I'm only interested in one specific enum field
- check equality, behaviour is weird? >5=6=7 >Ok("5")
- after finishing up the parser,write a test for parser and scanner and see if you can speed it up through removing the clonse and passing the reference. Check AOC 2022 for axmple of optimizing and measuring speed (I put some explenation there in the past).
- remove #[allow(dead_code, unused_variables)]

# Precedence rules
| Rule            | Production                                                |
|-----------------|-----------------------------------------------------------|

| varDecl         |  → "var" IDENTIFIER  ("=" expression )? ";";  
| exprStmt        |  → expression ";" ;         |
| printStmt       |  → "print" expression ";" ;|
| expression      | → equality ;                                             |
| equality        | → comparison ( ( "!=" \| "==" ) comparison )* ;         |
| comparison      | → term ( ( ">" \| ">=" \| "<" \| "<=" ) term )* ;        |
| term            | → factor ( ( "-" \| "+" ) factor )* ;                    |
| factor          | → unary ( ( "/" \| "\*" ) unary )* ;                      |
| unary           | → ( "!" \| "-" ) unary \| primary ;                     |
| primary         | → NUMBER \| STRING \| "true" \| "false" \| "nil"         |
|                 | \| "(" expression ")" | IDENTIFIER ;                     |

in parser.rs you'll see that we'll keep passing the tokens until primary which are the leaves of the AST, we'll go deeper whilst respecting the rulst of precedence and association.

### Example

3 * (1 + 2) - 1

__AST__
       -
      / \
     *   1
    / \
   3   +
      / \
     1   2

__AST code representation__
Expression(ExpressionStmt { expression: Binary(BinaryExpr { left: Binary(BinaryExpr { left: Literal(LiteralExpr { value: Integer(3.0) }), operator: Token { token_type: Star, lexeme: "*", literal: Some(String("")), line: 0 }, right: Grouping(GroupingExpr { expression: Binary(BinaryExpr { left: Literal(LiteralExpr { value: Integer(1.0) }), operator: Token { token_type: Plus, lexeme: "+", literal: Some(String("")), line: 0 }, right: Literal(LiteralExpr { value: Integer(2.0) }) }) }) }), operator: Token { token_type: Minus, lexeme: "-", literal: Some(String("")), line: 0 }, right: Literal(LiteralExpr { value: Integer(1.0) }) }) })



 
# EXPRESSION GRAMMAR RULES
| Production Rule | Syntax                                             | Description                                           |
|------------------|----------------------------------------------------|-------------------------------------------------------|
| expression       | `literal \| unary \| binary \| grouping`          | An expression can be a literal, unary, binary, or grouped expression. |
| literal          | `NUMBER \| STRING \| "true" \| "false" \| "nil"` | A literal can be a number, string, true, false, or nil. |
| grouping         | `("(" expression ")")`                             | A grouping is an expression enclosed in parentheses.   |
| unary            | `("-" \| "!") expression`                          | A unary operation is negation or logical NOT applied to an expression. |
| binary           | `expression operator expression`                   | A binary operation is an expression with an operator and another expression. |
| operator         | `"==" \| "!=" \| "<" \| "<=" \| ">" \| ">=" \| "+" \| "-" \| "*" \| "/"` | An operator can be equal, not equal, less than, less than or equal to, greater than, greater than or equal to, addition, subtraction, multiplication, or division. |




__Expression__ produce values
__Statements__ produce side effects. Preform actions or control the flow of a program

# STATEMENT GRAMAR RULES

| Production     | Expansion                                      |
|----------------|------------------------------------------------|
| program        | declaration* EOF ;                             |
| declaration    | varDecel | statement;                          |
| statement      | exprStmt                                       |
|                | printStmt                                      |
| exprStmt       | expression ";"                                 |
| printStmt      | "print" expression ";"                         |


# RLOX - ruts implementation of Lox
Project contains executable for Lox and AST struct generator
- cargo run --bin lox
- cargo run --bin generate_ast


# Learned java
- post increment is first using the variable and then adding to it charAt(i++), gets the char at index i and then increments the index
- pre increment means incrementing the variable before using it

# learned rust
- when you impl std::Display for a Struct or enum you can use {} for printing with cusotm formatting instead of {:?} 
- When you encounter the error messages like "self is a & reference, so the data it refers to cannot be borrowed as mutable," it means that the method is attempting to modify the state of the object, but the method signature does not allow it because it's working with an immutable reference.
  - use &T if you need to read the data
  - use &mut T if you need to modify the data
  - use T if you need to move/drop the data
- impl PartialEq for struct or enum to implement your custom equality rules. Equality comparison in Lox is different then the equality rules in rust
- Do you want your struct to own the String of reference it? In most cases you want your string to own it. Otherwise you have to start adding lifetimes in order to prevent the sturct outliving the &str ref. 
- I put a field of a struct in an enum, because it could be of type String or u32. Maybe I'll turn it into a generic later.
- tubo fish ::<u32>(), it looks like a fish, moves like a fish and parse like a parser.
-  dyn: Short for "dynamic," this keyword is used to indicate that we're working with trait objects, where the concrete type implementing the trait is known only at runtime.
- In rust modules are not mapped to the FS like f.e. Java. You can declare a module with the mod keyword and have multiple mods in the same file. In rust Sub modules must be declared within the parent module -> in our case inside the lib.rs
- For some use cases, when matching enums, match is awkward. in this case it's easier to use if let {} else {}. In _match_ you have to go over all the arms and in if let you're interested in matching only one specific case. [if let] (https://doc.rust-lang.org/book/ch06-03-if-let.html) 




## Error handling
- ? propagates the error to the caller of the function, so unwrap OR return ERR(From::from(err)). In our case the cli is the caller of main
- creating custom errors and throwing them if  `ok_or_else` && `unwrap_or_else(|| {Err(create custom error)})` fails
- not sure yet how to transform errors? Box<dyn Error> ?
- Try to create custom errors and return and transform them as needed? -> custom errors overkill?

Resources
[Custom Errors] (https://learning-rust.github.io/docs/custom-error-types/)
[Basic explenation](https://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html)
[Faster than lime: AOC 22 day 1] (https://fasterthanli.me/series/advent-of-code-2022/part-1#getting-started)
[Defining an Error type](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/reenter_question_mark.html)



