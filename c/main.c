#include <stdio.h>

static void fun(void);

int main(void) {
	fun();
	return 1;
}


static void fun(void) {
	printf("hai");
}
