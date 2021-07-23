#![allow(non_snake_case, non_camel_case_types, dead_code)]

use std::os::raw::{c_char, c_void};

/* Primitive types that match up with Java equivalents */
pub type jboolean = u8; /* unsigned 8 bits */
pub type jbyte = i8; /* signed 8 bits */
pub type jchar = u16; /* unsigned 16 bits */
pub type jshort = i16; /* signed 16 bits */
pub type jint = i32; /* signed 32 bits */
pub type jlong = i64; /* signed 64 bits */
pub type jfloat = f32; /* 32-bit IEEE 754 */
pub type jdouble = f64; /* 64-bit IEEE 754 */

/* "cardinal indices and sizes" */
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

#[repr(C)]
pub struct _jfieldID; /* opaque structure */
pub type jfieldID = *mut _jfieldID; /* field IDs */

#[repr(C)]
pub struct _jmethodID; /* opaque structure */
pub type jmethodID = *mut _jmethodID; /* method IDs */

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

/*
 * Table of interface function pointers
 */
#[repr(C)]
pub struct JNINativeInterface {
    reserved0: *mut c_void,
    reserved1: *mut c_void,
    reserved2: *mut c_void,
    reserved3: *mut c_void,
}

/*
 * JNI invocation interface
 */
#[repr(C)]
pub struct JNIInvokeInterface {
    reserved0: *mut c_void,
    reserved1: *mut c_void,
    reserved2: *mut c_void,
}

#[repr(C)]
pub struct JavaVMAttachArgs {
    version: jint,       /* must be >= JNI_VERSION_1_1 */
    name: *const c_char, /* NULL or name of thread as modified UTF-8 str */
    group: jobject,      /* global ref of a ThreadGroup object, or NULL */
}

/*
 * JNI 1.2+ initialization.  (As of 1.6, the pre-1.2 structures are no
 * longer supported.)
 */
#[repr(C)]
pub struct JavaVMOption {
    optionString: *const c_char,
    extraInfo: *mut c_void,
}

#[repr(C)]
pub struct JavaVMInitArgs {
    version: jint, /* use JNI_VERSION_1_2 or later */

    nOptions: jint,
    options: *mut JavaVMOption,
    ignoreUnrecognized: jboolean,
}

extern "system" {}

// /*
//  * VM initialization functions.
//  *
//  * Note these are the only symbols exported for JNI by the VM.
//  */
// jint JNI_GetDefaultJavaVMInitArgs(void*);
// jint JNI_CreateJavaVM(JavaVM**, JNIEnv**, void*);
// jint JNI_GetCreatedJavaVMs(JavaVM**, jsize, jsize*);

// /*
//  * Prototypes for functions exported by loadable shared libs.  These are
//  * called by JNI, not provided by JNI.
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

pub const JNI_OK: jint = 0; /* no error */
pub const JNI_ERR: jint = -1; /* generic error */
pub const JNI_EDETACHED: jint = -2; /* thread detached from the VM */
pub const JNI_EVERSION: jint = -3; /* JNI version error */
pub const JNI_ENOMEM: jint = -4; /* Out of memory */
pub const JNI_EEXIST: jint = -5; /* VM already created */
pub const JNI_EINVAL: jint = -6; /* Invalid argument */

pub const JNI_COMMIT: jint = 1; /* copy content, do not free buffer */
pub const JNI_ABORT: jint = 2; /* free buffer w/o copying back */
