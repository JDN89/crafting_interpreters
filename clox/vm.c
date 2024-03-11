#include "vm.h"
#include "common.h"

// Author decided to declare global vm instead of declaring a pointer to the vm
// as to save lines of code
// Taking a VM pointer and passing it around would have been a better design
// choice, because it allows for more flexibility
// see: http://gameprogrammingpatterns.com/singleton.html

VM vm;

void initVM() {}

void freeVM() {}
