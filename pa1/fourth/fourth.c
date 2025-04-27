#include <stdio.h>
#include <stdlib.h>
#include <math.h>
void swapPivot(double** matrix, int cols, int row1, int row2); 
int main( int argc, char** argv) {
	if(argc <2){
		fprintf(stderr, "./fourth {text file}" );
		exit(1);
	}
	FILE* file = fopen(argv[1], "r");
	if(file == NULL){
		fprintf(stderr, "failed to open %s", argv[1]);
		exit(1);
	}
	int rows, cols;
	while(fscanf(file, "%d %d", &rows, &cols) == 2){
		double** matrix = (double**)malloc(rows * sizeof(double*));
		for(int i =0; i < rows; i ++){
			matrix[i] = (double*)malloc(cols * sizeof(double));
		}
		for(int r= 0; r <rows; r++){
			for(int c = 0; c <cols; c++){
				 if( fscanf(file, "%lf", &matrix[r][c]) != 1){
				 	return EXIT_FAILURE;
				 }
			}
		}

		//int pivot = 0;
		double epsilon = 1e-9;
		for(int i =0; i < rows; i ++){
			int largest = i;
				for(int j = i + 1; j < rows; j++){
					if (fabs(matrix[j][i] ) > fabs(matrix[largest][i])){
						largest = j;
		//				pivot =i;
					
					}
				}
			if(largest !=i){
				swapPivot(matrix,cols, i, largest);
				}
			if(fabs(matrix[i][i]) < epsilon){
				continue;
			}

		for(int k = i+1; k < rows; k++){
			if(fabs(matrix[k][i]) > epsilon){
				double factor = matrix[k][i]/matrix[i][i];
				for (int c = i; c < cols; c++){
					matrix[k][c] -= factor * matrix[i][c];
				}
			}
		}
	}
		for(int i = rows - 1; i >=0; i--) {
			if(fabs(matrix[i][i]) < epsilon){
				continue;
			}
			for(int j = i-1; j >=0; j--){
				if(fabs(matrix[j][i]) > epsilon){
					double coef = matrix[j][i] /matrix[i][i];
					for (int c = i; c < cols; c++){
						matrix[j][c] -= coef * matrix[i][c];
					}
				}
			}
		}
		for(int i =0; i < rows; i++){
			double piv = matrix[i][i];
			if(fabs(piv) > epsilon){
			for(int j = 0; j< cols; j++){
				matrix[i][j]/= piv;
		}
		}
	}
		for(int i = 0; i < rows; i++){
			for(int j =0; j < cols; j++){
				if(fabs(matrix[i][j]) <= 1e-10){
					matrix[i][j] = 0.0;
				}
				printf("%lf\t", matrix[i][j]);
		}
			printf("\n");
	}
		for(int i = 0; i< rows; i++){
			free(matrix[i]);
		}
		free(matrix);
	}	
	fclose(file);
	return EXIT_SUCCESS;

		
}
void swapPivot(double** matrix, int cols, int row1, int row2){
	for(int i = 0; i < cols; i++){
		double tmp = matrix[row1][i];
		matrix[row1][i] = matrix[row2][i];
		matrix[row2][i] = tmp;
	}
}
