#include<stdio.h>
#include<stdlib.h>
int main(int argc, char** argv){
  if(argc < 2) {
    fprintf(stderr,"./first {text file}");
    exit(1);
  }

  FILE* file = fopen(argv[1],"r");

  if(file == NULL){ //if fopen fails to open the file, it returns NULL
    fprintf(stderr,"%s failed to open",argv[1]);
    exit(1); //exits your program
  }

  int x, y;

  while(fscanf(file,"%d %d",&x, &y) == 2){
    printf("%d + %d = %d\n",x,y,x+y);
  }
  return 0;

}

