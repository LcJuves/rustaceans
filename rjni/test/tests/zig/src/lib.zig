const std = @import("std");

const jni = @import("zbind.zig");

const JNIEnv = jni.JNIEnv;
const JavaVM = jni.JavaVM;

const jclass = jni.jclass;
const jint = jni.jint;
const jstring = jni.jstring;
const jobject = jni.jobject;
const jfield_id = jni.jfield_id;
const jmethod_id = jni.jmethod_id;
const jboolean = jni.jboolean;
const jbyte_array = jni.jbyte_array;
const jthrowable = jni.jthrowable;

const jni_version_1_1 = jni.jni_version_1_1;
const jni_false = jni.jni_false;
const jni_true = jni.jni_true;
const jni_ok = jni.jni_ok;

pub export fn @"JNI_OnLoad"(vm: [*c]JavaVM, _: ?*anyopaque) jint {
    var env: [*c]JNIEnv = undefined;
    _ = vm.*.*.get_env.?(vm, @ptrCast([*c]?*anyopaque, &env), jni_version_1_1);
    const jcls_call_jni: jclass = env.*.*.find_class.?(env, "CallJNI");
    const jfid_load_status: jfield_id = env.*.*.get_static_field_id.?(env, jcls_call_jni, "loadStatus", "Ljava/lang/String;");
    const jstr_load_status: jstring = env.*.*.new_string_utf.?(env, "Loaded");
    env.*.*.set_static_object_field.?(env, jcls_call_jni, jfid_load_status, jstr_load_status);
    return jni_version_1_1;
}

pub export fn @"JNI_OnUnload"(_: [*c]JavaVM, _: ?*anyopaque) void {
    const stdout = std.io.getStdOut().writer();
    stdout.writeAll("JNI >>> OnUnload") catch unreachable;
}

pub export fn @"Java_CallJNI_getVersion"(env: [*c]JNIEnv, _: jclass) jint {
    return env.*.*.get_version.?(env);
}

pub export fn @"Java_CallJNI_defineClass"(env: [*c]JNIEnv, jcls: jclass, name: jstring, bytes: jbyte_array) jclass {
    const c_str_name = env.*.*.get_string_utf_chars.?(env, name, @intToPtr([*c]jboolean, jni_false));

    const jcls_class = env.*.*.find_class.?(env, "java/lang/Class");
    const jmid_get_class_loader = env.*.*.get_method_id.?(env, jcls_class, "getClassLoader", "()Ljava/lang/ClassLoader;");
    const loader = env.*.*.call_object_method.?(env, jcls, jmid_get_class_loader);

    const buf = env.*.*.get_byte_array_elements.?(env, bytes, @intToPtr([*c]jboolean, jni_false));
    const len = env.*.*.get_array_length.?(env, bytes);

    return env.*.*.define_class.?(env, c_str_name, loader, buf, len);
}

pub export fn @"Java_CallJNI_findClass"(env: [*c]JNIEnv, _: jclass, name: jstring) jclass {
    const c_str_name: [*c]const u8 = env.*.*.get_string_utf_chars.?(env, name, @intToPtr([*c]jboolean, jni_false));
    return env.*.*.find_class.?(env, c_str_name);
}

pub export fn @"Java_CallJNI_fromReflectedMethod"(env: [*c]JNIEnv, _: jclass, method: jobject) jstring {
    const jcls_string: jclass = env.*.*.find_class.?(env, "java/lang/String");
    const method_id: jmethod_id = env.*.*.from_reflected_method.?(env, method);
    return @ptrCast(jstring, env.*.*.call_static_object_method.?(env, jcls_string, method_id, jni_false));
}

pub export fn @"Java_CallJNI_fromReflectedField"(env: [*c]JNIEnv, _: jclass, field: jobject) jobject {
    const jcls_system: jclass = env.*.*.find_class.?(env, "java/lang/System");
    const field_id: jfield_id = env.*.*.from_reflected_field.?(env, field);
    return env.*.*.get_static_object_field.?(env, jcls_system, field_id);
}

pub export fn @"Java_CallJNI_toReflectedMethod"(env: [*c]JNIEnv, _: jclass) jobject {
    const jcls_string: jclass = env.*.*.find_class.?(env, "java/lang/String");
    const jmid_valueOf: jmethod_id = env.*.*.get_static_method_id.?(env, jcls_string, "valueOf", "(Z)Ljava/lang/String;");
    return env.*.*.to_reflected_method.?(env, jcls_string, jmid_valueOf, jni_true);
}

pub export fn @"Java_CallJNI_getSuperclass"(env: [*c]JNIEnv, _: jclass, clazz: jclass) jclass {
    return env.*.*.get_superclass.?(env, clazz);
}

pub export fn @"Java_CallJNI_isAssignableFrom"(env: [*c]JNIEnv, _: jclass, clazz1: jclass, clazz2: jclass) jboolean {
    return env.*.*.is_assignable_from.?(env, clazz1, clazz2);
}

pub export fn @"Java_CallJNI_toReflectedField"(env: [*c]JNIEnv, _: jclass) jobject {
    const jcls_system: jclass = env.*.*.find_class.?(env, "java/lang/System");
    const jfid_system_out: jfield_id = env.*.*.get_static_field_id.?(env, jcls_system, "out", "Ljava/io/PrintStream;");
    return env.*.*.to_reflected_field.?(env, jcls_system, jfid_system_out, jni_true);
}

// _00024 -> '$'
// _1 -> '_'
pub export fn @"Java_CallJNI__00024_1throw"(env: [*c]JNIEnv, _: jclass, obj: jthrowable) jint {
    return env.*.*.throw.?(env, obj);
}

pub export fn @"Java_CallJNI_throwNew"(env: [*c]JNIEnv, _: jclass, clazz: jclass, message: jstring) jint {
    const c_message = env.*.*.get_string_utf_chars.?(env, message, @intToPtr([*c]jboolean, jni_false));
    return env.*.*.throw_new.?(env, clazz, c_message);
}

pub export fn @"Java_CallJNI_exceptionOccurred"(env: [*c]JNIEnv, _: jclass) jthrowable {
    std.debug.assert(env.*.*.exception_occurred.?(env) == null);
    const jcls_runtime_exception = env.*.*.find_class.?(env, "java/lang/RuntimeException");
    std.debug.assert(env.*.*.throw_new.?(env, jcls_runtime_exception, "JNICALL") == jni_ok);
    const _jthrowable = env.*.*.exception_occurred.?(env);
    env.*.*.exception_clear.?(env);
    std.debug.assert(_jthrowable != null);
    return _jthrowable;
}

pub export fn @"Java_CallJNI_exceptionDescribe"(env: [*c]JNIEnv, _: jclass) void {
    const jcls_runtime_exception = env.*.*.find_class.?(env, "java/lang/RuntimeException");
    std.debug.assert(env.*.*.throw_new.?(env, jcls_runtime_exception, "JNICALL") == jni_ok);
    env.*.*.exception_describe.?(env);
}

pub export fn @"Java_CallJNI_exceptionClear"(env: [*c]JNIEnv, _: jclass) void {
    const jcls_runtime_exception = env.*.*.find_class.?(env, "java/lang/RuntimeException");
    std.debug.assert(env.*.*.throw_new.?(env, jcls_runtime_exception, "JNICALL") == jni_ok);
    env.*.*.exception_clear.?(env);
}

pub export fn @"Java_CallJNI_fatalError"(env: [*c]JNIEnv, _: jclass, msg: jstring) void {
    const c_msg = env.*.*.get_string_utf_chars.?(env, msg, @intToPtr([*c]jboolean, jni_false));
    env.*.*.fatal_error.?(env, c_msg);
}

pub export fn @"Java_CallJNI_getSystemOut"(env: [*c]JNIEnv, _: jclass) jobject {
    const jcls_system: jclass = env.*.*.find_class.?(env, "java/lang/System");
    const jfid_out: jfield_id = env.*.*.get_static_field_id.?(env, jcls_system, "out", "Ljava/io/PrintStream;");
    return env.*.*.get_static_object_field.?(env, jcls_system, jfid_out);
}
