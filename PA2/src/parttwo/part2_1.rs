#[cfg(test)]
pub mod test{

    use std::ffi::{c_char, c_uint, c_ulong, CStr};
    use libloading::{Library, Symbol};
    type DecToBinConversions = unsafe extern "C" fn(c_ulong, c_uint) -> *mut c_char;

    #[test]
    fn test_1(){
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let unsigned_decimal_to_binary: Symbol<DecToBinConversions> = unsafe {
            lib.get(b"unsigned_decimal_to_binary")
                .expect("Failed to get unsigned_decimal_to_binary")
        };
        
        let result = unsafe{unsigned_decimal_to_binary(15,5)};
        assert!(!result.is_null(),"return value is NULL");
        let result = unsafe { CStr::from_ptr(result) };
        let result = result.to_string_lossy().into_owned();
        assert_eq!(result,"01111","test_1 - FAILED")
    }

    #[test]
    fn test_2(){
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let unsigned_decimal_to_binary: Symbol<DecToBinConversions> = unsafe {
            lib.get(b"unsigned_decimal_to_binary")
                .expect("Failed to get unsigned_decimal_to_binary")
        };
        
        let result = unsafe{unsigned_decimal_to_binary(53,6)};
        assert!(!result.is_null(),"return value is NULL");
        let result = unsafe { CStr::from_ptr(result) };
        let result = result.to_string_lossy().into_owned();
        assert_eq!(result,"110101","test_2 - FAILED")
    }

    #[test]
    fn test_3(){
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let unsigned_decimal_to_binary: Symbol<DecToBinConversions> = unsafe {
            lib.get(b"unsigned_decimal_to_binary")
                .expect("Failed to get unsigned_decimal_to_binary")
        };
        
        let result = unsafe{unsigned_decimal_to_binary(150,8)};
        assert!(!result.is_null(),"return value is NULL");
        let result = unsafe { CStr::from_ptr(result) };
        let result = result.to_string_lossy().into_owned();
        assert_eq!(result,"10010110","test_3 - FAILED")
    }

    #[test]
    fn test_4(){
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let unsigned_decimal_to_binary: Symbol<DecToBinConversions> = unsafe {
            lib.get(b"unsigned_decimal_to_binary")
                .expect("Failed to get unsigned_decimal_to_binary")
        };
        let num = 928392;
        let result = unsafe{unsigned_decimal_to_binary(num,num.ilog2() + 1)};
        assert!(!result.is_null(),"return value is NULL");
        let result = unsafe { CStr::from_ptr(result) };
        let result = result.to_string_lossy().into_owned();
        assert_eq!(result,format!("{:b}",num),"test_4 - FAILED")
    }

    #[test]
    fn test_5(){
        let lib = unsafe { Library::new("c_src/pa2.so").expect("Failed to load library") };
        let unsigned_decimal_to_binary: Symbol<DecToBinConversions> = unsafe {
            lib.get(b"unsigned_decimal_to_binary")
                .expect("Failed to get unsigned_decimal_to_binary")
        };
        let num = rand::random_range(0..=u64::MAX);
        let result = unsafe{unsigned_decimal_to_binary(num,64)};
        let result = unsafe{CStr::from_ptr(result)};
        let result = result.to_string_lossy().into_owned();
        let padded_binary = format!("{:064b}", num);
        assert_eq!(result,padded_binary,"test_5 - FAILED!");
    }
}