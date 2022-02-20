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

const jni_version_1_1 = jni.jni_version_1_1;
var jni_false = jni.jni_false;
const jni_true = jni.jni_true;

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
    const stdout = @import("std").io.getStdOut().writer();
    stdout.writeAll("JNI >>> OnUnload") catch unreachable;
}

pub export fn @"Java_CallJNI_getVersion"(env: [*c]JNIEnv, _: jclass) jint {
    return env.*.*.get_version.?(env);
}

pub export fn @"Java_CallJNI_findClass"(env: [*c]JNIEnv, _: jclass, name: jstring) jclass {
    const c_str_name: [*c]const u8 = env.*.*.get_string_utf_chars.?(env, name, @intToPtr([*c]jboolean, jni_false.*));
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

pub export fn @"Java_CallJNI_getSystemOut"(env: [*c]JNIEnv, _: jclass) jobject {
    const jcls_system: jclass = env.*.*.find_class.?(env, "java/lang/System");
    const jfid_out: jfield_id = env.*.*.get_static_field_id.?(env, jcls_system, "out", "Ljava/io/PrintStream;");
    return env.*.*.get_static_object_field.?(env, jcls_system, jfid_out);
}
