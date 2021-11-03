// use std::ffi::CStr;

// pub struct JStr {
//     env: *mut JNIEnv,
//     inner: CStr,
// }

// impl JStr {
//     pub fn from_raw(env: *mut JNIEnv, str: jstring) -> Self {
//         let fn_get_string_utf_chars = (*(*env)).GetStringUTFChars;
//         let c_str_ptr = fn_get_string_utf_chars(env, jstring, JNI_FALSE as *mut jboolean);
//         JStr { inner: ptr }
//     }
// }
