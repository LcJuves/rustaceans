use std::ffi::*;
use std::os::raw::*;

use rjni::*;

impl_jni_on_load!(vm, _, {
    let env = env_from_vm!(vm);

    let jcls_call_jni = env_call!(env, find_class, char_const_ptr!("CallJNI"));
    let jfid_load_status = env_call!(
        env,
        get_static_field_id,
        jcls_call_jni,
        char_const_ptr!("loadStatus"),
        char_const_ptr!("Ljava/lang/String;")
    );
    env_call!(
        env,
        set_static_object_field,
        jcls_call_jni,
        jfid_load_status,
        jstring!(env, "Loaded")
    );
    JNI_VERSION_1_1
});

impl_jni_on_unload!(_, _, {
    println!("JNI >>> OnUnload");
});

jni_fn!(Java_CallJNI_getVersion, (env: *mut JNIEnv, _: Jclass), Jint, {
    env_call!(env, get_version)
});

jni_fn!(
    Java_CallJNI_defineClass,
    (
        env: *mut JNIEnv,
        _jcls: Jclass,
        _name: *const c_char,
        _loader: Jobject,
        _buf: *const Jbyte,
        _len: Jsize
    ),
    Jclass,
    { env_call!(env, find_class, char_const_ptr!("java/lang/Class")) }
);

jni_fn!(Java_CallJNI_findClass, (env: *mut JNIEnv, _: Jclass, name: Jstring), Jclass, {
    let c_str = env_call!(env, get_string_utfchars, name, JNI_FALSE as *mut Jboolean);
    env_call!(env, find_class, c_str)
});

jni_fn!(
    Java_CallJNI_fromReflectedMethod,
    (env: *mut JNIEnv, _: Jclass, method: Jobject),
    Jstring,
    {
        let jcls_string = env_call!(env, find_class, char_const_ptr!("java/lang/String"));
        let method_id = env_call!(env, from_reflected_method, method);
        env_call!(
            env,
            call_static_object_method,
            jcls_string,
            method_id,
            JNI_FALSE as *mut Jboolean
        ) as Jstring
    }
);

jni_fn!(Java_CallJNI_fromReflectedField, (env: *mut JNIEnv, _: Jclass, field: Jobject), Jobject, {
    let jcls_system = env_call!(env, find_class, char_const_ptr!("java/lang/System"));
    let field_id = env_call!(env, from_reflected_field, field);
    env_call!(env, get_static_object_field, jcls_system, field_id)
});

jni_fn!(Java_CallJNI_toReflectedMethod, (env: *mut JNIEnv, _: Jclass), Jobject, {
    let jcls_string = env_call!(env, find_class, char_const_ptr!("java/lang/String"));
    let jmid_value_of = env_call!(
        env,
        get_static_method_id,
        jcls_string,
        char_const_ptr!("valueOf"),
        char_const_ptr!("(Z)Ljava/lang/String;")
    );
    env_call!(env, to_reflected_method, jcls_string, jmid_value_of, JNI_TRUE)
});

jni_fn!(Java_CallJNI_getSystemOut, (env: *mut JNIEnv, _: Jclass), Jobject, {
    let jcls_system = env_call!(env, find_class, char_const_ptr!("java/lang/System"));
    let jfid_out = env_call!(
        env,
        get_static_field_id,
        jcls_system,
        char_const_ptr!("out"),
        char_const_ptr!("Ljava/io/PrintStream;")
    );
    env_call!(env, get_static_object_field, jcls_system, jfid_out)
});
