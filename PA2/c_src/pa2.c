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
	int numBits = strlen(input_string);
	unsigned long sum  = 0;
	for (int i = 0; i  < numBits; i++){
		char dig = input_string[i];
		if (dig == '1'){
			sum += (unsigned long)pow(2.0, numBits - 1 - i);
		}
	}
	return sum;
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
	
        int numBits = strlen(input_string);
        long sum  = 0;
        for (int i = 1; i  < numBits; i++){
                char dig = input_string[i];
                if (dig == '1'){
                        sum += (1L << (numBits - 1 - i));
                }
        }
	if(input_string[0] == '1'){
		sum = -sum;
	}
        return sum;
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
	char *dup_string = strdup(input_string);
	int numBits = strlen(input_string);
	int negZero = 1;


	if (dup_string[0] == '1'){
			
		for(int i = 0; i < numBits; i++){
			if(dup_string[i] == '1'){
				dup_string[i] = '0';
				
			} else{
				negZero = 0;
				dup_string[i] = '1';
			}	
		}
	} else {
		negZero = 0;
	}
	
	long res =  binary_to_unsigned_decimal(dup_string);
	if(input_string[0] == '1'){
		res = -res;
	}
	
	if(negZero){
		res = 0;
	}
	
	free(dup_string);
	return res;
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
	if(input_string[0] == '0'){
		return binary_to_unsigned_decimal(input_string);
	}

	long ones = binary_to_ones_complement_decimal(input_string);
	return ones - 1;

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
	char *res = (char *)malloc(sizeof(char) * (numbits + 1));
    	for (int i = 0; i < numbits; i++) {
 	   res[numbits - 1 - i] = (input & (1UL << i)) ? '1' : '0';
	}
   	res[numbits] = '\0';
    	return res;
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
	char *res = (char *)malloc(sizeof(char) * (numbits + 1));
    if (!res) return NULL;  // Handle malloc failure

    // Handle the smallest negative number (e.g., -128 for 8 bits)
    if (input == -(1L << (numbits - 1))) {
        res[0] = '1';
        for (int i = 1; i < numbits; i++) {
            res[i] = '0';
        }
        res[numbits] = '\0';
        return res;
    }

    // Handle other cases
    unsigned long magnitude = (input < 0) ? -input : input;
    char *mag = unsigned_decimal_to_binary(magnitude, numbits - 1);
    if (!mag) {
        free(res);
        return NULL;  // Handle malloc failure in unsigned_decimal_to_binary
    }

    res[0] = (input >= 0) ? '0' : '1';
    memcpy(res + 1, mag, numbits - 1);
    res[numbits] = '\0';

    free(mag);
		

	return res;
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
	
	
	char* res = signed_decimal_to_signed_magnitude(input, numbits);
	if(input < 0){
		for(unsigned int i = 1; i < numbits; i++){
                        if(res[i] == '1'){
                                res[i] = '0';
                        } else{
                           
                                res[i] = '1';
                        }
                }
	}
	res[numbits] = '\0';
	return res;
	
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
	
	char* res = signed_decimal_to_ones_complement(input, numbits);
	if(!res){
		return NULL;
	}
	
	unsigned long absVal = (input < 0) ? (~(-input) + 1) : input;

	for(int i = numbits - 1; i >= 0; i--){
		res[i] = (absVal & 1) + '0';
		absVal >>= 1;
	}
	res[numbits] = '\0';
	return res;
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
   
	char* exponent = malloc(sizeof(char) * exp + 1);
	for (int i = 0; i < exp; i++){
		exponent[i] = input[1 + i];
	}
	exponent[exp] = '\0';

	char* decimal = malloc(sizeof(char) * mantissa + 1);
	
	for(int i = 0; i < mantissa; i++){
		decimal[i] = input[i + exp + 1];
	}
	decimal[mantissa] = '\0';
	unsigned long decExponent = binary_to_unsigned_decimal(exponent);
	free(exponent);

	int bias = pow(2, exp - 1) - 1;
	int sign = (input[0] == '1') ? -1 : 1;
	int allExponentBits = 1;
	for(int i = 0; i < exp; i++) {
		if (input[i + 1] != '1'){
			allExponentBits = 0;
			break;
		}
	}

	int allExponentZero = 1;
	for(int i = 0; i < exp; i++){
		if(input[i + 1] != '0'){
			allExponentZero = 0;
			break;
		}
	}

	int mantissaZero = 1;
	for(int i = 0; i < mantissa; i++){
		if(decimal[i] != '0'){
			mantissaZero = 0;
			break;
		}
	}
	
	if(allExponentBits == 1){
		if(mantissaZero == 1){
			free(decimal);
			return sign * INFINITY;
		} else {
			free(decimal);
			return NAN;
		}
	}

	double mantissaAsDecimal = 0.0;

	for(int i = 0; i < mantissa; i++){
		if(decimal[i] == '1'){
			mantissaAsDecimal += pow(2, -(i + 1));
		}
	}
	free(decimal);
	
	if (allExponentZero){
		return sign * mantissaAsDecimal * pow(2, 1 - bias);
	}

	//add code to deal with not nromalized cases 
	
	return sign * (1.0 + mantissaAsDecimal) * pow(2, (int)decExponent-bias);

}


/**
 * part 3.2
 * @brief In this part, you'll be converting a decimal value, and converting it into it's ieee 754 representation given the number of exp, mantissa bits, and what rounding mode to use.
 * @param input The number you'll be converting into it's ieee 754 representation
 * @param exp_bits The number of exponent bits your representation should have.
 * @param mantissa_bits The number of mantissa bits your representation should have.
 * @param rounding_mode The rounding mode you should use: 0 to round up, 1 to round down, 2 for round to even.
 */// Helper: Convert a value into a rounded integer based on mode
unsigned long round_fraction(double X, int rounding_mode) {
    double lower = floor(X);
    double frac = X - lower;

    if (rounding_mode == ROUNDUP) {
        return (frac > 0.0) ? (unsigned long)(lower + 1.0) : (unsigned long)lower;
    } else if (rounding_mode == ROUNDDOWN) {
        return (unsigned long)lower;
    } else {
        // round to nearest even
        if (frac > 0.5) return (unsigned long)(lower + 1.0);
        else if (frac < 0.5) return (unsigned long)lower;
        else return ((fmod(lower, 2.0) == 0.0) ? (unsigned long)lower : (unsigned long)(lower + 1.0));
    }
}

// Helper: Fill exponent and mantissa bits into result
void fill_result_bits(char *result, int exp_bits, int mantissa_bits, int resExp, unsigned long mantissaVal) {
    // Exponent
    for (int i = exp_bits - 1; i >= 0; i--) {
        result[1 + i] = ((resExp >> (exp_bits - 1 - i)) & 1) ? '1' : '0';
    }

    // Mantissa
    for (int i = 0; i < mantissa_bits; i++) {
        int shift = mantissa_bits - 1 - i;
        result[1 + exp_bits + i] = ((mantissaVal >> shift) & 1) ? '1' : '0';
    }
}

char *decimal_to_ieee754_binary(double input, int exp_bits, int mantissa_bits, int rounding_mode) {
    int total_bits = 1 + exp_bits + mantissa_bits;

    
    if (total_bits == 64) {
        char *result = malloc(65);
        if (!result) return NULL;
        union { double d; unsigned long long bits; } u;
        u.d = input;
        for (int i = 63; i >= 0; i--) {
            result[63 - i] = ((u.bits >> i) & 1ULL) ? '1' : '0';
        }
        result[64] = '\0';
        return result;
    }

    char *result = malloc(total_bits + 1);
    if (!result) return NULL;
    result[total_bits] = '\0';


    result[0] = (input < 0) ? '1' : '0';
    double absVal = fabs(input);

    // Zero
    if (absVal == 0.0) {
        for (int i = 1; i < total_bits; i++) result[i] = '0';
        return result;
    }

    int bias = (1 << (exp_bits - 1)) - 1;
    double minNormal = pow(2, 1 - bias);
    int isDenormal = (absVal < minNormal);

    unsigned long mantissaVal = 0;
    int resExp = 0;

    if (!isDenormal) {
        // Normalized
        int e = (int)floor(log2(absVal));
        double significand = absVal / pow(2, e);
        double frac = significand - 1.0;
        double scaled = frac * pow(2, mantissa_bits);

        mantissaVal = round_fraction(scaled, rounding_mode);

        unsigned long maxMantissa = (1UL << mantissa_bits);
        if (mantissaVal == maxMantissa) {
            mantissaVal = 0;
            e += 1;
        }

        resExp = e + bias;
        if (resExp >= (1 << exp_bits) - 1) {
            resExp = (1 << exp_bits) - 1;
            mantissaVal = 0;
        }
    } else {
        // Denormalized
        resExp = 0;
        double scaled = absVal / pow(2, 1 - bias) * pow(2, mantissa_bits);
        mantissaVal = round_fraction(scaled, rounding_mode);

        unsigned long maxMantissa = (1UL << mantissa_bits);
        if (mantissaVal >= maxMantissa) {
            mantissaVal = maxMantissa - 1;
        }
    }

    fill_result_bits(result, exp_bits, mantissa_bits, resExp, mantissaVal);
    return result;
}

