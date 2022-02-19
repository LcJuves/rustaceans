pub const va_list = *anyopaque;

// Primitive types that match up with Java equivalents

pub const jboolean = [*c]const u8;
pub const jbyte = i8;
pub const jchar = u16;
pub const jshort = i16;
pub const jint = i32;
pub const jlong = i64;
pub const jfloat = f32;
pub const jdouble = f64;

/// "cardinal indices and sizes"
pub const jsize = jint;

// Reference types that match up with Java equivalents

pub const jobject = ?*anyopaque;
pub const jclass = jobject;
pub const jthrowable = jobject;
pub const jstring = jobject;

// Array types that match up with Java equivalents

pub const jarray = jobject;
pub const jbooleanArray = jarray;
pub const jbyteArray = jarray;
pub const jcharArray = jarray;
pub const jshortArray = jarray;
pub const jintArray = jarray;
pub const jlongArray = jarray;
pub const jfloatArray = jarray;
pub const jdoubleArray = jarray;
pub const jobjectArray = jarray;

pub const jweak = jobject;

/// When passing arguments from Rust to a Java method, the jvalue union is used
pub const jvalue = extern union {
    // Primitive types
    z: jboolean,
    b: jbyte,
    c: jchar,
    s: jshort,
    i: jint,
    j: jlong,
    f: jfloat,
    d: jdouble,
    // Reference types
    l: jobject,
};

pub const jfieldID = ?*anyopaque;
pub const jmethodID = ?*anyopaque;

/// Return values from JobjectRefType
pub const jobjectRefType = enum(c_uint) {
    JNIInvalidRefType = 0,
    JNILocalRefType = 1,
    JNIGlobalRefType = 2,
    JNIWeakGlobalRefType = 3,
};

pub const JNINativeMethod = extern struct {
    name: [*c]u8,
    signature: [*c]u8,
    fnPtr: ?*anyopaque,
};

/// `JNIEnv` implements the "Java Native Inferface", and contains most of what you'll use to interact with Java from Rust
pub const JNIEnv = [*c]const JNINativeInterface;
/// `JavaVM` (along with a handful of global functions) implements the "Java Invocation Interface",
/// which allow you to create and destroy a Java Virtual Machine
pub const JavaVM = [*c]const JNIInvokeInterface;

/// Table of interface function pointers
pub const JNINativeInterface = extern struct {
    reserved0: ?*anyopaque,
    reserved1: ?*anyopaque,
    reserved2: ?*anyopaque,

    reserved3: ?*anyopaque,
    GetVersion: ?fn (env: [*c]JNIEnv) callconv(.C) jint,

    DefineClass: ?fn (env: [*c]JNIEnv, name: [*c]const u8, loader: jobject, buf: [*c]const jbyte, len: jsize) callconv(.C) jclass,
    FindClass: ?fn (env: [*c]JNIEnv, name: [*c]const u8) callconv(.C) jclass,

    FromReflectedMethod: ?fn (env: [*c]JNIEnv, method: jobject) callconv(.C) jmethodID,
    FromReflectedField: ?fn (env: [*c]JNIEnv, field: jobject) callconv(.C) jfieldID,

    /// spec doesn't show Jboolean parameter
    ToReflectedMethod: ?fn (env: [*c]JNIEnv, cls: jclass, method_id: jmethodID, is_static: jboolean) callconv(.C) jobject,
    GetSuperclass: ?fn (env: [*c]JNIEnv, jclass) callconv(.C) jclass,
    IsAssignableFrom: ?fn (env: [*c]JNIEnv, jclass, jclass) callconv(.C) jboolean,
    ToReflectedField: ?fn (env: [*c]JNIEnv, jclass, jfieldID, jboolean) callconv(.C) jobject,
    Throw: ?fn (env: [*c]JNIEnv, jthrowable) callconv(.C) jint,
    ThrowNew: ?fn (env: [*c]JNIEnv, jclass, [*c]const u8) callconv(.C) jint,
    ExceptionOccurred: ?fn (env: [*c]JNIEnv) callconv(.C) jthrowable,
    ExceptionDescribe: ?fn (env: [*c]JNIEnv) callconv(.C) void,
    ExceptionClear: ?fn (env: [*c]JNIEnv) callconv(.C) void,
    FatalError: ?fn (env: [*c]JNIEnv, [*c]const u8) callconv(.C) void,
    PushLocalFrame: ?fn (env: [*c]JNIEnv, jint) callconv(.C) jint,
    PopLocalFrame: ?fn (env: [*c]JNIEnv, jobject) callconv(.C) jobject,
    NewGlobalRef: ?fn (env: [*c]JNIEnv, jobject) callconv(.C) jobject,
    DeleteGlobalRef: ?fn (env: [*c]JNIEnv, jobject) callconv(.C) void,
    DeleteLocalRef: ?fn (env: [*c]JNIEnv, jobject) callconv(.C) void,
    IsSameObject: ?fn (env: [*c]JNIEnv, jobject, jobject) callconv(.C) jboolean,
    NewLocalRef: ?fn (env: [*c]JNIEnv, jobject) callconv(.C) jobject,
    EnsureLocalCapacity: ?fn (env: [*c]JNIEnv, jint) callconv(.C) jint,
    AllocObject: ?fn (env: [*c]JNIEnv, jclass) callconv(.C) jobject,
    NewObject: ?fn (env: [*c]JNIEnv, jclass, jmethodID, ...) callconv(.C) jobject,
    NewObjectV: ?fn (env: [*c]JNIEnv, jclass, jmethodID, va_list) callconv(.C) jobject,
    NewObjectA: ?fn (env: [*c]JNIEnv, jclass, jmethodID, [*c]const jvalue) callconv(.C) jobject,
    GetObjectClass: ?fn (env: [*c]JNIEnv, jobject) callconv(.C) jclass,
    IsInstanceOf: ?fn (env: [*c]JNIEnv, jobject, jclass) callconv(.C) jboolean,
    GetMethodID: ?fn (env: [*c]JNIEnv, jclass, [*c]const u8, [*c]const u8) callconv(.C) jmethodID,
    CallObjectMethod: ?fn (env: [*c]JNIEnv, jobject, jmethodID, ...) callconv(.C) jobject,
    CallObjectMethodV: ?fn (env: [*c]JNIEnv, jobject, jmethodID, va_list) callconv(.C) jobject,
    CallObjectMethodA: ?fn (env: [*c]JNIEnv, jobject, jmethodID, [*c]const jvalue) callconv(.C) jobject,
    CallBooleanMethod: ?fn (env: [*c]JNIEnv, jobject, jmethodID, ...) callconv(.C) jboolean,
    CallBooleanMethodV: ?fn (env: [*c]JNIEnv, jobject, jmethodID, va_list) callconv(.C) jboolean,
    CallBooleanMethodA: ?fn (env: [*c]JNIEnv, jobject, jmethodID, [*c]const jvalue) callconv(.C) jboolean,
    CallByteMethod: ?fn (env: [*c]JNIEnv, jobject, jmethodID, ...) callconv(.C) jbyte,
    CallByteMethodV: ?fn (env: [*c]JNIEnv, jobject, jmethodID, va_list) callconv(.C) jbyte,
    CallByteMethodA: ?fn (env: [*c]JNIEnv, jobject, jmethodID, [*c]const jvalue) callconv(.C) jbyte,
    CallCharMethod: ?fn (env: [*c]JNIEnv, jobject, jmethodID, ...) callconv(.C) jchar,
    CallCharMethodV: ?fn (env: [*c]JNIEnv, jobject, jmethodID, va_list) callconv(.C) jchar,
    CallCharMethodA: ?fn (env: [*c]JNIEnv, jobject, jmethodID, [*c]const jvalue) callconv(.C) jchar,
    CallShortMethod: ?fn (env: [*c]JNIEnv, jobject, jmethodID, ...) callconv(.C) jshort,
    CallShortMethodV: ?fn (env: [*c]JNIEnv, jobject, jmethodID, va_list) callconv(.C) jshort,
    CallShortMethodA: ?fn (env: [*c]JNIEnv, jobject, jmethodID, [*c]const jvalue) callconv(.C) jshort,
    CallIntMethod: ?fn (env: [*c]JNIEnv, jobject, jmethodID, ...) callconv(.C) jint,
    CallIntMethodV: ?fn (env: [*c]JNIEnv, jobject, jmethodID, va_list) callconv(.C) jint,
    CallIntMethodA: ?fn (env: [*c]JNIEnv, jobject, jmethodID, [*c]const jvalue) callconv(.C) jint,
    CallLongMethod: ?fn (env: [*c]JNIEnv, jobject, jmethodID, ...) callconv(.C) jlong,
    CallLongMethodV: ?fn (env: [*c]JNIEnv, jobject, jmethodID, va_list) callconv(.C) jlong,
    CallLongMethodA: ?fn (env: [*c]JNIEnv, jobject, jmethodID, [*c]const jvalue) callconv(.C) jlong,
    CallFloatMethod: ?fn (env: [*c]JNIEnv, jobject, jmethodID, ...) callconv(.C) jfloat,
    CallFloatMethodV: ?fn (env: [*c]JNIEnv, jobject, jmethodID, va_list) callconv(.C) jfloat,
    CallFloatMethodA: ?fn (env: [*c]JNIEnv, jobject, jmethodID, [*c]const jvalue) callconv(.C) jfloat,
    CallDoubleMethod: ?fn (env: [*c]JNIEnv, jobject, jmethodID, ...) callconv(.C) jdouble,
    CallDoubleMethodV: ?fn (env: [*c]JNIEnv, jobject, jmethodID, va_list) callconv(.C) jdouble,
    CallDoubleMethodA: ?fn (env: [*c]JNIEnv, jobject, jmethodID, [*c]const jvalue) callconv(.C) jdouble,
    CallVoidMethod: ?fn (env: [*c]JNIEnv, jobject, jmethodID, ...) callconv(.C) void,
    CallVoidMethodV: ?fn (env: [*c]JNIEnv, jobject, jmethodID, va_list) callconv(.C) void,
    CallVoidMethodA: ?fn (env: [*c]JNIEnv, jobject, jmethodID, [*c]const jvalue) callconv(.C) void,
    CallNonvirtualObjectMethod: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, ...) callconv(.C) jobject,
    CallNonvirtualObjectMethodV: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, va_list) callconv(.C) jobject,
    CallNonvirtualObjectMethodA: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, [*c]const jvalue) callconv(.C) jobject,
    CallNonvirtualBooleanMethod: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, ...) callconv(.C) jboolean,
    CallNonvirtualBooleanMethodV: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, va_list) callconv(.C) jboolean,
    CallNonvirtualBooleanMethodA: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, [*c]const jvalue) callconv(.C) jboolean,
    CallNonvirtualByteMethod: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, ...) callconv(.C) jbyte,
    CallNonvirtualByteMethodV: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, va_list) callconv(.C) jbyte,
    CallNonvirtualByteMethodA: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, [*c]const jvalue) callconv(.C) jbyte,
    CallNonvirtualCharMethod: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, ...) callconv(.C) jchar,
    CallNonvirtualCharMethodV: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, va_list) callconv(.C) jchar,
    CallNonvirtualCharMethodA: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, [*c]const jvalue) callconv(.C) jchar,
    CallNonvirtualShortMethod: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, ...) callconv(.C) jshort,
    CallNonvirtualShortMethodV: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, va_list) callconv(.C) jshort,
    CallNonvirtualShortMethodA: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, [*c]const jvalue) callconv(.C) jshort,
    CallNonvirtualIntMethod: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, ...) callconv(.C) jint,
    CallNonvirtualIntMethodV: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, va_list) callconv(.C) jint,
    CallNonvirtualIntMethodA: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, [*c]const jvalue) callconv(.C) jint,
    CallNonvirtualLongMethod: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, ...) callconv(.C) jlong,
    CallNonvirtualLongMethodV: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, va_list) callconv(.C) jlong,
    CallNonvirtualLongMethodA: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, [*c]const jvalue) callconv(.C) jlong,
    CallNonvirtualFloatMethod: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, ...) callconv(.C) jfloat,
    CallNonvirtualFloatMethodV: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, va_list) callconv(.C) jfloat,
    CallNonvirtualFloatMethodA: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, [*c]const jvalue) callconv(.C) jfloat,
    CallNonvirtualDoubleMethod: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, ...) callconv(.C) jdouble,
    CallNonvirtualDoubleMethodV: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, va_list) callconv(.C) jdouble,
    CallNonvirtualDoubleMethodA: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, [*c]const jvalue) callconv(.C) jdouble,
    CallNonvirtualVoidMethod: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, ...) callconv(.C) void,
    CallNonvirtualVoidMethodV: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, va_list) callconv(.C) void,
    CallNonvirtualVoidMethodA: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethodID, [*c]const jvalue) callconv(.C) void,
    GetFieldID: ?fn (env: [*c]JNIEnv, jclass, [*c]const u8, [*c]const u8) callconv(.C) jfieldID,
    GetObjectField: ?fn (env: [*c]JNIEnv, jobject, jfieldID) callconv(.C) jobject,
    GetBooleanField: ?fn (env: [*c]JNIEnv, jobject, jfieldID) callconv(.C) jboolean,
    GetByteField: ?fn (env: [*c]JNIEnv, jobject, jfieldID) callconv(.C) jbyte,
    GetCharField: ?fn (env: [*c]JNIEnv, jobject, jfieldID) callconv(.C) jchar,
    GetShortField: ?fn (env: [*c]JNIEnv, jobject, jfieldID) callconv(.C) jshort,
    GetIntField: ?fn (env: [*c]JNIEnv, jobject, jfieldID) callconv(.C) jint,
    GetLongField: ?fn (env: [*c]JNIEnv, jobject, jfieldID) callconv(.C) jlong,
    GetFloatField: ?fn (env: [*c]JNIEnv, jobject, jfieldID) callconv(.C) jfloat,
    GetDoubleField: ?fn (env: [*c]JNIEnv, jobject, jfieldID) callconv(.C) jdouble,
    SetObjectField: ?fn (env: [*c]JNIEnv, jobject, jfieldID, jobject) callconv(.C) void,
    SetBooleanField: ?fn (env: [*c]JNIEnv, jobject, jfieldID, jboolean) callconv(.C) void,
    SetByteField: ?fn (env: [*c]JNIEnv, jobject, jfieldID, jbyte) callconv(.C) void,
    SetCharField: ?fn (env: [*c]JNIEnv, jobject, jfieldID, jchar) callconv(.C) void,
    SetShortField: ?fn (env: [*c]JNIEnv, jobject, jfieldID, jshort) callconv(.C) void,
    SetIntField: ?fn (env: [*c]JNIEnv, jobject, jfieldID, jint) callconv(.C) void,
    SetLongField: ?fn (env: [*c]JNIEnv, jobject, jfieldID, jlong) callconv(.C) void,
    SetFloatField: ?fn (env: [*c]JNIEnv, jobject, jfieldID, jfloat) callconv(.C) void,
    SetDoubleField: ?fn (env: [*c]JNIEnv, jobject, jfieldID, jdouble) callconv(.C) void,
    GetStaticMethodID: ?fn (env: [*c]JNIEnv, jclass, [*c]const u8, [*c]const u8) callconv(.C) jmethodID,
    CallStaticObjectMethod: ?fn (env: [*c]JNIEnv, jclass, jmethodID, ...) callconv(.C) jobject,
    CallStaticObjectMethodV: ?fn (env: [*c]JNIEnv, jclass, jmethodID, va_list) callconv(.C) jobject,
    CallStaticObjectMethodA: ?fn (env: [*c]JNIEnv, jclass, jmethodID, [*c]const jvalue) callconv(.C) jobject,
    CallStaticBooleanMethod: ?fn (env: [*c]JNIEnv, jclass, jmethodID, ...) callconv(.C) jboolean,
    CallStaticBooleanMethodV: ?fn (env: [*c]JNIEnv, jclass, jmethodID, va_list) callconv(.C) jboolean,
    CallStaticBooleanMethodA: ?fn (env: [*c]JNIEnv, jclass, jmethodID, [*c]const jvalue) callconv(.C) jboolean,
    CallStaticByteMethod: ?fn (env: [*c]JNIEnv, jclass, jmethodID, ...) callconv(.C) jbyte,
    CallStaticByteMethodV: ?fn (env: [*c]JNIEnv, jclass, jmethodID, va_list) callconv(.C) jbyte,
    CallStaticByteMethodA: ?fn (env: [*c]JNIEnv, jclass, jmethodID, [*c]const jvalue) callconv(.C) jbyte,
    CallStaticCharMethod: ?fn (env: [*c]JNIEnv, jclass, jmethodID, ...) callconv(.C) jchar,
    CallStaticCharMethodV: ?fn (env: [*c]JNIEnv, jclass, jmethodID, va_list) callconv(.C) jchar,
    CallStaticCharMethodA: ?fn (env: [*c]JNIEnv, jclass, jmethodID, [*c]const jvalue) callconv(.C) jchar,
    CallStaticShortMethod: ?fn (env: [*c]JNIEnv, jclass, jmethodID, ...) callconv(.C) jshort,
    CallStaticShortMethodV: ?fn (env: [*c]JNIEnv, jclass, jmethodID, va_list) callconv(.C) jshort,
    CallStaticShortMethodA: ?fn (env: [*c]JNIEnv, jclass, jmethodID, [*c]const jvalue) callconv(.C) jshort,
    CallStaticIntMethod: ?fn (env: [*c]JNIEnv, jclass, jmethodID, ...) callconv(.C) jint,
    CallStaticIntMethodV: ?fn (env: [*c]JNIEnv, jclass, jmethodID, va_list) callconv(.C) jint,
    CallStaticIntMethodA: ?fn (env: [*c]JNIEnv, jclass, jmethodID, [*c]const jvalue) callconv(.C) jint,
    CallStaticLongMethod: ?fn (env: [*c]JNIEnv, jclass, jmethodID, ...) callconv(.C) jlong,
    CallStaticLongMethodV: ?fn (env: [*c]JNIEnv, jclass, jmethodID, va_list) callconv(.C) jlong,
    CallStaticLongMethodA: ?fn (env: [*c]JNIEnv, jclass, jmethodID, [*c]const jvalue) callconv(.C) jlong,
    CallStaticFloatMethod: ?fn (env: [*c]JNIEnv, jclass, jmethodID, ...) callconv(.C) jfloat,
    CallStaticFloatMethodV: ?fn (env: [*c]JNIEnv, jclass, jmethodID, va_list) callconv(.C) jfloat,
    CallStaticFloatMethodA: ?fn (env: [*c]JNIEnv, jclass, jmethodID, [*c]const jvalue) callconv(.C) jfloat,
    CallStaticDoubleMethod: ?fn (env: [*c]JNIEnv, jclass, jmethodID, ...) callconv(.C) jdouble,
    CallStaticDoubleMethodV: ?fn (env: [*c]JNIEnv, jclass, jmethodID, va_list) callconv(.C) jdouble,
    CallStaticDoubleMethodA: ?fn (env: [*c]JNIEnv, jclass, jmethodID, [*c]const jvalue) callconv(.C) jdouble,
    CallStaticVoidMethod: ?fn (env: [*c]JNIEnv, jclass, jmethodID, ...) callconv(.C) void,
    CallStaticVoidMethodV: ?fn (env: [*c]JNIEnv, jclass, jmethodID, va_list) callconv(.C) void,
    CallStaticVoidMethodA: ?fn (env: [*c]JNIEnv, jclass, jmethodID, [*c]const jvalue) callconv(.C) void,
    GetStaticFieldID: ?fn (env: [*c]JNIEnv, jclass, [*c]const u8, [*c]const u8) callconv(.C) jfieldID,
    GetStaticObjectField: ?fn (env: [*c]JNIEnv, jclass, jfieldID) callconv(.C) jobject,
    GetStaticBooleanField: ?fn (env: [*c]JNIEnv, jclass, jfieldID) callconv(.C) jboolean,
    GetStaticByteField: ?fn (env: [*c]JNIEnv, jclass, jfieldID) callconv(.C) jbyte,
    GetStaticCharField: ?fn (env: [*c]JNIEnv, jclass, jfieldID) callconv(.C) jchar,
    GetStaticShortField: ?fn (env: [*c]JNIEnv, jclass, jfieldID) callconv(.C) jshort,
    GetStaticIntField: ?fn (env: [*c]JNIEnv, jclass, jfieldID) callconv(.C) jint,
    GetStaticLongField: ?fn (env: [*c]JNIEnv, jclass, jfieldID) callconv(.C) jlong,
    GetStaticFloatField: ?fn (env: [*c]JNIEnv, jclass, jfieldID) callconv(.C) jfloat,
    GetStaticDoubleField: ?fn (env: [*c]JNIEnv, jclass, jfieldID) callconv(.C) jdouble,
    SetStaticObjectField: ?fn (env: [*c]JNIEnv, jclass, jfieldID, jobject) callconv(.C) void,
    SetStaticBooleanField: ?fn (env: [*c]JNIEnv, jclass, jfieldID, jboolean) callconv(.C) void,
    SetStaticByteField: ?fn (env: [*c]JNIEnv, jclass, jfieldID, jbyte) callconv(.C) void,
    SetStaticCharField: ?fn (env: [*c]JNIEnv, jclass, jfieldID, jchar) callconv(.C) void,
    SetStaticShortField: ?fn (env: [*c]JNIEnv, jclass, jfieldID, jshort) callconv(.C) void,
    SetStaticIntField: ?fn (env: [*c]JNIEnv, jclass, jfieldID, jint) callconv(.C) void,
    SetStaticLongField: ?fn (env: [*c]JNIEnv, jclass, jfieldID, jlong) callconv(.C) void,
    SetStaticFloatField: ?fn (env: [*c]JNIEnv, jclass, jfieldID, jfloat) callconv(.C) void,
    SetStaticDoubleField: ?fn (env: [*c]JNIEnv, jclass, jfieldID, jdouble) callconv(.C) void,
    NewString: ?fn (env: [*c]JNIEnv, [*c]const jchar, jsize) callconv(.C) jstring,
    GetStringLength: ?fn (env: [*c]JNIEnv, jstring) callconv(.C) jsize,
    GetStringChars: ?fn (env: [*c]JNIEnv, jstring, [*c]jboolean) callconv(.C) [*c]const jchar,
    ReleaseStringChars: ?fn (env: [*c]JNIEnv, jstring, [*c]const jchar) callconv(.C) void,
    NewStringUTF: ?fn (env: [*c]JNIEnv, [*c]const u8) callconv(.C) jstring,
    GetStringUTFLength: ?fn (env: [*c]JNIEnv, jstring) callconv(.C) jsize,
    GetStringUTFChars: ?fn (env: [*c]JNIEnv, jstring, [*c]jboolean) callconv(.C) [*c]const u8,
    ReleaseStringUTFChars: ?fn (env: [*c]JNIEnv, jstring, [*c]const u8) callconv(.C) void,
    GetArrayLength: ?fn (env: [*c]JNIEnv, jarray) callconv(.C) jsize,
    NewObjectArray: ?fn (env: [*c]JNIEnv, jsize, jclass, jobject) callconv(.C) jobjectArray,
    GetObjectArrayElement: ?fn (env: [*c]JNIEnv, jobjectArray, jsize) callconv(.C) jobject,
    SetObjectArrayElement: ?fn (env: [*c]JNIEnv, jobjectArray, jsize, jobject) callconv(.C) void,
    NewBooleanArray: ?fn (env: [*c]JNIEnv, jsize) callconv(.C) jbooleanArray,
    NewByteArray: ?fn (env: [*c]JNIEnv, jsize) callconv(.C) jbyteArray,
    NewCharArray: ?fn (env: [*c]JNIEnv, jsize) callconv(.C) jcharArray,
    NewShortArray: ?fn (env: [*c]JNIEnv, jsize) callconv(.C) jshortArray,
    NewIntArray: ?fn (env: [*c]JNIEnv, jsize) callconv(.C) jintArray,
    NewLongArray: ?fn (env: [*c]JNIEnv, jsize) callconv(.C) jlongArray,
    NewFloatArray: ?fn (env: [*c]JNIEnv, jsize) callconv(.C) jfloatArray,
    NewDoubleArray: ?fn (env: [*c]JNIEnv, jsize) callconv(.C) jdoubleArray,
    GetBooleanArrayElements: ?fn (env: [*c]JNIEnv, jbooleanArray, [*c]jboolean) callconv(.C) [*c]jboolean,
    GetByteArrayElements: ?fn (env: [*c]JNIEnv, jbyteArray, [*c]jboolean) callconv(.C) [*c]jbyte,
    GetCharArrayElements: ?fn (env: [*c]JNIEnv, jcharArray, [*c]jboolean) callconv(.C) [*c]jchar,
    GetShortArrayElements: ?fn (env: [*c]JNIEnv, jshortArray, [*c]jboolean) callconv(.C) [*c]jshort,
    GetIntArrayElements: ?fn (env: [*c]JNIEnv, jintArray, [*c]jboolean) callconv(.C) [*c]jint,
    GetLongArrayElements: ?fn (env: [*c]JNIEnv, jlongArray, [*c]jboolean) callconv(.C) [*c]jlong,
    GetFloatArrayElements: ?fn (env: [*c]JNIEnv, jfloatArray, [*c]jboolean) callconv(.C) [*c]jfloat,
    GetDoubleArrayElements: ?fn (env: [*c]JNIEnv, jdoubleArray, [*c]jboolean) callconv(.C) [*c]jdouble,
    ReleaseBooleanArrayElements: ?fn (env: [*c]JNIEnv, jbooleanArray, [*c]jboolean, jint) callconv(.C) void,
    ReleaseByteArrayElements: ?fn (env: [*c]JNIEnv, jbyteArray, [*c]jbyte, jint) callconv(.C) void,
    ReleaseCharArrayElements: ?fn (env: [*c]JNIEnv, jcharArray, [*c]jchar, jint) callconv(.C) void,
    ReleaseShortArrayElements: ?fn (env: [*c]JNIEnv, jshortArray, [*c]jshort, jint) callconv(.C) void,
    ReleaseIntArrayElements: ?fn (env: [*c]JNIEnv, jintArray, [*c]jint, jint) callconv(.C) void,
    ReleaseLongArrayElements: ?fn (env: [*c]JNIEnv, jlongArray, [*c]jlong, jint) callconv(.C) void,
    ReleaseFloatArrayElements: ?fn (env: [*c]JNIEnv, jfloatArray, [*c]jfloat, jint) callconv(.C) void,
    ReleaseDoubleArrayElements: ?fn (env: [*c]JNIEnv, jdoubleArray, [*c]jdouble, jint) callconv(.C) void,
    GetBooleanArrayRegion: ?fn (env: [*c]JNIEnv, jbooleanArray, jsize, jsize, [*c]jboolean) callconv(.C) void,
    GetByteArrayRegion: ?fn (env: [*c]JNIEnv, jbyteArray, jsize, jsize, [*c]jbyte) callconv(.C) void,
    GetCharArrayRegion: ?fn (env: [*c]JNIEnv, jcharArray, jsize, jsize, [*c]jchar) callconv(.C) void,
    GetShortArrayRegion: ?fn (env: [*c]JNIEnv, jshortArray, jsize, jsize, [*c]jshort) callconv(.C) void,
    GetIntArrayRegion: ?fn (env: [*c]JNIEnv, jintArray, jsize, jsize, [*c]jint) callconv(.C) void,
    GetLongArrayRegion: ?fn (env: [*c]JNIEnv, jlongArray, jsize, jsize, [*c]jlong) callconv(.C) void,
    GetFloatArrayRegion: ?fn (env: [*c]JNIEnv, jfloatArray, jsize, jsize, [*c]jfloat) callconv(.C) void,
    GetDoubleArrayRegion: ?fn (env: [*c]JNIEnv, jdoubleArray, jsize, jsize, [*c]jdouble) callconv(.C) void,
    SetBooleanArrayRegion: ?fn (env: [*c]JNIEnv, jbooleanArray, jsize, jsize, [*c]const jboolean) callconv(.C) void,
    SetByteArrayRegion: ?fn (env: [*c]JNIEnv, jbyteArray, jsize, jsize, [*c]const jbyte) callconv(.C) void,
    SetCharArrayRegion: ?fn (env: [*c]JNIEnv, jcharArray, jsize, jsize, [*c]const jchar) callconv(.C) void,
    SetShortArrayRegion: ?fn (env: [*c]JNIEnv, jshortArray, jsize, jsize, [*c]const jshort) callconv(.C) void,
    SetIntArrayRegion: ?fn (env: [*c]JNIEnv, jintArray, jsize, jsize, [*c]const jint) callconv(.C) void,
    SetLongArrayRegion: ?fn (env: [*c]JNIEnv, jlongArray, jsize, jsize, [*c]const jlong) callconv(.C) void,
    SetFloatArrayRegion: ?fn (env: [*c]JNIEnv, jfloatArray, jsize, jsize, [*c]const jfloat) callconv(.C) void,
    SetDoubleArrayRegion: ?fn (env: [*c]JNIEnv, jdoubleArray, jsize, jsize, [*c]const jdouble) callconv(.C) void,
    RegisterNatives: ?fn (env: [*c]JNIEnv, jclass, [*c]const JNINativeMethod, jint) callconv(.C) jint,
    UnregisterNatives: ?fn (env: [*c]JNIEnv, jclass) callconv(.C) jint,
    MonitorEnter: ?fn (env: [*c]JNIEnv, jobject) callconv(.C) jint,
    MonitorExit: ?fn (env: [*c]JNIEnv, jobject) callconv(.C) jint,
    GetJavaVM: ?fn (env: [*c]JNIEnv, [*c][*c]JavaVM) callconv(.C) jint,
    GetStringRegion: ?fn (env: [*c]JNIEnv, jstring, jsize, jsize, [*c]jchar) callconv(.C) void,
    GetStringUTFRegion: ?fn (env: [*c]JNIEnv, jstring, jsize, jsize, [*c]u8) callconv(.C) void,
    GetPrimitiveArrayCritical: ?fn (env: [*c]JNIEnv, jarray, [*c]jboolean) callconv(.C) ?*anyopaque,
    ReleasePrimitiveArrayCritical: ?fn (env: [*c]JNIEnv, jarray, ?*anyopaque, jint) callconv(.C) void,
    GetStringCritical: ?fn (env: [*c]JNIEnv, jstring, [*c]jboolean) callconv(.C) [*c]const jchar,
    ReleaseStringCritical: ?fn (env: [*c]JNIEnv, jstring, [*c]const jchar) callconv(.C) void,
    NewWeakGlobalRef: ?fn (env: [*c]JNIEnv, jobject) callconv(.C) jweak,
    DeleteWeakGlobalRef: ?fn (env: [*c]JNIEnv, jweak) callconv(.C) void,
    ExceptionCheck: ?fn (env: [*c]JNIEnv) callconv(.C) jboolean,
    NewDirectByteBuffer: ?fn (env: [*c]JNIEnv, ?*anyopaque, jlong) callconv(.C) jobject,
    GetDirectBufferAddress: ?fn (env: [*c]JNIEnv, jobject) callconv(.C) ?*anyopaque,
    GetDirectBufferCapacity: ?fn (env: [*c]JNIEnv, jobject) callconv(.C) jlong,
    GetObjectRefType: ?fn (env: [*c]JNIEnv, jobject) callconv(.C) jobjectRefType,
    GetModule: ?fn (env: [*c]JNIEnv, jclass) callconv(.C) jobject,
};

/// JNI invocation interface
pub const JNIInvokeInterface = extern struct {
    reserved0: ?*anyopaque,
    reserved1: ?*anyopaque,
    reserved2: ?*anyopaque,
    DestroyJavaVM: ?fn ([*c]JavaVM) callconv(.C) jint,
    AttachCurrentThread: ?fn ([*c]JavaVM, [*c]?*anyopaque, ?*anyopaque) callconv(.C) jint,
    DetachCurrentThread: ?fn ([*c]JavaVM) callconv(.C) jint,
    GetEnv: ?fn ([*c]JavaVM, [*c]?*anyopaque, jint) callconv(.C) jint,
    AttachCurrentThreadAsDaemon: ?fn ([*c]JavaVM, [*c]?*anyopaque, ?*anyopaque) callconv(.C) jint,
};

pub const JavaVMAttachArgs = extern struct {
    version: jint, // must be >= JNI_VERSION_1_1
    name: [*c]u8, // NULL or name of thread as modified UTF-8 str
    group: jobject, // global ref of a ThreadGroup object, or NULL
};

/// JNI 1.2+ initialization (As of 1.6, the pre-1.2 structures are no longer supported)
pub const JavaVMOption = extern struct {
    optionString: [*c]u8,
    extraInfo: ?*anyopaque,
};

pub const JavaVMInitArgs = extern struct {
    version: jint, // use JNI_VERSION_1_1 or later
    nOptions: jint,
    options: [*c]JavaVMOption,
    ignoreUnrecognized: jboolean,
};

// /*
// * VM initialization functions
// *
// * Note these are the only symbols exported for JNI by the VM
// */
pub extern fn JNI_GetDefaultJavaVMInitArgs(args: ?*anyopaque) jint;
pub extern fn JNI_CreateJavaVM(pvm: [*c][*c]JavaVM, penv: [*c]?*anyopaque, args: ?*anyopaque) jint;
pub extern fn JNI_GetCreatedJavaVMs([*c][*c]JavaVM, jsize, [*c]jsize) jint;

// /*
//  * Manifest constants
//  */
pub const JNI_FALSE = @as(jboolean, 0);
pub const JNI_TRUE = @as(jboolean, 1);

pub const JNI_VERSION_1_1 = @as(jint, 0x00010001);
pub const JNI_VERSION_1_2 = @as(jint, 0x00010002);
pub const JNI_VERSION_1_4 = @as(jint, 0x00010004);
pub const JNI_VERSION_1_6 = @as(jint, 0x00010006);
pub const JNI_VERSION_1_8 = @as(jint, 0x00010008);
pub const JNI_VERSION_9 = @as(jint, 0x00090000);
pub const JNI_VERSION_10 = @as(jint, 0x000a0000);

// /*
//  * possible return values for JNI functions
//  */

/// no error
pub const JNI_OK = @as(jint, 0);
/// generic error
pub const JNI_ERR = @as(jint, -1);
/// thread detached from the VM
pub const JNI_EDETACHED = @as(jint, -2);
/// JNI version error
pub const JNI_EVERSION = @as(jint, -3);
/// Out of memory
pub const JNI_ENOMEM = @as(jint, -4);
/// VM already created
pub const JNI_EEXIST = @as(jint, -5);
/// Invalid argument
pub const JNI_EINVAL = @as(jint, -6);

// /*
//  * used in ReleaseScalarArrayElements
//  */

/// copy content, do not free buffer
pub const JNI_COMMIT = @as(jint, 1);
/// free buffer w/o copying back
pub const JNI_ABORT = @as(jint, 2);
