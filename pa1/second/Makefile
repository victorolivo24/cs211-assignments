all: second
	
second: second.c
	gcc second.c -o second -fsanitize=address,undefined -Wall -Werror -std=c11 -lm -g -Og
clean: second
	rm -f second
