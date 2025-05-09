use std::ffi::*;

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
    (env: *mut JNIEnv, jcls: Jclass, name: Jstring, bytes: JbyteArray),
    Jclass,
    {
        let name = env_call!(env, get_string_utf_chars, name, JNI_FALSE as *mut Jboolean);

        let jcls_class = env_call!(env, find_class, char_const_ptr!("java/lang/Class"));
        let jmid_get_class_loader = env_call!(
            env,
            get_method_id,
            jcls_class,
            char_const_ptr!("getClassLoader"),
            char_const_ptr!("()Ljava/lang/ClassLoader;")
        );
        let loader = env_call!(env, call_object_method, jcls, jmid_get_class_loader);

        let buf = env_call!(env, get_byte_array_elements, bytes, JNI_FALSE as *mut Jboolean);
        let len = env_call!(env, get_array_length, bytes);
        env_call!(env, define_class, name, loader, buf, len)
    }
);

jni_fn!(Java_CallJNI_findClass, (env: *mut JNIEnv, _: Jclass, name: Jstring), Jclass, {
    let c_str = env_call!(env, get_string_utf_chars, name, JNI_FALSE as *mut Jboolean);
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

jni_fn!(Java_CallJNI_getSuperclass, (env: *mut JNIEnv, _: Jclass, clazz: Jclass), Jclass, {
    env_call!(env, get_superclass, clazz)
});

jni_fn!(
    Java_CallJNI_isAssignableFrom,
    (env: *mut JNIEnv, _: Jclass, clazz1: Jclass, clazz2: Jclass),
    Jboolean,
    { env_call!(env, is_assignable_from, clazz1, clazz2) }
);

jni_fn!(Java_CallJNI_toReflectedField, (env: *mut JNIEnv, _: Jclass), Jobject, {
    let jcls_system = env_call!(env, find_class, char_const_ptr!("java/lang/System"));
    let jfid_system_out = env_call!(
        env,
        get_static_field_id,
        jcls_system,
        char_const_ptr!("out"),
        char_const_ptr!("Ljava/io/PrintStream;")
    );
    env_call!(env, to_reflected_field, jcls_system, jfid_system_out, JNI_TRUE)
});

// _00024 -> '$'
// _1 -> '_'
jni_fn!(Java_CallJNI__00024_1throw, (env: *mut JNIEnv, _: Jclass, obj: Jthrowable), Jint, {
    env_call!(env, throw, obj)
});

jni_fn!(
    Java_CallJNI_throwNew,
    (env: *mut JNIEnv, _: Jclass, clazz: Jclass, message: Jstring),
    Jint,
    {
        let message = env_call!(env, get_string_utf_chars, message, JNI_FALSE as *mut Jboolean);
        env_call!(env, throw_new, clazz, message)
    }
);

jni_fn!(Java_CallJNI_exceptionOccurred, (env: *mut JNIEnv, _: Jclass), Jthrowable, {
    assert!(env_call!(env, exception_occurred).is_null());
    let jcls_runtime_exception =
        env_call!(env, find_class, char_const_ptr!("java/lang/RuntimeException"));
    assert!(
        env_call!(env, throw_new, jcls_runtime_exception, char_const_ptr!("JNICALL")) == JNI_OK
    );
    let jthrowable = env_call!(env, exception_occurred);
    env_call!(env, exception_clear);
    assert!(!jthrowable.is_null());
    jthrowable
});

jni_fn!(Java_CallJNI_exceptionDescribe, (env: *mut JNIEnv, _: Jclass), {
    let jcls_runtime_exception =
        env_call!(env, find_class, char_const_ptr!("java/lang/RuntimeException"));
    assert!(
        env_call!(env, throw_new, jcls_runtime_exception, char_const_ptr!("JNICALL")) == JNI_OK
    );
    env_call!(env, exception_describe);
});

jni_fn!(Java_CallJNI_exceptionClear, (env: *mut JNIEnv, _: Jclass), {
    let jcls_runtime_exception =
        env_call!(env, find_class, char_const_ptr!("java/lang/RuntimeException"));
    assert!(
        env_call!(env, throw_new, jcls_runtime_exception, char_const_ptr!("JNICALL")) == JNI_OK
    );
    env_call!(env, exception_clear);
});

jni_fn!(Java_CallJNI_fatalError, (env: *mut JNIEnv, _: Jclass, msg: Jstring), {
    let msg = env_call!(env, get_string_utf_chars, msg, JNI_FALSE as *mut Jboolean);
    env_call!(env, fatal_error, msg);
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
