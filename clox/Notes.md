# Notes

<!-- TODO:  interpret function vm.c -->


[Compiler Explorer] (https://godbolt.org) 


As I understand it now is that we store chunks of executions in an array (compcat, O(1) index lookup, O(1) insertions at the end).

## General overview

In The compiler we scan the source code, tokenize and emit byte code. Our compiler als contains the parser struct where we store the current operator and the previous operator for PRATT parsing -> comparing precedence levels of the operators (prefix and infix) (see explanation chapter 17). We have a single pass compiler: the compiler processes the source code in a linear manner, from start to finish, without the need for multiple passes. This approach can simplify the compiler's implementation and make it more efficient in terms of memory usage and processing time.
We store the information in a chunk Struct, which contains a OP_CODE array, where we store the OP instructions and index values of the correlating constant values and an ValueArray where we store the constant values. This struct acts as an intermediary between the compiler and the virtual machine.
We pass the chunk to the Virtual machine where we iterate continiuosly over the OP_CODE array, executing instructions sequentially and passing values from the Chunk constant array to the VM stack according to the instructions we have to execute. It maintains a constant stack for storing and manipulating values as instructed by the bytecode. 


## Project setup

<!-- TODO: -->
Reread chapter 14. I had forgotten that in chunk.code we store the operation code as well as the index of the constant we encounter
The constant value we store in the chunk.Value array -> we can look it up using the index stored in code 
so when we encounter OP_CONSTANT in code we know that the next value will be the index which we retrieve using chunk.ip (which is a pointer pointing to code)

__Scanner__: The __Token__ doesn't contain the raw lexeme. It just points to the start of the source code and contains the length. See notes chapter 16

__Compiler__: A __parser__ produces an AST—just like jlox does—and then a __code generator__ traverses the AST and outputs target code

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

## Chapter 16: Scanning

In the context of the Clox compiler, there are a few key differences in how tokens are managed compared to Jlox:

Error Handling:

In Jlox, the scanner itself reports errors like unterminated strings or unrecognized characters.
In Clox, the scanner produces a synthetic "error" token for such errors and passes it to the compiler. This allows the compiler to initiate error recovery before reporting the error.
Representation of Lexemes:

In Jlox, each token stores its lexeme as a separate string object.
In Clox, tokens store references to the original source string. Each token consists of a pointer to the first character of its lexeme and the length of the lexeme. This eliminates the need to manage memory for lexemes separately and simplifies token handling. As long as the main source code string outlives all tokens, this approach works fine.


## Chapter 17 : Compiling expressions


To address the issue described, where the unary() function mistakenly __consumes more tokens__ than intended due to calling expression() which parses any expression regardless of precedence, you need to revise the parsing strategy to consider operator precedence.

One common approach to handle operator precedence in parsing expressions is to use a technique called __precedence climbing or Pratt parsing__. In this approach, each operator has a precedence level, and parsing functions are organized in a way that respects these precedence levels.

### PRAT and recursion

#### Recursion
A recursive function is a function that call's itself. Each function call goes on the __call stack__. Here each function exists with each own environment and variables. We keep call the function until we reach the basecase. Then we start to unwind the stack poping the calls of the stack from most recent to oldest call - __LIFO__.

In our compiler we keep calling __parsePrecendence__ until we encounter an operator with a lower precendence. Each recursive call we add at minimum +1 to the precedence leve, because binary operations are left asociative. Meaning 1+2+3+4 -> ((1+2)+3)+4. By adding +1, We make sure that we parse (1+2) add it to the chunk array as OP_CONSTANT OP_CONSTANT and OP_ADD. 
We compare two operator types with eachoter. Over each number we advance and emit the value to the value stack, the prvious operator is stored in Parser.previous and the currrent operator gets stored in parser.current. We compare the precendence of these two operators and if the precedence is lower we skip the while loop, emit the operator and unwind the stack until the base case PREC_ASSIGNMENT. Here we encounter a number again, emit the bytecode, compare the current token with the next token in the while loop, whilst advancing over the number and pushing them on the value stack.

[recursion explenation](https://www.freecodecamp.org/news/how-recursion-works-explained-with-flowcharts-and-a-video-de61f40cb7f9/)


#### Recognizing identifiers via tries and state machines
Very interesting part with some good links:
[syntax diagrams](https://en.wikipedia.org/wiki/Syntax_diagram)
[Trie](https://en.wikipedia.org/wiki/Trie)
[Deterministic finite automaton](https://en.wikipedia.org/wiki/Deterministic_finite_automaton)
[State design pattern](https://gameprogrammingpatterns.com/state.html)
