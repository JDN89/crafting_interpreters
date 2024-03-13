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
