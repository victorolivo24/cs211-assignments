#[cfg(test)]
pub mod test{
    use libloading::{Library, Symbol};
    use std::ffi::{c_char, c_ulong, CString};
    type FuncUnsignedDecToBin = unsafe extern "C" fn(*const c_char) -> c_ulong;
    #[test]
    fn test_1() {

        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let unsigned_dec_to_bin: Symbol<FuncUnsignedDecToBin> = unsafe {
            lib.get(b"binary_to_unsigned_decimal")
                .expect("Failed to get binary_to_unsigned_decimal")
        };
        let str = CString::new("1111").expect("Failed to create c_string");
        let result = unsafe { unsigned_dec_to_bin(str.as_ptr()) };
        assert_eq!(result, 15, "test1 - FAILED!");
    }
    #[test]
    fn test_2() {

        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let unsigned_dec_to_bin: Symbol<FuncUnsignedDecToBin> = unsafe {
            lib.get(b"binary_to_unsigned_decimal")
                .expect("Failed to get binary_to_unsigned_decimal")
        };
        let str = CString::new("10000000").expect("Failed to create c_string");
        let result = unsafe { unsigned_dec_to_bin(str.as_ptr()) };
        assert_eq!(result, 128, "test2 - FAILED!");
    }

    #[test]
    fn test_3() {

        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let unsigned_dec_to_bin: Symbol<FuncUnsignedDecToBin> = unsafe {
            lib.get(b"binary_to_unsigned_decimal")
                .expect("Failed to get binary_to_unsigned_decimal")
        };
        let str = CString::new("0").expect("Failed to create c_string");
        let result = unsafe { unsigned_dec_to_bin(str.as_ptr()) };
        assert_eq!(result, 0, "test3 - FAILED!");
    }

    #[test]
    fn test_4() {

        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let unsigned_dec_to_bin: Symbol<FuncUnsignedDecToBin> = unsafe {
            lib.get(b"binary_to_unsigned_decimal")
                .expect("Failed to get binary_to_unsigned_decimal")
        };
        let str = CString::new("1111111111111111111111111111111111111111111111111111111111111111")
            .expect("Failed to create c_string");
        let result = unsafe { unsigned_dec_to_bin(str.as_ptr()) };
        assert_eq!(
            result, 0b1111111111111111111111111111111111111111111111111111111111111111,
            "test4 - FAILED!"
        );
    }

    #[test]
    fn test_5() {

        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let unsigned_dec_to_bin: Symbol<FuncUnsignedDecToBin> = unsafe {
            lib.get(b"binary_to_unsigned_decimal")
                .expect("Failed to get binary_to_unsigned_decimal")
        };
        let rand_num: u64 = rand::random_range(0..=u64::MAX);
        let str = CString::new(format!("{:#b}",rand_num))
            .expect("Failed to create c_string");
        let result = unsafe { unsigned_dec_to_bin(str.as_ptr()) };
        assert_eq!(
            result, rand_num,
            "test5 - FAILED!"
        );
    }
}