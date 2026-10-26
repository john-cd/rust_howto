#include <stdio.h>

/* ANCHOR: add */
int add(int left, int right) {
    return left + right;
}
/* ANCHOR_END: add */

/* ANCHOR: greet */
void hello(void) {
    puts("Hello from C!");
}

void greet(const char *name) {
    printf("Hello, %s!\n", name);
}
/* ANCHOR_END: greet */
