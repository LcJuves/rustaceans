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

unsafe_jni_fn_def!(
    Java_CallJNI_getSystemOut,
    (env: *mut JNIEnv, _: jclass),
    jobject,
    {
        let fn_find_class = (*(*env)).FindClass.unwrap();
        let jcls_system = fn_find_class(env, CString::new("java/lang/System").unwrap().into_raw());

        let fn_get_static_field_id = (*(*env)).GetStaticFieldID.unwrap();
        let jfid_out = fn_get_static_field_id(
            env,
            jcls_system,
            CString::new("out").unwrap().into_raw(),
            CString::new("Ljava/io/PrintStream;").unwrap().into_raw(),
        );

        let fn_get_static_object_field = (*(*env)).GetStaticObjectField.unwrap();
        let out = fn_get_static_object_field(env, jcls_system, jfid_out);

        out
    }
);
