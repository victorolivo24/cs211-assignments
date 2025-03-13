#[cfg(test)]
pub mod test {

    use libloading::{Library, Symbol};
    use std::ffi::{c_char, c_long, c_uint, CStr};
    type DecToBinConversions = unsafe extern "C" fn(c_long, c_uint) -> *mut c_char;

    #[test]
    fn test_1() {
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let signed_decimal_to_ones_complement: Symbol<DecToBinConversions> = unsafe {
            lib.get(b"signed_decimal_to_ones_complement")
                .expect("Failed to get signed_decimal_to_ones_complement")
        };

        let result = unsafe { signed_decimal_to_ones_complement(15, 7) };
        assert!(!result.is_null(), "return value is NULL");
        let result = unsafe { CStr::from_ptr(result) };
        let result = result.to_string_lossy().into_owned();
        assert_eq!(result, "0001111", "test_1 - FAILED");
    }

    #[test]
    fn test_2() {
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let signed_decimal_to_ones_complement: Symbol<DecToBinConversions> = unsafe {
            lib.get(b"signed_decimal_to_ones_complement")
                .expect("Failed to get signed_decimal_to_ones_complement")
        };

        let result = unsafe { signed_decimal_to_ones_complement(-53, 10) };
        assert!(!result.is_null(), "return value is NULL");
        let result = unsafe { CStr::from_ptr(result) };
        let result = result.to_string_lossy().into_owned();
        assert_eq!(result, "1111001010", "test_2 - FAILED");
    }

    #[test]
    fn test_3() {
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let signed_decimal_to_ones_complement: Symbol<DecToBinConversions> = unsafe {
            lib.get(b"signed_decimal_to_ones_complement")
                .expect("Failed to get signed_decimal_to_ones_complement")
        };

        let result = unsafe { signed_decimal_to_ones_complement(-1, 32) };
        assert!(!result.is_null(), "return value is NULL");
        let result = unsafe { CStr::from_ptr(result) };
        let result = result.to_string_lossy().into_owned();
        assert_eq!(
            result, "11111111111111111111111111111110",
            "test_3 - FAILED"
        );
    }

    #[test]
    fn test_4() {
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let signed_decimal_to_ones_complement: Symbol<DecToBinConversions> = unsafe {
            lib.get(b"signed_decimal_to_ones_complement")
                .expect("Failed to get signed_decimal_to_ones_complement")
        };
        let result = unsafe { signed_decimal_to_ones_complement(255, 16) };
        assert!(!result.is_null(), "return value is NULL");
        let result = unsafe { CStr::from_ptr(result) };
        let result = result.to_string_lossy().into_owned();
        assert_eq!(
            result, "0000000011111111",
            "test_4 - FAILED"
        );
    }

    #[test]
    fn test_5() {
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let signed_decimal_to_ones_complement: Symbol<DecToBinConversions> = unsafe {
            lib.get(b"signed_decimal_to_ones_complement")
                .expect("Failed to get signed_decimal_to_ones_complement")
        };
        let result = rand::random_range(0..=i64::MAX);
        let input_num = if rand::random_bool(0.5) {-result} else {result};
        let return_val = unsafe{signed_decimal_to_ones_complement(input_num,64)};
        let expected = format!("{:0>64b}", if input_num < 0 {!result} else {result});
        let return_val = unsafe {CStr::from_ptr(return_val)};
        let return_val = return_val.to_string_lossy().into_owned();
        assert_eq!(expected,return_val);
    }
}
