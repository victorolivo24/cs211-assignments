
#include <stdio.h>
#include <stdlib.h>
int main(int argc, char** argv){
	if(argc < 2){
		fprintf(stderr, "%s", argv[0]);
    		exit(1);
  	}

	FILE* fptr = fopen(argv[1],"r");
	if(fptr == NULL){
		printf("failed empty");
		return EXIT_FAILURE;
	}

	int rows;
	int cols;
	if(fscanf(fptr, "%d %d", &rows,&cols) != 2)
	{
		printf("failed");
		exit(1);
	}
		

	int **matrix = (int**)malloc(rows * sizeof(int*));
	
	for (int i = 0; i < rows; i++){
		matrix[i] = (int*)malloc(rows * sizeof(int));
	}
	for(int i = 0; i <rows; i++){
		for(int j = 0; j < rows; j++){
			if(fscanf(fptr, "%d", &matrix[i][j]) != 1){
				exit(1);
				printf("failed");
			}
		}
	}
	//maybe need to free allocated memory
	fclose(fptr);
	for(int i = 0; i < rows; i++){
		for (int j = i + 1; j < rows; j++){
			int temp = matrix[i][j];
			matrix[i][j] = matrix[j][i];
			matrix[j][i] = temp;
		}
	}

	for(int i = 0; i < rows; i++){
		int start = 0;
		int last = rows -1;
		while (start < last) {
			int temp =matrix[i][start];
			matrix[i][start] = matrix[i][last];
			start++;
			matrix[i][last] = temp;
			last--;
		}
	}
	for(int i = 0; i < rows; i++){
		for(int j = 0; j < rows; j++){
			printf("%d\t", matrix[i][j]);
		}
		printf("\n");
	}
	for (int i = 0; i < rows; i++) {
        	free(matrix[i]);  // Free each row
    	}
    	free(matrix);
	return EXIT_SUCCESS;
}

