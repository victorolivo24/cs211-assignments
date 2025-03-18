#[cfg(test)]
pub mod test{
    use std::ffi::{c_char, c_long, CString};

    use libloading::{Library, Symbol};
    type SignedBinToDec = unsafe extern "C" fn(*const c_char) -> c_long;

    #[test]
    fn test_1(){
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let binary_to_signed_magnitude_decimal: Symbol<SignedBinToDec> = unsafe {
            lib.get(b"binary_to_signed_magnitude_decimal")
                .expect("Failed to get binary_to_signed_magnitude_decimal")
        };

        let str = CString::new("1001")
            .expect("Failed to create c_string");
        let result = unsafe {binary_to_signed_magnitude_decimal(str.as_ptr()) };
        assert_eq!(
            result, -1,
            "binary_to_signed_magnitude_decimal1 - FAILED!"
        );
    }

    #[test]
    fn test_2(){
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let binary_to_signed_magnitude_decimal: Symbol<SignedBinToDec> = unsafe {
            lib.get(b"binary_to_signed_magnitude_decimal")
                .expect("Failed to get binary_to_signed_magnitude_decimal")
        };

        let str = CString::new("0001")
            .expect("Failed to create c_string");
        let result = unsafe {binary_to_signed_magnitude_decimal(str.as_ptr()) };
        assert_eq!(
            result, 1,
            "binary_to_signed_magnitude_decimal2 - FAILED!"
        );
    }

    #[test]
    fn test_3(){
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let binary_to_signed_magnitude_decimal: Symbol<SignedBinToDec> = unsafe {
            lib.get(b"binary_to_signed_magnitude_decimal")
                .expect("Failed to get binary_to_signed_magnitude_decimal")
        };

        let str = CString::new("11111")
            .expect("Failed to create c_string");
        let result = unsafe {binary_to_signed_magnitude_decimal(str.as_ptr()) };
        assert_eq!(
            result, -15,
            "binary_to_signed_magnitude_decimal3 - FAILED!"
        );
    }

    #[test]
    fn test_4(){
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let binary_to_signed_magnitude_decimal: Symbol<SignedBinToDec> = unsafe {
            lib.get(b"binary_to_signed_magnitude_decimal")
                .expect("Failed to get binary_to_signed_magnitude_decimal")
        };

        let str = CString::new("1000000011011")
            .expect("Failed to create c_string");
        let result = unsafe {binary_to_signed_magnitude_decimal(str.as_ptr()) };
        assert_eq!(
            result, -27,
            "binary_to_signed_magnitude_decimal4 - FAILED!"
        );
    }

    #[test]
    fn test_5(){
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let binary_to_signed_magnitude_decimal: Symbol<SignedBinToDec> = unsafe {
            lib.get(b"binary_to_signed_magnitude_decimal")
                .expect("Failed to get binary_to_signed_magnitude_decimal")
        };
        let rand_num: i64 = rand::random_range(0..i64::MAX);
        let signed = if rand::random_bool(0.5) {'1'} else {'0'};
        let str = CString::new(format!("{}{:b}",signed,rand_num))
            .expect("Failed to create c_string");
        let result = unsafe {binary_to_signed_magnitude_decimal(str.as_ptr()) };
        assert_eq!(
            result, if signed == '1' {-1 * rand_num} else {rand_num},
            "binary_to_signed_magnitude_decimal4 - FAILED!"
        );
    }
}