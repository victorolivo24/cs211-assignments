# Programming Assignment 1

This is the first programming assignment for CS211. It aims to get you accustomed to the syntax of the C programming language.

## Cloning this repository

To clone this repository, you can run this command on the ilab machine:

```bash
git clone https://github.com/AFE123x/sp25-cs211-assignments.git
```

This will create a folder named ```sp25-cs211-assignments```, containing the autograder, test cases, and a sample program for reference.

## pa1 folder

- You are not given template code. Instead, you will make the pa1 folder in the ```sp25-cs211-assignment``` folder
## Errata

February 4th, 2025 @ 14:16

- The test cases neglected to address potential differences in the calculation. I've changed the output test cases to remove the -0.000000. 
- In other words, if you have -0.000000, you should print 0.000000.

February 12th, 2025 @ 16:03

- I added the input and output for the first part. This is **solely for reference; you should not read from a text file for part 1**.
- I also updated the autograder to specify, for part 1, what input failed
