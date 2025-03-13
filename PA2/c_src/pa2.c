/* Includes you should need */
#include <stdio.h>
#include <math.h>
#include <string.h>
#include <stdlib.h>
#include <assert.h>

#define ROUNDUP 0
#define ROUNDDOWN 1
#define ROUNDTOEVEN 2

/**
 * Part 1.1 
 * @brief You'll be converting a Binary string into it's unsigned decimal value.
 * @param input_string the binary string you'll be converting to decimal.
 * @return the converting decimal value (an unsigned long)
 */
unsigned long binary_to_unsigned_decimal(const char *input_string)
{
    /* Your Implementation */
}

/**
 * Part 1.2
 * @brief You'll be converting a binary string into it's decimal value using the signed magnitude representation
 * @param input_string, the binary string you'll be converting to decimal
 * @return the signed decimal value (a signed long type using signed magnitude)
 */
long binary_to_signed_magnitude_decimal(const char *input_string)
{
    /* Your Implementation */
}

/**
 * Part 1.2
 * @brief You'll be converting a binary string into it's decimal value using ones complement.
 * @param input_string, the binary string you'll be converting to decimal
 * @return the signed decimal value (a signed long type using ones complement)
 */
long binary_to_ones_complement_decimal(const char *input_string)
{
    /* Your implementation */
}

/**
 * Part 1.4
 * @brief Convert a Binary string into it's decimal value using twos complement.
 * @param input_string: The string you're going to convert to decimal
 * @return the unsigned long using twos complement
 */
long binary_to_twos_complement_decimal(const char *input_string)
{
    /* Your implementation */
}

/**
 * part 2.1
 * @brief In this part, you'll be converting an unsigned decimal value into it's unsigned binary representation
 * @param input The decimal value you'll be converting to binary
 * @param numbits The the bitwidth of your converted value.
 * @return the binary string (you should be allocating the string on the heap!)
 */
char *unsigned_decimal_to_binary(unsigned long input, unsigned int numbits)
{
    /* Your implementation */
}

/**
 * part 2.1
 * @brief In this part, you'll be converting an unsigned decimal value into it's binary representation using signed magnitude
 * @param input The decimal value you'll be converting to binary
 * @param numbits The the bitwidth of your converted value.
 * @return the binary string (you should be allocating the string on the heap!)
 */
char *signed_decimal_to_signed_magnitude(long input, unsigned int numbits)
{
    /* Your implementation */
}

/**
 * part 2.1
 * @brief In this part, you'll be converting an unsigned decimal value into it's ones complement binary representation
 * @param input The decimal value you'll be converting to binary
 * @param numbits The the bitwidth of your converted value.
 * @return the binary string (you should be allocating the string on the heap!)
 */
char *signed_decimal_to_ones_complement(long input, unsigned int numbits)
{
    /* Your implementation */
}

/**
 * part 2.1
 * @brief In this part, you'll be converting an unsigned decimal value into it's two complement binary representation
 * @param input The decimal value you'll be converting to binary
 * @param numbits The the bitwidth of your converted value.
 * @return the binary string (you should be allocating the string on the heap!)
 */
char *signed_decimal_to_twos_complement(long input, unsigned int numbits)
{
    /* Your implementation*/
}

/**
 * Part 3.1
 * @brief In this part, you'll be converting an ieee754 string with a specified number of exponent and mantissa bits into it's decimal value (always 1 sign bit).
 * @param input the ieee 754 string to convert into it's decimal value.
 * @param exp The number of exponent bits in the input string.
 * @param mantissa The number of mantissa bits in the input string.
 */
double ieee754_to_decimal(char *input, int exp, int mantissa)
{
   /* Your implementation */
}


/**
 * part 3.2
 * @brief In this part, you'll be converting a decimal value, and converting it into it's ieee 754 representation given the number of exp, mantissa bits, and what rounding mode to use.
 * @param input The number you'll be converting into it's ieee 754 representation
 * @param exp_bits The number of exponent bits your representation should have.
 * @param mantissa_bits The number of mantissa bits your representation should have.
 * @param rounding_mode The rounding mode you should use: 0 to round up, 1 to round down, 2 for round to even.
 */
char *decimal_to_ieee754_binary(double input, int exp_bits, int mantissa_bits, int rounding_mode)
{
    printf("%d %d %d\n",ROUNDUP,ROUNDDOWN,ROUNDTOEVEN); //Demonstration on how to use macros
    /* Your implementation */
}