#include <stdio.h>
#include <stdlib.h>
#include <math.h>
int main(int argc, char** argv){
	int sum = 0;
	int x = atoi(argv[1]);
	for (int i = 1; i <x; i++) {
		if (x % i == 0){
			sum+= i;
		}
	}
	if (sum !=x){
		printf("%d\n", -1);
		return EXIT_SUCCESS;
	}
	for (int p = 2; p < 100; p++){
		int mp = ((int)pow(2, p -1) * ((int)pow(2,p) - 1));
		if(mp == x){
			printf("%d\n", (int)pow(2, p) - 1);
			return EXIT_SUCCESS;
		}
	}
	return EXIT_SUCCESS;

}
