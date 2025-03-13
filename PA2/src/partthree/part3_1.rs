#[cfg(test)]
pub mod test{
    use std::ffi::{c_char, c_double, c_int, CString};

    use libloading::{Library, Symbol};
    use rand::Rng;

    type FloatToDec = unsafe extern "C" fn(*const c_char,c_int,c_int) -> c_double;

    #[test]
    fn test_1(){
        let lib = unsafe { Library::new("c_src/pa2.so").expect("unable to open c_src/pa2.c") }; 
        let float_to_dec:  Symbol<FloatToDec> = unsafe {lib.get(b"ieee754_to_decimal").expect("Failed to find ieee754_to_decimal function")};
        let string = CString::new("111111001").expect("Failed to convert string to C String!");
        let result = unsafe{float_to_dec(string.as_ptr(),5,3)};
        assert!(result.is_nan());
    }

    #[test]
    fn test_2(){
        let lib = unsafe { Library::new("c_src/pa2.so").expect("unable to open c_src/pa2.c") }; 
        let float_to_dec:  Symbol<FloatToDec> = unsafe {lib.get(b"ieee754_to_decimal").expect("Failed to find ieee754_to_decimal function")};
        let string = CString::new("000000111").expect("Failed to convert string to C String!");
        let result = unsafe{float_to_dec(string.as_ptr(),5,3)};
        
        let exp:f64 = 2.0;
        let expected: f64 = 0.875 * exp.powf(-14.0);
        assert_eq!(result,expected,"test_2 - FAILED!");
    }

    #[test]
    fn test_3(){
        let lib = unsafe { Library::new("c_src/pa2.so").expect("unable to open c_src/pa2.c") }; 
        let float_to_dec:  Symbol<FloatToDec> = unsafe {lib.get(b"ieee754_to_decimal").expect("Failed to find ieee754_to_decimal function")};
        let string = CString::new("1100101").expect("Failed to convert string to C String!");
        let result = unsafe{float_to_dec(string.as_ptr(),3,3)};
        let expected: f64 = -3.25;
        assert_eq!(result,expected,"test_3 - FAILED!");
    }

    #[test]
    fn test_4(){
        let lib = unsafe { Library::new("c_src/pa2.so").expect("unable to open c_src/pa2.c") }; 
        let float_to_dec:  Symbol<FloatToDec> = unsafe {lib.get(b"ieee754_to_decimal").expect("Failed to find ieee754_to_decimal function")};
        let string = CString::new("11001001000100010001000010010000").expect("Failed to convert string to C String!");
        let result = unsafe{float_to_dec(string.as_ptr(),8,23)};
        let expected: f64 = -594185.0;
        assert_eq!(result,expected,"test_3 - FAILED!");
    }

    #[test]
    fn test_5() {
        let lib = unsafe { Library::new("c_src/pa2.so").expect("unable to open c_src/pa2.c") }; 
        let float_to_dec: Symbol<FloatToDec> = unsafe { lib.get(b"ieee754_to_decimal").expect("Failed to find ieee754_to_decimal function") };
    
        let input: f64 = rand::rng().random_range(-1.0e6..=1.0e6);
        let input_bits = input.to_bits(); // Convert to IEEE 754 representation
        let input_binary = format!("{:064b}", input_bits);
    
        let string = CString::new(input_binary).expect("Failed to convert string to C String!");
        let result = unsafe { float_to_dec(string.as_ptr(), 11, 52) };
    
        let expected: f64 = input; // Make sure this matches what you expect
        assert_eq!(result, expected, "test_5 - FAILED!");
    }
    
}