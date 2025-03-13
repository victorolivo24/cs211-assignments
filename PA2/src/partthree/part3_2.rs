#[cfg(test)]
pub mod test{
    use std::ffi::{c_char, c_double, c_int, CStr};

    use libloading::{Library, Symbol};
    use rand::Rng;

    type Decimaltofloat = unsafe extern "C" fn(c_double, c_int, c_int, c_int) -> *const c_char;

    #[test]
    fn test_1(){
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library")};
        let dec_to_float: Symbol<Decimaltofloat> = unsafe { lib.get(b"decimal_to_ieee754_binary").expect("Failed to load symbol") };
        let result = unsafe{dec_to_float(0.21875,3,3,2)};
        let result = unsafe { CStr::from_ptr(result) };
        let result = result.to_string_lossy().into_owned();
        assert_eq!(result,"0000111","test 1 - FAILED!")
    }
    #[test]
    fn test_2(){
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library")};
        let dec_to_float: Symbol<Decimaltofloat> = unsafe { lib.get(b"decimal_to_ieee754_binary").expect("Failed to load symbol") };
        let result = unsafe{dec_to_float(-0.21875,3,3,2)};
        let result = unsafe { CStr::from_ptr(result) };
        let result = result.to_string_lossy().into_owned();
        assert_eq!(result,"1000111","test 1 - FAILED!")
    }
    #[test]
    fn test_3(){
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library")};
        let dec_to_float: Symbol<Decimaltofloat> = unsafe { lib.get(b"decimal_to_ieee754_binary").expect("Failed to load symbol") };
        let result = unsafe{dec_to_float(-27.0,4,3,0)};
        let result = unsafe { CStr::from_ptr(result) };
        let result = result.to_string_lossy().into_owned();
        assert_eq!(result,"11011110","test 1 - FAILED!");
    }

    #[test]
    fn test_4(){
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library")};
        let dec_to_float: Symbol<Decimaltofloat> = unsafe { lib.get(b"decimal_to_ieee754_binary").expect("Failed to load symbol") };
        let result = unsafe{dec_to_float(-1.203125,5,3,1)};
        let result = unsafe { CStr::from_ptr(result) };
        let result = result.to_string_lossy().into_owned();
        assert_eq!(result,"101111000","test 1 - FAILED!");
    }

    #[test]
    fn test_5() {
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library")};
        let dec_to_float: Symbol<Decimaltofloat> = unsafe { lib.get(b"decimal_to_ieee754_binary").expect("Failed to load symbol") };
        let input: f64 = rand::rng().random_range(-1.0e6..=1.0e6);
        let input_bits = input.to_bits(); // Convert to IEEE 754 representation
        let input_binary = format!("{:064b}", input_bits);
        let result = unsafe{dec_to_float(input,11,52,2)};
        let result = unsafe {CStr::from_ptr(result)};
        let result = result.to_string_lossy().into_owned();
        assert_eq!(result, input_binary, "test_5 - FAILED!");
    }
}