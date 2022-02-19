const jni = @import("zbind.zig");

const JNIEnv = jni.JNIEnv;
const JavaVM = jni.JavaVM;

const jclass = jni.jclass;
const jint = jni.jint;
const jstring = jni.jstring;
const jobject = jni.jobject;
const jfieldID = jni.jfieldID;
const jmethodID = jni.jmethodID;

const JNI_VERSION_1_1 = jni.JNI_VERSION_1_1;
const JNI_FALSE = jni.JNI_FALSE;
const JNI_TRUE = jni.JNI_TRUE;

pub export fn JNI_OnLoad(vm: [*c]JavaVM, _: ?*anyopaque) jint {
    var env: [*c]JNIEnv = undefined;
    _ = vm.*.*.GetEnv.?(vm, @ptrCast([*c]?*anyopaque, &env), JNI_VERSION_1_1);
    var jcls_CallJNI: jclass = env.*.*.FindClass.?(env, "CallJNI");
    var jfid_loadStatus: jfieldID = env.*.*.GetStaticFieldID.?(env, jcls_CallJNI, "loadStatus", "Ljava/lang/String;");
    var jstr_loadStatus: jstring = env.*.*.NewStringUTF.?(env, "Loaded");
    env.*.*.SetStaticObjectField.?(env, jcls_CallJNI, jfid_loadStatus, jstr_loadStatus);
    return JNI_VERSION_1_1;
}

pub export fn JNI_OnUnload(_: [*c]JavaVM, _: ?*anyopaque) void {
    const stdout = @import("std").io.getStdOut().writer();
    stdout.writeAll("JNI >>> OnUnload") catch unreachable;
}

pub export fn Java_CallJNI_getVersion(env: [*c]JNIEnv, _: jclass) jint {
    return env.*.*.GetVersion.?(env);
}

pub export fn Java_CallJNI_findClass(env: [*c]JNIEnv, _: jclass, name: jstring) jclass {
    var c_str_name: [*c]const u8 = env.*.*.GetStringUTFChars.?(env, name, null);
    return env.*.*.FindClass.?(env, c_str_name);
}

pub export fn Java_CallJNI_fromReflectedMethod(env: [*c]JNIEnv, _: jclass, method: jobject) jstring {
    var jcls_String: jclass = env.*.*.FindClass.?(env, "java/lang/String");
    var method_id: jmethodID = env.*.*.FromReflectedMethod.?(env, method);
    return @ptrCast(jstring, env.*.*.CallStaticObjectMethod.?(env, jcls_String, method_id, JNI_FALSE));
}

pub export fn Java_CallJNI_fromReflectedField(env: [*c]JNIEnv, _: jclass, field: jobject) jobject {
    var jcls_System: jclass = env.*.*.FindClass.?(env, "java/lang/System");
    var field_id: jfieldID = env.*.*.FromReflectedField.?(env, field);
    return env.*.*.GetStaticObjectField.?(env, jcls_System, field_id);
}

pub export fn Java_CallJNI_toReflectedMethod(env: [*c]JNIEnv, _: jclass) jobject {
    var jcls_String: jclass = env.*.*.FindClass.?(env, "java/lang/String");
    var jmid_valueOf: jmethodID = env.*.*.GetStaticMethodID.?(env, jcls_String, "valueOf", "(Z)Ljava/lang/String;");
    return env.*.*.ToReflectedMethod.?(env, jcls_String, jmid_valueOf, JNI_TRUE);
}

pub export fn Java_CallJNI_getSystemOut(env: [*c]JNIEnv, _: jclass) jobject {
    var jcls_System: jclass = env.*.*.FindClass.?(env, "java/lang/System");
    var jfid_out: jfieldID = env.*.*.GetStaticFieldID.?(env, jcls_System, "out", "Ljava/io/PrintStream;");
    return env.*.*.GetStaticObjectField.?(env, jcls_System, jfid_out);
}
