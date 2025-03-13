#[cfg(test)]
pub mod test{

    use libloading::{Library, Symbol};
    use std::ffi::{c_char, c_long, c_uint, CStr};
    type DecToBinConversions = unsafe extern "C" fn(c_long, c_uint) -> *mut c_char;
    #[test]
    fn test_1(){
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let signed_decimal_to_twos_complement: Symbol<DecToBinConversions> = unsafe {
            lib.get(b"signed_decimal_to_twos_complement")
                .expect("Failed to get signed_decimal_to_twos_complement")
        };

        let result = unsafe {signed_decimal_to_twos_complement(-1,9)};
        assert!(!result.is_null());
        let result = unsafe { CStr::from_ptr(result) };
        let result = result.to_string_lossy().into_owned();
        assert_eq!(result,"111111111","test 1 - FAILED!");
    }

    #[test]
    fn test_2(){
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let signed_decimal_to_twos_complement: Symbol<DecToBinConversions> = unsafe {
            lib.get(b"signed_decimal_to_twos_complement")
                .expect("Failed to get signed_decimal_to_twos_complement")
        };

        let result = unsafe {signed_decimal_to_twos_complement(15,7)};
        assert!(!result.is_null());
        let result = unsafe { CStr::from_ptr(result) };
        let result = result.to_string_lossy().into_owned();
        assert_eq!(result,"0001111","test 1 - FAILED!");
    }

    #[test]
    fn test_3(){
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let signed_decimal_to_twos_complement: Symbol<DecToBinConversions> = unsafe {
            lib.get(b"signed_decimal_to_twos_complement")
                .expect("Failed to get signed_decimal_to_twos_complement")
        };
        let result = unsafe {signed_decimal_to_twos_complement(-98342,18)};
        assert!(!result.is_null());
        let result = unsafe { CStr::from_ptr(result) };
        let result = result.to_string_lossy().into_owned();
        assert_eq!(result,"100111111111011010","test 1 - FAILED!");
                          
    }

    #[test]
    fn test_4(){
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let signed_decimal_to_twos_complement: Symbol<DecToBinConversions> = unsafe {
            lib.get(b"signed_decimal_to_twos_complement")
                .expect("Failed to get signed_decimal_to_twos_complement")
        };
        let result = unsafe {signed_decimal_to_twos_complement(-16777216,25)};
        assert!(!result.is_null());
        let result = unsafe { CStr::from_ptr(result) };
        let result = result.to_string_lossy().into_owned();
        assert_eq!(result,"1000000000000000000000000","test 1 - FAILED!");
                          
    }

    #[test]
    fn test_5(){
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let signed_decimal_to_twos_complement: Symbol<DecToBinConversions> = unsafe {
            lib.get(b"signed_decimal_to_twos_complement")
                .expect("Failed to get signed_decimal_to_twos_complement")
        };
        let input_num: i64 = rand::random_range(i64::MIN..=i64::MAX) as i64;
        let expected = format!("{:#b}", input_num);
        let expected = &expected[2..];
        let expected = if input_num < 0 {
            format!("{:1>64}",expected)
        }
        else{
            format!("{:0>64}",expected)
        };
        let result = unsafe { signed_decimal_to_twos_complement(input_num, 64) };
        assert!(!result.is_null());
        let result = unsafe { CStr::from_ptr(result) };
        let result = result.to_string_lossy().into_owned();
        
        assert_eq!(result, expected, "test 5 - FAILED! input num: {}", input_num);
    }
    

}