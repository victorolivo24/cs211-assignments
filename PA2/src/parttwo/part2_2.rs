#[cfg(test)]
pub mod test {

    use libloading::{Library, Symbol};
    use std::ffi::{c_char, c_long, c_uint, CStr};
    type DecToBinConversions = unsafe extern "C" fn(c_long, c_uint) -> *mut c_char;

    #[test]
    fn test_1() {
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let signed_decimal_to_signed_magnitude: Symbol<DecToBinConversions> = unsafe {
            lib.get(b"signed_decimal_to_signed_magnitude")
                .expect("Failed to get signed_decimal_to_signed_magnitude")
        };

        let result = unsafe { signed_decimal_to_signed_magnitude(15, 7) };
        assert!(!result.is_null(), "return value is NULL");
        let result = unsafe { CStr::from_ptr(result) };
        let result = result.to_string_lossy().into_owned();
        assert_eq!(result, "0001111", "test_1 - FAILED")
    }

    #[test]
    fn test_2() {
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let signed_decimal_to_signed_magnitude: Symbol<DecToBinConversions> = unsafe {
            lib.get(b"signed_decimal_to_signed_magnitude")
                .expect("Failed to get signed_decimal_to_signed_magnitude")
        };

        let result = unsafe { signed_decimal_to_signed_magnitude(-53, 10) };
        assert!(!result.is_null(), "return value is NULL");
        let result = unsafe { CStr::from_ptr(result) };
        let result = result.to_string_lossy().into_owned();
        assert_eq!(result, "1000110101", "test_2 - FAILED")
    }

    #[test]
    fn test_3() {
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let signed_decimal_to_signed_magnitude: Symbol<DecToBinConversions> = unsafe {
            lib.get(b"signed_decimal_to_signed_magnitude")
                .expect("Failed to get signed_decimal_to_signed_magnitude")
        };

        let result = unsafe { signed_decimal_to_signed_magnitude(-1, 32) };
        assert!(!result.is_null(), "return value is NULL");
        let result = unsafe { CStr::from_ptr(result) };
        let result = result.to_string_lossy().into_owned();
        assert_eq!(
            result, "10000000000000000000000000000001",
            "test_3 - FAILED"
        )
    }

    #[test]
    fn test_4() {
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let signed_decimal_to_signed_magnitude: Symbol<DecToBinConversions> = unsafe {
            lib.get(b"signed_decimal_to_signed_magnitude")
                .expect("Failed to get signed_decimal_to_signed_magnitude")
        };
        let num = -3134978898391187824;
        let result = unsafe { signed_decimal_to_signed_magnitude(num, 64) };
        assert!(!result.is_null(), "return value is NULL");
        let result = unsafe { CStr::from_ptr(result) };
        let result = result.to_string_lossy().into_owned();
        assert_eq!(
            result, "1010101110000001101011101011001011000000111100111100000101110000",
            "test_4 - FAILED"
        )
    }

    #[test]
    fn test_5() {
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let signed_decimal_to_signed_magnitude: Symbol<DecToBinConversions> = unsafe {
            lib.get(b"signed_decimal_to_signed_magnitude")
                .expect("Failed to get signed_decimal_to_signed_magnitude")
        };
        let num = rand::random_range(0..=i64::MAX);
        let num_bits: i64 = ((num.ilog2() + 2) as i64) as i64;
        let temp_num = if rand::random_bool(0.5) { -num } else { num };
        let result = unsafe { signed_decimal_to_signed_magnitude(temp_num, num_bits as u32) };
        assert!(!result.is_null(), "return value is NULL");

        let abs_bin = format!("{:b}", num);
        let sign_bit = if temp_num < 0 { "1" } else { "0" };
        let padded_binary = format!("{:0width$}", abs_bin, width = (num_bits - 1) as usize);
        let expected = format!("{}{}", sign_bit, padded_binary);
        
        let result = unsafe { CStr::from_ptr(result) };
        let result = result.to_string_lossy().into_owned();
        assert_eq!(result, expected, "test_5 - FAILED")
    }
}
