# Notes

<!-- TODO:  interpret function vm.c -->


[Compiler Explorer] (https://godbolt.org) 


As I understand it now is that we store chunks of executions in an array (compcat, O(1) index lookup, O(1) insertions at the end).

## Project setup

<!-- TODO: -->
Reread chapter 14. I had forgotten that in chunk.code we store the operation code as well as the index of the constant we encounter
The constant value we store in the chunk.Value array -> we can look it up using the index stored in code 
so when we encounter OP_CONSTANT in code we know that the next value will be the index which we retrieve using chunk.ip (which is a pointer pointing to code)


## 14 Chuncks of Bytecode

### Why is walking the AST slow
- Each piece of syntax is a differnt node/ class in the AST. All these classes have pointers between them. The classes are probalby spread out in memory causing __Spacial locality__. The CPU can process data faster than it can pull it from RAM. To compensate the CPU's cache data and do this via pull in extra adjecent data in cache. Meaning if your data is all stored close to eachother tey can be processed faster (up to 100x).

### 
__Bytecode__ : one byte operation code represents instructions (add, subtract,...).


### What will we use isntead of __AST__ ?

#### __stack-based__ bytecode instructions and why not __register-based__?
There’s only a single instruction to decode and dispatch, and the whole thing fits in four bytes. Decoding is more complex because of the additional operands, but it’s still a net win. There’s no pushing and popping or other stack manipulation.

The main implementation of Lua used to be stack-based. For Lua 5.0, the implementers switched to a register instruction set and noted a speed improvement. The amount of improvement, naturally, depends heavily on the details of the language semantics, specific instruction set, and compiler sophistication, but that should get your attention.

That raises the obvious question of why I’m going to spend the rest of the book doing a stack-based bytecode. Register VMs are neat, but they are quite a bit harder to write a compiler for. For what is likely to be your very first compiler, I wanted to stick with an instruction set that’s easy to generate and easy to execute. Stack-based bytecode is marvelously simple.

It’s also much better known in the literature and the community. Even though you may eventually move to something more advanced, it’s a good common ground to share with the rest of your language hacker peers.

The Lua dev team—Roberto Ierusalimschy, Waldemar Celes, and Luiz Henrique de Figueiredo—wrote a fantastic paper on this, one of my all time favorite computer science papers, “The Implementation of Lua 5.0” (PDF).
[Lua 5 -> from stackbased to register-based](https://www.lua.org/doc/jucs05.pdf)

## Compiler

In the context of the Clox compiler, there are a few key differences in how tokens are managed compared to Jlox:

Error Handling:

In Jlox, the scanner itself reports errors like unterminated strings or unrecognized characters.
In Clox, the scanner produces a synthetic "error" token for such errors and passes it to the compiler. This allows the compiler to initiate error recovery before reporting the error.
Representation of Lexemes:

In Jlox, each token stores its lexeme as a separate string object.
In Clox, tokens store references to the original source string. Each token consists of a pointer to the first character of its lexeme and the length of the lexeme. This eliminates the need to manage memory for lexemes separately and simplifies token handling. As long as the main source code string outlives all tokens, this approach works fine.
