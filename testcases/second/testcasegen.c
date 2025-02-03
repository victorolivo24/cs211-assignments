#include<stdio.h>
#include<stdlib.h>
#include<time.h>
int main(int argc, char** argv){
    if(argc < 4){
        fprintf(stderr,"%s {filename} {rows} {cols}\n",argv[0]);
    }
    srand(time(NULL));
    FILE* file = fopen(argv[1],"w");
    int rows = atoi(argv[2]);
    int cols = atoi(argv[3]);
    fprintf(file,"%d\n%d\n",cols,rows);
    for(int i = 0; i < cols; i++){
        for(int j = 0; j < rows - 1; j++){
            fprintf(file,"%d\t",rand() % 200);
        }
        fprintf(file,"%d\n",rand() % 200);
    }
}