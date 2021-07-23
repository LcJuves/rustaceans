mod rjdef;
mod rust_jni_sys;

use crate::rust_jni_sys::jboolean;
use rust_jni_sys::{JNIEnv, JNI_FALSE};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use rust_jni_sys::{jclass, jstring};

#[no_mangle]
pub extern "system" fn Java_HelloWorld_hello(
    env: *mut JNIEnv,
    _class: jclass,
    input: jstring,
) -> jstring {
    unsafe {
        let fn_get_string_utf_chars = (*(*env)).GetStringUTFChars.unwrap();
        let c_str: *const c_char = fn_get_string_utf_chars(env, input, JNI_FALSE as *mut jboolean);
        let r_str = CStr::from_ptr(c_str).to_string_lossy();

        println!("String from java: {}", r_str);

        let r_str = format!("Hello, {}!", r_str);

        println!("String from java: {}", r_str);

        let fn_new_string_utf = (*(*env)).NewStringUTF.unwrap();
        return fn_new_string_utf(env, CString::new(r_str).unwrap().into_raw());
    }
}
