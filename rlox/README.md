## TODO
- Read chapter 15 and 16 of rust book: RefCell and Arc,...
- format BinExpr,... so you can print a nice AST and paste it in the readme
- test al scenarios: does the interpreter work 
- see if you can replace match in some places with __if let__, better to use in combination with enums where I'm only interested in one specific enum field
- check equality, behaviour is weird? >5=6=7 >Ok("5")
- after finishing up the parser,write a test for parser and scanner and see if you can speed it up through removing the clonse and passing the reference. Check AOC 2022 for axmple of optimizing and measuring speed (I put some explenation there in the past).
- remove #[allow(dead_code, unused_variables)]
-   sfd


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



