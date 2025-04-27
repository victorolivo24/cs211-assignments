#include <stdio.h>
#include <stdlib.h>

int** createMatrix(int len){
	int** matrix = (int**)malloc(sizeof(int*) * len);
	for(int i = 0; i < len; i++){
		matrix[i] = (int*)malloc(sizeof(int) * len);
	}
	return matrix;
}
void freeMatrix(int **matrix, int len){
	for(int i = 0; i < len; i++){
		free(matrix[i]);
	}
	free(matrix);
}
// method for two by two
long long det(int **matrix, int len){
	if(len ==1){
		return(long long)matrix[0][0];
	}
	if(len == 2){
		return ((long long)matrix[0][0] * matrix[1][1]) - ( (long long) matrix[1][0] * matrix[0][1] );
	}
	long long determ = 0;
	int** smallerMatrix = createMatrix(len-1);
	for(int curCol = 0; curCol < len; curCol++){
		int smalRow = 0;

		for(int i =1; i<len; i++){
			int smalCol = 0;
			for(int j = 0; j <len; j++){
				if( j != curCol){
					smallerMatrix[smalRow][smalCol] = matrix[i][j];
					smalCol++;
				}
			}
		smalRow++;
		}
		long long piv = matrix[0][curCol];
		if(curCol % 2 != 0){
			piv*=-1;
		}	
		determ += piv * det(smallerMatrix, len-1);
	}
	freeMatrix(smallerMatrix, len -1);
	return determ;

}	
int main (int argc, char** argv){
	if(argc < 2){
		fprintf(stderr, "./fifth {text file}");
		exit(1);
	}
	FILE* file = fopen(argv[1], "r");
	if(file == NULL){
		fprintf(stderr,"%s failed to open", argv[1]);
		exit(1);
	}
	int rows;
	while(fscanf(file, "%d", &rows) == 1){
		int** matrix = createMatrix(rows);
		for(int i = 0; i < rows; i++){
			for(int j = 0; j < rows; j++){
				if(fscanf(file, "%d", &matrix[i][j]) != 1){
					exit(1);
				}
		}
	}
	long long determinant = det(matrix, rows);
	freeMatrix(matrix, rows);
	printf("%lld\n", determinant);
	}
	fclose(file);
	return EXIT_SUCCESS;
}
