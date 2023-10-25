# learned rust

## TODO
- create LoxError struct -> throw error in the appropriate places
- place struct in Enum later on

- report error

- Part where you write java code to producre ast -> look into how to do this in rust with macros

## Error handling:

? propagates the error to the caller of the function, so unwrap OR return ERR(From::from(err)). In our case the cli is the caller of main
- ? =  unwrap or return Err(From::from(err))
- not sure yet how to transform errors? Box<dyn Error> ?
- Try to create custom errors and return and transform them as needed? -> custom errors overkill?

Resources
[Custom Errors] (https://learning-rust.github.io/docs/custom-error-types/)
[Basic explenation](https://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html)
[Faster than lime: AOC 22 day 1] (https://fasterthanli.me/series/advent-of-code-2022/part-1#getting-started)
[Defining an Error type](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/reenter_question_mark.html)


