
# RLOX - ruts implementation of Lox
Project contains executable for Lox and AST struct generator
- cargo run --bin lox
- cargo run --bin generate_ast

## TODO
- place breakpoints and go in detail through the code
- make Literal in Token an option;
- place ast printer in a unit test?

    pub literal: Literal,

# Learned java
- post increment is first using the variable and then adding to it charAt(i++), gets the char at index i and then increments the index
- pre increment means incrementing the variable before using it

# learned rust
- Do you want your struct to own the String of reference it? In most cases you want your string to own it. Otherwise you have to start adding lifetimes in order to prevent the sturct outliving the &str ref. 
- I put a field of a struct in an enum, because it could be of type String or u32. Maybe I'll turn it into a generic later.
- tubo fish ::<u32>(), it looks like a fish, moves like a fish and parse like a parser.
-  dyn: Short for "dynamic," this keyword is used to indicate that we're working with trait objects, where the concrete type implementing the trait is known only at runtime.

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



