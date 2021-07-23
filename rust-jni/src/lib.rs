mod rjdef;
// mod rust_jni_sys;

// use crate::rust_jni_sys::jboolean;
// use rust_jni_sys::{JNIEnv, JNI_FALSE};
// use rust_jni_sys::{jclass, jstring};

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::os::raw::c_uint;
use std::os::raw::c_void;

use rjdef::{
    jboolean, jclass, jint, jstring, JNIEnv, JNINativeInterface, JavaVM, JNI_FALSE, JNI_OK,
    JNI_VERSION_10,
};

#[no_mangle]
pub unsafe extern "system" fn JNI_OnLoad(vm: *mut JavaVM, reserved: *mut c_void) -> jint {
    /* code */
    /* The return value must be >= JNI_VERSION_1_1 */
    println!("OnLoad");
    JNI_VERSION_10
}

#[no_mangle]
pub unsafe extern "system" fn JNI_OnUnload(vm: *mut JavaVM, reserved: *mut c_void) {
    /* code */
}

// #[no_mangle]
// pub extern "system" fn Java_HelloWorld_hello(
//     env: *mut JNIEnv,
//     _class: jclass,
//     input: jstring,
// ) -> jstring {
//     unsafe {
//         let fn_get_string_utf_chars = (*(*env)).GetStringUTFChars;

//         let c_str: *const c_char = fn_get_string_utf_chars(env, input, JNI_FALSE as *mut jboolean);
//         let r_str = CStr::from_ptr(c_str).to_string_lossy();

//         let fn_new_string_utf = (*(*env)).NewStringUTF;

//         let fn_find_class = (*(*env)).FindClass;

//         let jcls_string = fn_find_class(env, CString::new("java/lang/String").unwrap().into_raw());

//         let fn_get_static_methodid = (*(*env)).GetStaticMethodID;

//         let fn_get_version = (*(*env)).GetVersion;
//         let version = fn_get_version(env);

//         let jmid_string_valueof = fn_get_static_methodid(
//             env,
//             jcls_string,
//             CString::new("valueOf").unwrap().into_raw(),
//             CString::new("(I)Ljava/lang/String;").unwrap().into_raw(),
//         );

//         let fn_call_object_method = (*(*env)).CallObjectMethod;
//         let value_of_string =
//             fn_call_object_method(env, jcls_string, jmid_string_valueof, version);

//         let c_str: *const c_char =
//             fn_get_string_utf_chars(env, value_of_string, JNI_FALSE as *mut jboolean);

//         let r_str2 = CStr::from_ptr(c_str).to_string_lossy();

//         // let jcls_string = (*(*env)).FindClass(env,);

//         println!("String from java: {}, {}", r_str, r_str2);

//         match (*(*env)).GetModule {
//             Some(_)=>{

//             }
//             None=> {
//                 println!("PASS")
//             }
//         }

//         let r_str = format!("Hello, {}!", r_str);

//         println!("String from java: {}", r_str);

//         let fn_new_string_utf = (*(*env)).NewStringUTF;
//         return fn_new_string_utf(env, CString::new(r_str).unwrap().into_raw());
//     }
// }
