#[cfg(test)]
pub mod test {
    use std::ffi::{c_char, c_long, CString};

    use libloading::{Library, Symbol};
    type SignedBinToDec = unsafe extern "C" fn(*const c_char) -> c_long;
    #[test]
    fn test_1() {
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let binary_to_twos_complement_decimal: Symbol<SignedBinToDec> = unsafe {
            lib.get(b"binary_to_twos_complement_decimal")
                .expect("Failed to get unsigned_bin_to_dec")
        };
        let str = CString::new("1111").expect("Failed to create c_string");
        let result = unsafe { binary_to_twos_complement_decimal(str.as_ptr()) };
        assert_eq!(result, -1, "binary_to_twos_complement_decimal1 - FAILED!");
    }

    #[test]
    fn test_2() {
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let binary_to_twos_complement_decimal: Symbol<SignedBinToDec> = unsafe {
            lib.get(b"binary_to_twos_complement_decimal")
                .expect("Failed to get unsigned_bin_to_dec")
        };
        let str = CString::new("11111110").expect("Failed to create c_string");
        let result = unsafe { binary_to_twos_complement_decimal(str.as_ptr()) };
        assert_eq!(result, -2, "binary_to_twos_complement_decimal2 - FAILED!");
    }

    #[test]
    fn test_3() {
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let binary_to_twos_complement_decimal: Symbol<SignedBinToDec> = unsafe {
            lib.get(b"binary_to_twos_complement_decimal")
                .expect("Failed to get unsigned_bin_to_dec")
        };
        let str = CString::new("01111111").expect("Failed to create c_string");
        let result = unsafe { binary_to_twos_complement_decimal(str.as_ptr()) };
        assert_eq!(result, 127, "binary_to_twos_complement_decimal3 - FAILED!");
    }

    #[test]
    fn test_4() {
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let binary_to_twos_complement_decimal: Symbol<SignedBinToDec> = unsafe {
            lib.get(b"binary_to_twos_complement_decimal")
                .expect("Failed to get unsigned_bin_to_dec")
        };
        let str = CString::new("10101010").expect("Failed to create c_string");
        let result = unsafe { binary_to_twos_complement_decimal(str.as_ptr()) };
        assert_eq!(
            result,
            (0b10101010u8 as i8) as i64,
            "binary_to_twos_complement_decimal3 - FAILED!"
        );
    }

    #[test]
    fn test_5() {
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let binary_to_twos_complement_decimal: Symbol<SignedBinToDec> = unsafe {
            lib.get(b"binary_to_twos_complement_decimal")
                .expect("Failed to get unsigned_bin_to_dec")
        };
        let input: i64 = rand::random_range(i64::MIN..=i64::MAX);

        let str = CString::new(format!("{:#b}", input)).expect("Failed to create c_string");
        println!("{:#b}", input);
        let result = unsafe { binary_to_twos_complement_decimal(str.as_ptr()) };
        assert_eq!(
            result, input,
            "binary_to_twos_complement_decimal3 - FAILED!"
        );
    }
}
