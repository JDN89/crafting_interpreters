# Notes

[Compiler Explorer] (https://godbolt.org) 


As I understand it now is that we store chunks of executions in an array (compcat, O(1) index lookup, O(1) insertions at the end).

## 14 Chuncks of Bytecode

### Why is walking the AST slow
- Each piece of syntax is a differnt node/ class in the AST. All these classes have pointers between them. The classes are probalby spread out in memory causing __Spacial locality__. The CPU can process data faster than it can pull it from RAM. To compensate the CPU's cache data and do this via pull in extra adjecent data in cache. Meaning if your data is all stored close to eachother tey can be processed faster (up to 100x).

### 
__Bytecode__ : one byte operation code represents instructions (add, subtract,...).


### What will we use isntead of __AST__ ?
