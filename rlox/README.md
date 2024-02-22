
## Review

- Get a better grasps of how precendence rules are defined in the parser and get translated into an AST


## TODO

- in user interface:         if let Err(e) = run(&buf, &mut interpreter) { => probably not all the errors need to interupt the program and report an error?
- set up CI/CD that runst integration tests when you push to github
- fix fun sayHi error -> capital H is the issue
- sep through function flow to really understand
- ignore native functions for now aka clock, and just continue implementing the normal funciton flow
  - review that chapter later
- cleanup runtime error message and impl fn
- create builder patterns for structs -> see luke for inspiration
- profile interpreter once it's finished

- when interpreter part is finished -> rewrite the parser:
  - https://www.journal.stuffwithstuff.com/2011/03/19/pratt-parsers-expression-parsing-made-easy/
  - parser talk jonathan blow around minute 55. recursive descent when prescedence is increasing and logical parse single binary with while loop when precedence is decreasing
- check equality, behaviour is weird? >5=6=7 >Ok("5")



### Example

3 * (1 + 2) - 1


Lexeme: 3 - Literal: Integer: 3
Lexeme: *
Lexeme: 1 - Literal: Integer: 1
Lexeme: 2 - Literal: Integer: 2
Lexeme: )
Lexeme: -
Lexeme: 1 - Literal: Integer: 1
Lexeme: ;

__AST__
       -
      / \
     *   1
    / \
   3   +
      / \
     1   2

Expression:
  Binary : ( left:
    Binary : ( left:
      Literal:  (Integer: 3)
      operator:  *
      right :  Grouping:  (
        Binary : ( left:
          Literal:  (Integer: 1)
          operator:  +
          right :   Literal:  (Integer: 2)
        )
      )
    )
    operator:  -
    right :   Literal:  (Integer: 1)
  )

# Language titbits

#### Imperative vs Declarative:
- It’s possible to create a language that has variables but does not let you reassign—or mutate—them. Haskell is one example. SML supports only mutable references and arrays—variables cannot be reassigned. Rust steers you away from mutation by requiring a mut modifier to enable assignment. Lox is not so austere. Lox is an __imperative__ language, and mutation comes with the territory. 

#### Assignment: expression vs statement

- That little __=__ syntax is more complex than it might seem. Like most C-derived languages, __assignment is an expression__ and not a statement. You can embed assignments withing larger expressions. Whereas with an expression you can't embed the assignment withing a larger expression because it doesn't hold a value, but causes a side effect.

JAVA
```
int x = 5;
int y = (x = 10) + 2;
```
PYTHON
```
result = (x = 10) + 5  # This line will result in a syntax error
print(result)
```

Implications:
__Precedence and Association__: In C-derived languages, the assignment operator has lower precedence than most other operators, which means it is often the "weakest link" in an expression, evaluating last.


The distinction between __l-values__ and __r-values__ is fundamental to understanding how values are assigned in programming languages. Let's break down the concepts:

l-value: An l-value represents a storage location in memory. It refers to the left-hand side of an assignment operation, where a value can be stored. An l-value is something that can appear on the left side of an assignment statement. In your example, a is an l-value because it represents a storage location where a value can be assigned.

r-value: An r-value represents the content stored in a memory location. It refers to the right-hand side of an assignment operation, where a value is retrieved. An r-value is something that produces a value. In your example, the string "value" is an r-value because it produces a value.

In the code you provided:

javascript
```
var a = "before";
a = "value";
```
In the first line, a is an l-value because it's on the left side of the = sign, indicating that we are storing the value "before" in the memory location represented by a.

In the second line, a is also an l-value, but "value" is an r-value because it produces the value that we want to store in the memory location represented by a.

Now, regarding the syntax tree and parsing:

When parsing an assignment expression, the parser encounters the left-hand side (l-value) and doesn't know it's an l-value until it sees the = sign. This is because the same identifier (a in this case) could be an l-value or an r-value in different contexts. To handle this, the parser usually creates an abstract syntax tree (AST) node for the assignment expression (Expr.Assign in your example), and the details of whether a particular expression is an l-value or r-value are determined during later stages of the compilation process.


# RLOX - ruts implementation of Lox
Project contains executable for Lox and AST struct generator
- cargo run --bin lox
- cargo run --bin generate_ast


# Learned java
- post increment is first using the variable and then adding to it charAt(i++), gets the char at index i and then increments the index
- pre increment means incrementing the variable before using it

# learned rust
  - RefCell -> Couldn't use __&mut self__ in visitor pattern -> to much code ahd to be rewritten, might rewrite it in a later version on a different branch. I had to use RefCell to mutate the values of data that was behind an immutable reference. I used RefCell in combination with Rc -> because we had multiple owners of mutable data and without Rc with got already borrowed: BorrowMutError.
__Interior mutability__ in Rust refers to the ability to mutate data even when it is behind an immutable reference. In Rust, by default, you can't mutate data through an immutable reference (__&T__). However, there are situations where you might need to mutate data even when you only have an immutable reference to it. This is where interior mutability comes into play. The primary tool for achieving interior mutability is the __Cell__ (for types that implement Copy: Primitves) and __RefCell__ (for types that don't implement copy) types in the standard library. When you see __&mut self__ in Rust, it typically indicates a mutable borrow of self within a method. This is a __compile-time mechanism__ and is different from interior mutability.

Interior mutability is more relevant when you need to mutate data through a shared reference (&T). For example, in situations where you have a shared reference to data that needs to be mutated, but you don't want to pass around mutable references or __transfer ownership__. This pattern is useful in scenarios like implementing smart pointers, caches, or other scenarios where shared mutable access is required. The use of RefCell enables you to bypass Rust's borrow checker rules for a specific scenario.

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



