#[macro_use]
mod rbind;

use rbind::*;
use std::ffi::*;
use std::os::raw::*;

impl_jni_on_load!(_, _, {
    println!("JNI >>> OnLoad");
    JNI_VERSION_1_1
});

impl_jni_on_unload!(_, _, {
    println!("JNI >>> OnUnload");
});

unsafe_jni_fn_def!(Java_CallJNI_getVersion, (env: *mut JNIEnv, _: Jclass), Jint, {
    let fn_get_version = (*(*env)).get_version.unwrap();
    fn_get_version(env)
});

unsafe_jni_fn_def!(
    Java_CallJNI_defineClass,
    (
        env: *mut JNIEnv,
        _: Jclass,
        _name: *const c_char,
        _loader: Jobject,
        _buf: *const Jbyte,
        _len: Jsize
    ),
    Jclass,
    {
        let fn_find_class = (*(*env)).find_class.unwrap();
        fn_find_class(env, CString::new("java/lang/Class").unwrap().into_raw())
    }
);

unsafe_jni_fn_def!(Java_CallJNI_findClass, (env: *mut JNIEnv, _: Jclass, name: Jstring), Jclass, {
    let fn_get_string_utf_chars = (*(*env)).get_string_utfchars.unwrap();
    let c_str = fn_get_string_utf_chars(env, name, JNI_FALSE as *mut Jboolean);

    let fn_find_class = (*(*env)).find_class.unwrap();
    fn_find_class(env, c_str)
});

unsafe_jni_fn_def!(Java_CallJNI_getSystemOut, (env: *mut JNIEnv, _: Jclass), Jobject, {
    let fn_find_class = (*(*env)).find_class.unwrap();
    let jcls_system = fn_find_class(env, CString::new("java/lang/System").unwrap().into_raw());

    let fn_get_static_field_id = (*(*env)).get_static_field_id.unwrap();
    let jfid_out = fn_get_static_field_id(
        env,
        jcls_system,
        CString::new("out").unwrap().into_raw(),
        CString::new("Ljava/io/PrintStream;").unwrap().into_raw(),
    );

    let fn_get_static_object_field = (*(*env)).get_static_object_field.unwrap();
    let out = fn_get_static_object_field(env, jcls_system, jfid_out);

    out
});
