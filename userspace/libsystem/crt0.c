#include <unistd.h>

extern int main(void);

void _start(void) {
	int ret = main();
	_exit(ret);
}
