#[macro_use]
mod jdef;

use jdef::*;
use std::ffi::*;
use std::os::raw::*;

impl_jni_onload!(
  _,
  _,
  {
    println!("JNI >>> OnLoad");
    JNI_VERSION_10
  }
);

impl_jni_unload!(
  _,
  _,
  {
    println!("JNI >>> Unload");
  }
);

unsafe_jni_fn_def!(
    Java_CallJNI_getVersion,
    (env: *mut JNIEnv, _: jclass),
    jint,
    {
        let fn_get_version = (*(*env)).GetVersion.unwrap();
        fn_get_version(env)
    }
);

unsafe_jni_fn_def!(
    Java_CallJNI_defineClass,
    (
        env: *mut JNIEnv,
        _: jclass,
        _name: *const c_char,
        _loader: jobject,
        _buf: *const jbyte,
        _len: jsize
    ),
    jclass,
    {
        let fn_find_class = (*(*env)).FindClass.unwrap();
        fn_find_class(env, CString::new("java/lang/Class").unwrap().into_raw())
    }
);

unsafe_jni_fn_def!(
    Java_CallJNI_findClass,
    (env: *mut JNIEnv, _: jclass, name: jstring),
    jclass,
    {
        let fn_get_string_utf_chars = (*(*env)).GetStringUTFChars.unwrap();
        let c_str = fn_get_string_utf_chars(env, name, JNI_FALSE as *mut jboolean);

        let fn_find_class = (*(*env)).FindClass.unwrap();
        fn_find_class(env, c_str)
    }
);
