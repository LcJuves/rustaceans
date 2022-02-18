use std::ffi::*;
use std::os::raw::*;

use rjni::*;

fn char_const_ptr(str: &str) -> *const c_char {
    let c_string = CString::new(str);
    c_string.unwrap().into_raw()
}

fn new_jstring(env: *mut JNIEnv, str: &str) -> Jstring {
    let fn_new_string_utf = unsafe { (*(*env)).new_string_utf.unwrap() };
    unsafe { fn_new_string_utf(env, char_const_ptr(str)) }
}

impl_jni_on_load!(vm, _, {
    unsafe {
        let fn_get_env = (*(*vm)).get_env.unwrap();
        let mut jenv = std::ptr::null_mut::<c_void>();
        fn_get_env(vm, &mut jenv as *mut *mut c_void, JNI_VERSION_1_1);
        let jenv = jenv as *mut JNIEnv;

        let fn_find_class = (*(*jenv)).find_class.unwrap();
        let fn_get_static_field_id = (*(*jenv)).get_static_field_id.unwrap();
        let fn_set_static_object_field = (*(*jenv)).set_static_object_field.unwrap();

        let jcls_call_jni = fn_find_class(jenv, char_const_ptr("CallJNI"));
        let jfid_load_status = fn_get_static_field_id(
            jenv,
            jcls_call_jni,
            char_const_ptr("loadStatus"),
            char_const_ptr("Ljava/lang/String;"),
        );
        fn_set_static_object_field(
            jenv,
            jcls_call_jni,
            jfid_load_status,
            new_jstring(jenv, "Loaded"),
        );
    }
    JNI_VERSION_1_1
});

impl_jni_on_unload!(_, _, {
    println!("JNI >>> OnUnload");
});

unsafe_jni_fn!(Java_CallJNI_getVersion, (env: *mut JNIEnv, _: Jclass), Jint, {
    let fn_get_version = (*(*env)).get_version.unwrap();
    fn_get_version(env)
});

unsafe_jni_fn!(
    Java_CallJNI_defineClass,
    (
        env: *mut JNIEnv,
        jcls: Jclass,
        _name: *const c_char,
        _loader: Jobject,
        _buf: *const Jbyte,
        _len: Jsize
    ),
    Jclass,
    { Java_CallJNI_findClass(env, jcls, new_jstring(env, "java/lang/Class"),) }
);

unsafe_jni_fn!(Java_CallJNI_findClass, (env: *mut JNIEnv, _: Jclass, name: Jstring), Jclass, {
    let fn_get_string_utf_chars = (*(*env)).get_string_utfchars.unwrap();
    let c_str = fn_get_string_utf_chars(env, name, JNI_FALSE as *mut Jboolean);

    let fn_find_class = (*(*env)).find_class.unwrap();
    fn_find_class(env, c_str)
});

unsafe_jni_fn!(
    Java_CallJNI_fromReflectedMethod,
    (env: *mut JNIEnv, jcls: Jclass, method: Jobject),
    Jstring,
    {
        let jcls_string = Java_CallJNI_findClass(env, jcls, new_jstring(env, "java/lang/String"));

        let fn_from_reflected_method = (*(*env)).from_reflected_method.unwrap();
        let method_id = fn_from_reflected_method(env, method);

        let fn_call_static_object_method = (*(*env)).call_static_object_method.unwrap();
        let call_object_ret =
            fn_call_static_object_method(env, jcls_string, method_id, JNI_FALSE as *mut Jboolean);
        call_object_ret as Jstring
    }
);

unsafe_jni_fn!(Java_CallJNI_getSystemOut, (env: *mut JNIEnv, jcls: Jclass), Jobject, {
    let jcls_system = Java_CallJNI_findClass(env, jcls, new_jstring(env, "java/lang/System"));

    let fn_get_static_field_id = (*(*env)).get_static_field_id.unwrap();
    let jfid_out = fn_get_static_field_id(
        env,
        jcls_system,
        char_const_ptr("out"),
        char_const_ptr("Ljava/io/PrintStream;"),
    );

    let fn_get_static_object_field = (*(*env)).get_static_object_field.unwrap();
    let out = fn_get_static_object_field(env, jcls_system, jfid_out);

    out
});
