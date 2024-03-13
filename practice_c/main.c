#include <stdio.h>

int main() {
  int a[] = {1, 2, 3};
  int len = sizeof(a) / sizeof(int);
  int *x = a;
  int i = 0;
  for (i = 0; i < len; i++) {
    // we dereference *x to get the underlying value it points to!!
    printf("Address of subscript %d = %d Value = %d\n", i, x, *x);
    x++;
  }
}
