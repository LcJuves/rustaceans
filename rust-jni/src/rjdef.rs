#![allow(non_snake_case, non_camel_case_types, dead_code)]

use std::os::raw::{c_char, c_void};

pub type va_list = *mut c_void;

/* Primitive types that match up with Java equivalents */
pub type jboolean = u8; /* unsigned 8 bits */
pub type jbyte = i8; /* signed 8 bits */
pub type jchar = u16; /* unsigned 16 bits */
pub type jshort = i16; /* signed 16 bits */
pub type jint = i32; /* signed 32 bits */
pub type jlong = i64; /* signed 64 bits */
pub type jfloat = f32; /* 32-bit IEEE 754 */
pub type jdouble = f64; /* 64-bit IEEE 754 */

/// "cardinal indices and sizes"
pub type jsize = jint;

/*
 * Reference types, in C
 */
pub type jobject = *mut c_void;
pub type jclass = jobject;
pub type jstring = jobject;
pub type jarray = jobject;
pub type jobjectArray = jarray;
pub type jbooleanArray = jarray;
pub type jbyteArray = jarray;
pub type jcharArray = jarray;
pub type jshortArray = jarray;
pub type jintArray = jarray;
pub type jlongArray = jarray;
pub type jfloatArray = jarray;
pub type jdoubleArray = jarray;
pub type jthrowable = jobject;
pub type jweak = jobject;

/// opaque structure
#[repr(C)]
pub struct _jfieldID;
/// field IDs
pub type jfieldID = *mut _jfieldID;

/// opaque structure
#[repr(C)]
pub struct _jmethodID;
/// method IDs
pub type jmethodID = *mut _jmethodID;

#[repr(C)]
pub union jvalue {
    pub z: jboolean,
    pub b: jbyte,
    pub c: jchar,
    pub s: jshort,
    pub i: jshort,
    pub j: jlong,
    pub f: jfloat,
    pub d: jdouble,
    pub l: jobject,
}

#[repr(C)]
pub enum jobjectRefType {
    JNIInvalidRefType = 0,
    JNILocalRefType = 1,
    JNIGlobalRefType = 2,
    JNIWeakGlobalRefType = 3,
}

#[repr(C)]
pub struct JNINativeMethod {
    pub name: *const c_char,
    pub signature: *const c_char,
    pub fnPtr: *mut c_void,
}

pub type JNIEnv = *const JNINativeInterface;
pub type JavaVM = *const JNIInvokeInterface;

/// Table of interface function pointers
#[repr(C)]
pub struct JNINativeInterface {
    reserved0: *mut c_void,
    reserved1: *mut c_void,
    reserved2: *mut c_void,
    reserved3: *mut c_void,

    GetVersion: unsafe extern "system" fn(env: *mut JNIEnv) -> jint,

    DefineClass: unsafe extern "system" fn(
        env: *mut JNIEnv,
        name: *const c_char,
        loader: jobject,
        buf: *const jbyte,
        len: jsize,
    ) -> jclass,
    FindClass: unsafe extern "system" fn(env: *mut JNIEnv, name: *const c_char) -> jclass,

    FromReflectedMethod: unsafe extern "system" fn(env: *mut JNIEnv, method: jobject) -> jmethodID,
    FromReflectedField: unsafe extern "system" fn(env: *mut JNIEnv, field: jobject) -> jfieldID,

    ToReflectedMethod: unsafe extern "system" fn(
        env: *mut JNIEnv,
        cls: jclass,
        methodID: jmethodID,
        isStatic: jboolean,
    ) -> jobject,

    GetSuperclass: unsafe extern "system" fn(env: *mut JNIEnv, sub: jclass) -> jclass,
    IsAssignableFrom:
        unsafe extern "system" fn(env: *mut JNIEnv, sub: jclass, sup: jclass) -> jboolean,

    ToReflectedField: unsafe extern "system" fn(
        env: *mut JNIEnv,
        cls: jclass,
        fieldID: jfieldID,
        isStatic: jboolean,
    ) -> jobject,

    Throw: unsafe extern "system" fn(env: *mut JNIEnv, obj: jthrowable) -> jint,
    ThrowNew:
        unsafe extern "system" fn(env: *mut JNIEnv, clazz: jclass, msg: *const c_char) -> jint,
    ExceptionOccurred: unsafe extern "system" fn(env: *mut JNIEnv) -> jthrowable,
    ExceptionDescribe: unsafe extern "system" fn(env: *mut JNIEnv) -> !,
    ExceptionClear: unsafe extern "system" fn(env: *mut JNIEnv) -> !,
    FatalError: unsafe extern "system" fn(env: *mut JNIEnv, msg: *const c_char) -> !,

    PushLocalFrame: unsafe extern "system" fn(env: *mut JNIEnv, capacity: jint) -> jint,
    PopLocalFrame: unsafe extern "system" fn(env: *mut JNIEnv, result: jobject) -> jobject,

    NewGlobalRef: unsafe extern "system" fn(env: *mut JNIEnv, gref: jobject) -> jobject,
    DeleteGlobalRef: unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject) -> !,
    DeleteLocalRef: unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject) -> !,
    IsSameObject:
        unsafe extern "system" fn(env: *mut JNIEnv, obj1: jobject, obj2: jobject) -> jboolean,
    NewLocalRef: unsafe extern "system" fn(env: *mut JNIEnv, lref: jobject) -> jobject,
    EnsureLocalCapacity: unsafe extern "system" fn(env: *mut JNIEnv, capacity: jint) -> jint,

    AllocObject: unsafe extern "system" fn(env: *mut JNIEnv, clazz: jclass) -> jobject,
    NewObject:
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jobject,
    NewObjectV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jobject,
    NewObjectA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jobject,

    GetObjectClass: unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject) -> jclass,
    IsInstanceOf:
        unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject, clazz: jclass) -> jboolean,

    GetMethodID: unsafe extern "system" fn(
        env: *mut JNIEnv,
        name: *const c_char,
        sig: *const c_char,
    ) -> jmethodID,

    CallObjectMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jobject,
    CallObjectMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list,
    ) -> jobject,
    CallObjectMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jobject,

    CallBooleanMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jboolean,
    CallBooleanMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list,
    ) -> jboolean,
    CallBooleanMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jboolean,

    CallByteMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jbyte,
    CallByteMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list,
    ) -> jbyte,
    CallByteMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jbyte,

    CallCharMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jchar,
    CallCharMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list,
    ) -> jchar,
    CallCharMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jchar,

    CallShortMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jshort,
    CallShortMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list,
    ) -> jshort,
    CallShortMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jshort,

    CallIntMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jint,
    CallIntMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list,
    ) -> jint,
    CallIntMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jint,

    CallLongMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jlong,
    CallLongMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list,
    ) -> jlong,
    CallLongMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jlong,

    CallFloatMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jfloat,
    CallFloatMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list,
    ) -> jfloat,
    CallFloatMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jfloat,

    CallDoubleMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jdouble,
    CallDoubleMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list,
    ) -> jdouble,
    CallDoubleMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jdouble,

    CallVoidMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> !,
    CallVoidMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list,
    ) -> !,
    CallVoidMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> !,

    CallNonvirtualObjectMethod: unsafe extern "C" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        ...
    ) -> jobject,
    CallNonvirtualObjectMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jobject,
    CallNonvirtualObjectMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jobject,

    CallNonvirtualBooleanMethod: unsafe extern "C" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        ...
    ) -> jboolean,
    CallNonvirtualBooleanMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jboolean,
    CallNonvirtualBooleanMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jboolean,

    CallNonvirtualByteMethod: unsafe extern "C" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        ...
    ) -> jbyte,
    CallNonvirtualByteMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jbyte,
    CallNonvirtualByteMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jbyte,

    CallNonvirtualCharMethod: unsafe extern "C" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        ...
    ) -> jchar,
    CallNonvirtualCharMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jchar,
    CallNonvirtualCharMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jchar,

    CallNonvirtualShortMethod: unsafe extern "C" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        ...
    ) -> jshort,
    CallNonvirtualShortMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jshort,
    CallNonvirtualShortMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jshort,
}

/// JNI invocation interface
#[repr(C)]
pub struct JNIInvokeInterface {
    reserved0: *mut c_void,
    reserved1: *mut c_void,
    reserved2: *mut c_void,

    DestroyJavaVM: unsafe extern "system" fn(vm: *mut JavaVM) -> jint,

    AttachCurrentThread: unsafe extern "system" fn(
        vm: *mut JavaVM,
        penv: *mut *mut c_void,
        args: *mut c_void,
    ) -> jint,

    DetachCurrentThread: unsafe extern "system" fn(vm: *mut JavaVM) -> jint,

    GetEnv:
        unsafe extern "system" fn(vm: *mut JavaVM, penv: *mut *mut c_void, version: jint) -> jint,

    AttachCurrentThreadAsDaemon: unsafe extern "system" fn(
        vm: *mut JavaVM,
        penv: *mut *mut c_void,
        args: *mut c_void,
    ) -> jint,
}

#[repr(C)]
pub struct JavaVMAttachArgs {
    version: jint,       /* must be >= JNI_VERSION_1_1 */
    name: *const c_char, /* NULL or name of thread as modified UTF-8 str */
    group: jobject,      /* global ref of a ThreadGroup object, or NULL */
}

/// JNI 1.2+ initialization (As of 1.6, the pre-1.2 structures are no longer supported)
#[repr(C)]
pub struct JavaVMOption {
    optionString: *const c_char,
    extraInfo: *mut c_void,
}

#[repr(C)]
pub struct JavaVMInitArgs {
    version: jint, /* use JNI_VERSION_1_1 or later */

    nOptions: jint,
    options: *mut JavaVMOption,
    ignoreUnrecognized: jboolean,
}

extern "system" {
    /*
     * VM initialization functions
     *
     * Note these are the only symbols exported for JNI by the VM
     */
    pub fn JNI_GetDefaultJavaVMInitArgs(args: *mut c_void) -> jint;
    pub fn JNI_CreateJavaVM(
        pvm: *mut *mut JavaVM,
        penv: *mut *mut c_void,
        args: *mut c_void,
    ) -> jint;
    pub fn JNI_GetCreatedJavaVMs(
        vm_buf: *mut *mut JavaVM,
        bufLen: jsize,
        numVMs: *mut jsize,
    ) -> jint;
}

// /*
//  * Prototypes for functions exported by loadable shared libs
//  * These are called by JNI, not provided by JNI
//  */
// JNIEXPORT jint JNI_OnLoad(JavaVM* vm, void* reserved);
// JNIEXPORT void JNI_OnUnload(JavaVM* vm, void* reserved);

/*
 * Manifest constants
 */
pub const JNI_FALSE: jboolean = 0;
pub const JNI_TRUE: jboolean = 1;

pub const JNI_VERSION_1_1: jint = 0x00010001;
pub const JNI_VERSION_1_2: jint = 0x00010002;
pub const JNI_VERSION_1_4: jint = 0x00010004;
pub const JNI_VERSION_1_6: jint = 0x00010006;
pub const JNI_VERSION_1_8: jint = 0x00010008;
pub const JNI_VERSION_9: jint = 0x00090000;
pub const JNI_VERSION_10: jint = 0x000a0000;

pub const JNI_OK: jint = 0; /* no error */
pub const JNI_ERR: jint = -1; /* generic error */
pub const JNI_EDETACHED: jint = -2; /* thread detached from the VM */
pub const JNI_EVERSION: jint = -3; /* JNI version error */
pub const JNI_ENOMEM: jint = -4; /* Out of memory */
pub const JNI_EEXIST: jint = -5; /* VM already created */
pub const JNI_EINVAL: jint = -6; /* Invalid argument */

pub const JNI_COMMIT: jint = 1; /* copy content, do not free buffer */
pub const JNI_ABORT: jint = 2; /* free buffer w/o copying back */
