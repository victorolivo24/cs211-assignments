#include<stdio.h>
#include<stdlib.h>

/**
  * This sample code is to help you get accustomed to basic C syntax,
  * and how to read from text files.
  **/
int main(int argc, char** argv){ //similar to String[] args it java. argc is args.length, argv is the args array itself.
  if(argc < 2) { //checks if there are 2 arguments!
    fprintf(stderr,"./first {text file}"); //prints messages to stderr. Primarily for error messages
    exit(1); //exits the program with a return code 1.
  }

  FILE* file = fopen(argv[1],"r"); //this will open a file specified by the user.

  if(file == NULL){ //if fopen fails to open the file, it returns NULL
    fprintf(stderr,"%s failed to open",argv[1]);
    exit(1); //exits your program
  }

  int x, y;

  while(fscanf(file,"%d %d",&x, &y) == 2){ //attempts to read two integers, and stores them in x and y. fscanf returns the number of items successfully read.
    printf("%d + %d = %d\n",x,y,x+y);
  }
  return 0;

}

