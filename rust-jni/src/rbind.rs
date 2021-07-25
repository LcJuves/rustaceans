#![allow(non_snake_case, non_camel_case_types, dead_code)]

//! Rust bindings to the JNI
//! ******
//! Prototypes for functions exported by loadable shared libs
//! 
//! These are called by JNI, not provided by JNI

/**
 * Copyright (c) 2021 Liangcheng Juves <liangchengj@outlook.com>
 *
 * Permission is hereby granted, free of charge, to any
 * person obtaining a copy of this software and associated
 * documentation files (the "Software"), to deal in the
 * Software without restriction, including without
 * limitation the rights to use, copy, modify, merge,
 * publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software
 * is furnished to do so, subject to the following
 * conditions:
 *
 * The above copyright notice and this permission notice
 * shall be included in all copies or substantial portions
 * of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
 * ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
 * TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
 * PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
 * SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
 * CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
 * OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
 * IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

/// # Examples
///
/// ```rust
/// unsafe_extern_system_fn!((env: *mut JNIEnv) -> jint)
/// ```
/// expand to
/// ```rust
/// Option<unsafe extern "system" fn(env: *mut JNIEnv) -> jint>
/// ```
#[macro_export]
macro_rules! unsafe_extern_system_fn {
    (($($param_name:tt: $param_type:ty), *) -> $ret_ty:ty) => {
        Option<unsafe extern "system" fn($($param_name: $param_type, )*) -> $ret_ty>
    }
}

#[macro_export]
macro_rules! unsafe_extern_c_var_fn {
    (($($param_name:tt: $param_type:ty), *) -> $ret_ty:ty) => {
        Option<unsafe extern "C" fn($($param_name: $param_type, )* ...) -> $ret_ty>
    }
}

#[macro_export]
macro_rules! jni_fn_def {
    ($name:tt, ($($ident:tt: $ty:ty), *), $ret_ty:ty, $code:block) => {
        #[no_mangle]
        pub extern "system" fn $name($($ident: $ty, )*) -> $ret_ty $code
    };
}

#[macro_export]
macro_rules! unsafe_jni_fn_def {
    ($name:tt, ($($ident:tt: $ty:ty), *), $ret_ty:ty, $code:block) => {
        #[no_mangle]
        pub unsafe extern "system" fn $name($($ident: $ty, )*) -> $ret_ty $code
    };
}

/// Defined by native libraries
///
/// ##  JNI_OnLoad
/// ***
/// ```rust
/// #[no_mangle]
/// pub unsafe extern "system" fn JNI_OnLoad(vm: *mut JavaVM, reserved: *mut c_void) -> jint {
///     /* code */
///     /* The return value must be >= JNI_VERSION_1_1 */
/// }
/// ```
/// ### Or use the `impl_jni_on_load` macro, like this
/// ``` rust
/// impl_jni_on_load!(vm, reserved, {
///     /* code */
///     /* The return value must be >= JNI_VERSION_1_1 */
/// });
/// ```
#[macro_export]
macro_rules! impl_jni_on_load {
    ($param_vm_name:tt, $param_reserved_name:tt, $code:block) => {
        jni_fn_def!(
            JNI_OnLoad,
            (
                $param_vm_name: *mut JavaVM,
                $param_reserved_name: *mut c_void
            ),
            jint,
            $code
        );
    };
}

/// Defined by native libraries
///
/// ## JNI_OnUnload
/// ***
/// ```rust
/// #[no_mangle]
/// pub unsafe extern "system" fn JNI_OnUnload(vm: *mut JavaVM, reserved: *mut c_void) {
///     /* code */
/// }
/// ```
/// ### Or use the `impl_jni_on_unload` macro, like this
/// ```rust
/// impl_jni_on_unload!(vm, reserved, { /* code */ });
/// ```
#[macro_export]
macro_rules! impl_jni_on_unload {
    ($param_vm_name:tt, $param_reserved_name:tt, $code:block) => {
        jni_fn_def!(
            JNI_OnUnload,
            (
                $param_vm_name: *mut JavaVM,
                $param_reserved_name: *mut c_void
            ),
            (),
            $code
        );
    };
}

use std::ffi::c_void;
use std::os::raw::c_char;

pub type va_list = *mut c_void;

/* Primitive types that match up with Java equivalents */

/// unsigned 8 bits
pub type jboolean = u8;
/// signed 8 bits
pub type jbyte = i8;
/// unsigned 16 bits
pub type jchar = u16;
/// signed 16 bits
pub type jshort = i16;
/// signed 32 bits
pub type jint = i32;
/// signed 64 bits
pub type jlong = i64;
/// 32-bit IEEE 754
pub type jfloat = f32;
/// 64-bit IEEE 754
pub type jdouble = f64;

/// "cardinal indices and sizes"
pub type jsize = jint;

/* Reference types that match up with Java equivalents */

pub type jobject = *mut c_void;
pub type jclass = jobject;
pub type jthrowable = jobject;
pub type jstring = jobject;

/* Array types that match up with Java equivalents */

pub type jarray = jobject;
pub type jbooleanArray = jarray;
pub type jbyteArray = jarray;
pub type jcharArray = jarray;
pub type jshortArray = jarray;
pub type jintArray = jarray;
pub type jlongArray = jarray;
pub type jfloatArray = jarray;
pub type jdoubleArray = jarray;
pub type jobjectArray = jarray;

pub type jweak = jobject;

/// When passing arguments from Rust to a Java method, the jvalue union is used
#[repr(C)]
pub union jvalue {
    // Primitive types
    pub z: jboolean,
    pub b: jbyte,
    pub c: jchar,
    pub s: jshort,
    pub i: jint,
    pub j: jlong,
    pub f: jfloat,
    pub d: jdouble,
    // Reference types
    pub l: jobject,
}

/// field IDs
pub type jfieldID = *mut c_void;

/// method IDs
pub type jmethodID = *mut c_void;

/// Return values from jobjectRefType
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

/// `JNIEnv` implements the "Java Native Inferface", and contains most of what you'll use to interact with Java from Rust
pub type JNIEnv = *const JNINativeInterface;
/// `JavaVM` (along with a handful of global functions) implements the "Java Invocation Interface",
/// which allow you to create and destroy a Java Virtual Machine
pub type JavaVM = *const JNIInvokeInterface;

/// Table of interface function pointers
#[repr(C)]
pub struct JNINativeInterface {
    reserved0: *mut c_void,
    reserved1: *mut c_void,
    reserved2: *mut c_void,

    reserved3: *mut c_void,
    pub GetVersion: unsafe extern "system" fn(env: *mut JNIEnv) -> jint,

    pub DefineClass: unsafe extern "system" fn(
        env: *mut JNIEnv,
        name: *const c_char,
        loader: jobject,
        buf: *const jbyte,
        len: jsize,
    ) -> jclass,
    pub FindClass: unsafe extern "system" fn(env: *mut JNIEnv, name: *const c_char) -> jclass,

    pub FromReflectedMethod:
        unsafe extern "system" fn(env: *mut JNIEnv, method: jobject) -> jmethodID,
    pub FromReflectedField: unsafe extern "system" fn(env: *mut JNIEnv, field: jobject) -> jfieldID,

    /// spec doesn't show jboolean parameter
    pub ToReflectedMethod: unsafe extern "system" fn(
        env: *mut JNIEnv,
        cls: jclass,
        methodID: jmethodID,
        isStatic: jboolean,
    ) -> jobject,

    pub GetSuperclass: unsafe extern "system" fn(env: *mut JNIEnv, sub: jclass) -> jclass,
    pub IsAssignableFrom:
        unsafe extern "system" fn(env: *mut JNIEnv, sub: jclass, sup: jclass) -> jboolean,

    /// spec doesn't show jboolean parameter
    pub ToReflectedField: unsafe extern "system" fn(
        env: *mut JNIEnv,
        cls: jclass,
        fieldID: jfieldID,
        isStatic: jboolean,
    ) -> jobject,

    pub Throw: unsafe extern "system" fn(env: *mut JNIEnv, obj: jthrowable) -> jint,
    pub ThrowNew:
        unsafe extern "system" fn(env: *mut JNIEnv, clazz: jclass, msg: *const c_char) -> jint,
    pub ExceptionOccurred: unsafe extern "system" fn(env: *mut JNIEnv) -> jthrowable,
    pub ExceptionDescribe: unsafe extern "system" fn(env: *mut JNIEnv) -> !,
    pub ExceptionClear: unsafe extern "system" fn(env: *mut JNIEnv) -> !,
    pub FatalError: unsafe extern "system" fn(env: *mut JNIEnv, msg: *const c_char) -> !,

    pub PushLocalFrame: unsafe extern "system" fn(env: *mut JNIEnv, capacity: jint) -> jint,
    pub PopLocalFrame: unsafe extern "system" fn(env: *mut JNIEnv, result: jobject) -> jobject,

    pub NewGlobalRef: unsafe extern "system" fn(env: *mut JNIEnv, gref: jobject) -> jobject,
    pub DeleteGlobalRef: unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject) -> !,
    pub DeleteLocalRef: unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject) -> !,
    pub IsSameObject:
        unsafe extern "system" fn(env: *mut JNIEnv, obj1: jobject, obj2: jobject) -> jboolean,
    pub NewLocalRef: unsafe extern "system" fn(env: *mut JNIEnv, lref: jobject) -> jobject,
    pub EnsureLocalCapacity: unsafe extern "system" fn(env: *mut JNIEnv, capacity: jint) -> jint,

    pub AllocObject: unsafe extern "system" fn(env: *mut JNIEnv, clazz: jclass) -> jobject,
    pub NewObject:
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jobject,
    pub NewObjectV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jobject,
    pub NewObjectA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jobject,

    pub GetObjectClass: unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject) -> jclass,
    pub IsInstanceOf:
        unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject, clazz: jclass) -> jboolean,

    pub GetMethodID: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        name: *const c_char,
        sig: *const c_char,
    ) -> jmethodID,

    pub CallObjectMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jobject,
    pub CallObjectMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list,
    ) -> jobject,
    pub CallObjectMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jobject,

    pub CallBooleanMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jboolean,
    pub CallBooleanMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list,
    ) -> jboolean,
    pub CallBooleanMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jboolean,

    pub CallByteMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jbyte,
    pub CallByteMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list,
    ) -> jbyte,
    pub CallByteMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jbyte,

    pub CallCharMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jchar,
    pub CallCharMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list,
    ) -> jchar,
    pub CallCharMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jchar,

    pub CallShortMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jshort,
    pub CallShortMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list,
    ) -> jshort,
    pub CallShortMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jshort,

    pub CallIntMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jint,
    pub CallIntMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list,
    ) -> jint,
    pub CallIntMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jint,

    pub CallLongMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jlong,
    pub CallLongMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list,
    ) -> jlong,
    pub CallLongMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jlong,

    pub CallFloatMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jfloat,
    pub CallFloatMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list,
    ) -> jfloat,
    pub CallFloatMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jfloat,

    pub CallDoubleMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jdouble,
    pub CallDoubleMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list,
    ) -> jdouble,
    pub CallDoubleMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jdouble,

    pub CallVoidMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> !,
    pub CallVoidMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list,
    ) -> !,
    pub CallVoidMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> !,

    pub CallNonvirtualObjectMethod: unsafe extern "C" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        ...
    ) -> jobject,
    pub CallNonvirtualObjectMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jobject,
    pub CallNonvirtualObjectMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jobject,

    pub CallNonvirtualBooleanMethod: unsafe extern "C" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        ...
    ) -> jboolean,
    pub CallNonvirtualBooleanMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jboolean,
    pub CallNonvirtualBooleanMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jboolean,

    pub CallNonvirtualByteMethod: unsafe extern "C" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        ...
    ) -> jbyte,
    pub CallNonvirtualByteMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jbyte,
    pub CallNonvirtualByteMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jbyte,

    pub CallNonvirtualCharMethod: unsafe extern "C" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        ...
    ) -> jchar,
    pub CallNonvirtualCharMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jchar,
    pub CallNonvirtualCharMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jchar,

    pub CallNonvirtualShortMethod: unsafe extern "C" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        ...
    ) -> jshort,
    pub CallNonvirtualShortMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jshort,
    pub CallNonvirtualShortMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jshort,

    pub CallNonvirtualIntMethod: unsafe extern "C" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        ...
    ) -> jint,
    pub CallNonvirtualIntMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jint,
    pub CallNonvirtualIntMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jint,

    pub CallNonvirtualLongMethod: unsafe extern "C" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        ...
    ) -> jlong,
    pub CallNonvirtualLongMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jlong,
    pub CallNonvirtualLongMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jlong,

    pub CallNonvirtualFloatMethod: unsafe extern "C" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        ...
    ) -> jfloat,
    pub CallNonvirtualFloatMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jfloat,
    pub CallNonvirtualFloatMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jfloat,

    pub CallNonvirtualDoubleMethod: unsafe extern "C" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        ...
    ) -> jdouble,
    pub CallNonvirtualDoubleMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jdouble,
    pub CallNonvirtualDoubleMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jdouble,

    pub CallNonvirtualVoidMethod: unsafe extern "C" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        ...
    ) -> !,
    pub CallNonvirtualVoidMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> !,
    pub CallNonvirtualVoidMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> !,

    pub GetFieldID: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        name: *const c_char,
        sig: *const c_char,
    ) -> jfieldID,

    pub GetObjectField:
        unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jobject,
    pub GetBooleanField:
        unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jboolean,
    pub GetByteField:
        unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jbyte,
    pub GetCharField:
        unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jchar,
    pub GetShortField:
        unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jshort,
    pub GetIntField:
        unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jint,
    pub GetLongField:
        unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jlong,
    pub GetFloatField:
        unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jfloat,
    pub GetDoubleField:
        unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jdouble,

    pub SetObjectField: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        fieldID: jfieldID,
        val: jobject,
    ) -> !,
    pub SetBooleanField: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        fieldID: jfieldID,
        val: jboolean,
    ) -> !,
    pub SetByteField: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        fieldID: jfieldID,
        val: jbyte,
    ) -> !,
    pub SetCharField: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        fieldID: jfieldID,
        val: jchar,
    ) -> !,
    pub SetShortField: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        fieldID: jfieldID,
        val: jshort,
    ) -> !,
    pub SetIntField: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        fieldID: jfieldID,
        val: jint,
    ) -> !,
    pub SetLongField: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        fieldID: jfieldID,
        val: jlong,
    ) -> !,
    pub SetFloatField: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        fieldID: jfieldID,
        val: jfloat,
    ) -> !,
    pub SetDoubleField: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        fieldID: jfieldID,
        val: jdouble,
    ) -> !,

    pub GetStaticMethodID: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        name: *const c_char,
        sig: *const c_char,
    ) -> jmethodID,

    pub CallStaticObjectMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jobject,
    pub CallStaticObjectMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jobject,
    pub CallStaticObjectMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jobject,

    pub CallStaticBooleanMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jboolean,
    pub CallStaticBooleanMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jboolean,
    pub CallStaticBooleanMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jboolean,

    pub CallStaticByteMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jbyte,
    pub CallStaticByteMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jbyte,
    pub CallStaticByteMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jbyte,

    pub CallStaticCharMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jchar,
    pub CallStaticCharMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jchar,
    pub CallStaticCharMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jchar,

    pub CallStaticShortMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jshort,
    pub CallStaticShortMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jshort,
    pub CallStaticShortMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jshort,

    pub CallStaticIntMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jint,
    pub CallStaticIntMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jint,
    pub CallStaticIntMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jint,

    pub CallStaticLongMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jlong,
    pub CallStaticLongMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jlong,
    pub CallStaticLongMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jlong,

    pub CallStaticFloatMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jfloat,
    pub CallStaticFloatMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jfloat,
    pub CallStaticFloatMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jfloat,

    pub CallStaticDoubleMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jdouble,
    pub CallStaticDoubleMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> jdouble,
    pub CallStaticDoubleMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> jdouble,

    pub CallStaticVoidMethod:
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> !,
    pub CallStaticVoidMethodV: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list,
    ) -> !,
    pub CallStaticVoidMethodA: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> !,

    pub GetStaticFieldID: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        name: *const c_char,
        sig: *const c_char,
    ) -> jfieldID,
    pub GetStaticObjectField:
        unsafe extern "system" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jobject,
    pub GetStaticBooleanField:
        unsafe extern "system" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jboolean,
    pub GetStaticByteField:
        unsafe extern "system" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jbyte,
    pub GetStaticCharField:
        unsafe extern "system" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jchar,
    pub GetStaticShortField:
        unsafe extern "system" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jshort,
    pub GetStaticIntField:
        unsafe extern "system" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jint,
    pub GetStaticLongField:
        unsafe extern "system" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jlong,
    pub GetStaticFloatField:
        unsafe extern "system" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jfloat,
    pub GetStaticDoubleField:
        unsafe extern "system" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jdouble,

    pub SetStaticObjectField: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        fieldID: jfieldID,
        value: jobject,
    ) -> !,
    pub SetStaticBooleanField: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        fieldID: jfieldID,
        value: jboolean,
    ) -> !,
    pub SetStaticByteField: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        fieldID: jfieldID,
        value: jbyte,
    ) -> !,
    pub SetStaticCharField: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        fieldID: jfieldID,
        value: jchar,
    ) -> !,
    pub SetStaticShortField: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        fieldID: jfieldID,
        value: jshort,
    ) -> !,
    pub SetStaticIntField: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        fieldID: jfieldID,
        value: jint,
    ) -> !,
    pub SetStaticLongField: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        fieldID: jfieldID,
        value: jlong,
    ) -> !,
    pub SetStaticFloatField: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        fieldID: jfieldID,
        value: jfloat,
    ) -> !,
    pub SetStaticDoubleField: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        fieldID: jfieldID,
        value: jdouble,
    ) -> !,

    pub NewString:
        unsafe extern "system" fn(env: *mut JNIEnv, unicode: *const jchar, len: jsize) -> jstring,
    pub GetStringLength: unsafe extern "system" fn(env: *mut JNIEnv, str: jstring) -> jsize,
    pub GetStringChars: unsafe extern "system" fn(
        env: *mut JNIEnv,
        str: jstring,
        isCopy: *mut jboolean,
    ) -> *const jchar,
    pub ReleaseStringChars:
        unsafe extern "system" fn(env: *mut JNIEnv, str: jstring, chars: *const jchar) -> !,

    pub NewStringUTF: unsafe extern "system" fn(env: *mut JNIEnv, utf: *const c_char) -> jstring,
    pub GetStringUTFLength: unsafe extern "system" fn(env: *mut JNIEnv, str: jstring) -> jsize,
    pub GetStringUTFChars: unsafe extern "system" fn(
        env: *mut JNIEnv,
        str: jstring,
        isCopy: *mut jboolean,
    ) -> *const c_char,
    pub ReleaseStringUTFChars:
        unsafe extern "system" fn(env: *mut JNIEnv, str: jstring, chars: *const c_char) -> !,

    pub GetArrayLength: unsafe extern "system" fn(env: *mut JNIEnv, array: jarray) -> jsize,

    pub NewObjectArray: unsafe extern "system" fn(
        env: *mut JNIEnv,
        len: jsize,
        clazz: jclass,
        init: jobject,
    ) -> jobjectArray,
    pub GetObjectArrayElement:
        unsafe extern "system" fn(env: *mut JNIEnv, array: jobjectArray, index: jsize) -> jobject,
    pub SetObjectArrayElement: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jobjectArray,
        index: jsize,
        val: jobject,
    ) -> !,

    pub NewBooleanArray: unsafe extern "system" fn(env: *mut JNIEnv, len: jsize) -> jbooleanArray,
    pub NewByteArray: unsafe extern "system" fn(env: *mut JNIEnv, len: jsize) -> jbyteArray,
    pub NewCharArray: unsafe extern "system" fn(env: *mut JNIEnv, len: jsize) -> jcharArray,
    pub NewShortArray: unsafe extern "system" fn(env: *mut JNIEnv, len: jsize) -> jshortArray,
    pub NewIntArray: unsafe extern "system" fn(env: *mut JNIEnv, len: jsize) -> jintArray,
    pub NewLongArray: unsafe extern "system" fn(env: *mut JNIEnv, len: jsize) -> jlongArray,
    pub NewFloatArray: unsafe extern "system" fn(env: *mut JNIEnv, len: jsize) -> jfloatArray,
    pub NewDoubleArray: unsafe extern "system" fn(env: *mut JNIEnv, len: jsize) -> jdoubleArray,

    pub GetBooleanArrayElements: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jbooleanArray,
        isCopy: *mut jboolean,
    ) -> *mut jboolean,
    pub GetByteArrayElements: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jbyteArray,
        isCopy: *mut jboolean,
    ) -> *mut jbyte,
    pub GetCharArrayElements: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jcharArray,
        isCopy: *mut jboolean,
    ) -> *mut jchar,
    pub GetShortArrayElements: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jshortArray,
        isCopy: *mut jboolean,
    ) -> *mut jshort,
    pub GetIntArrayElements: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jintArray,
        isCopy: *mut jboolean,
    ) -> *mut jint,
    pub GetLongArrayElements: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jlongArray,
        isCopy: *mut jboolean,
    ) -> *mut jlong,
    pub GetFloatArrayElements: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jfloatArray,
        isCopy: *mut jboolean,
    ) -> *mut jfloat,
    pub GetDoubleArrayElements: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jdoubleArray,
        isCopy: *mut jboolean,
    ) -> *mut jdouble,

    pub ReleaseBooleanArrayElements: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jbooleanArray,
        elems: *mut jboolean,
        mode: jint,
    ) -> !,
    pub ReleaseByteArrayElements: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jbyteArray,
        elems: *mut jbyte,
        mode: jint,
    ) -> !,
    pub ReleaseCharArrayElements: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jcharArray,
        elems: *mut jchar,
        mode: jint,
    ) -> !,
    pub ReleaseShortArrayElements: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jshortArray,
        elems: *mut jshort,
        mode: jint,
    ) -> !,
    pub ReleaseIntArrayElements: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jintArray,
        elems: *mut jint,
        mode: jint,
    ) -> !,
    pub ReleaseLongArrayElements: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jlongArray,
        elems: *mut jlong,
        mode: jint,
    ) -> !,
    pub ReleaseFloatArrayElements: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jfloatArray,
        elems: *mut jfloat,
        mode: jint,
    ) -> !,
    pub ReleaseDoubleArrayElements: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jdoubleArray,
        elems: *mut jdouble,
        mode: jint,
    ) -> !,

    pub GetBooleanArrayRegion: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jbooleanArray,
        start: jsize,
        len: jsize,
        buf: *mut jboolean,
    ) -> !,
    pub GetByteArrayRegion: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jbyteArray,
        start: jsize,
        len: jsize,
        buf: *mut jbyte,
    ) -> !,
    pub GetCharArrayRegion: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jcharArray,
        start: jsize,
        len: jsize,
        buf: *mut jchar,
    ) -> !,
    pub GetShortArrayRegion: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jshortArray,
        start: jsize,
        len: jsize,
        buf: *mut jshort,
    ) -> !,
    pub GetIntArrayRegion: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jintArray,
        start: jsize,
        len: jsize,
        buf: *mut jint,
    ) -> !,
    pub GetLongArrayRegion: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jlongArray,
        start: jsize,
        len: jsize,
        buf: *mut jlong,
    ) -> !,
    pub GetFloatArrayRegion: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jfloatArray,
        start: jsize,
        len: jsize,
        buf: *mut jfloat,
    ) -> !,
    pub GetDoubleArrayRegion: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jdoubleArray,
        start: jsize,
        len: jsize,
        buf: *mut jdouble,
    ) -> !,

    pub SetBooleanArrayRegion: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jbooleanArray,
        start: jsize,
        len: jsize,
        buf: *const jboolean,
    ) -> !,
    pub SetByteArrayRegion: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jbyteArray,
        start: jsize,
        len: jsize,
        buf: *const jbyte,
    ) -> !,
    pub SetCharArrayRegion: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jcharArray,
        start: jsize,
        len: jsize,
        buf: *const jchar,
    ) -> !,
    pub SetShortArrayRegion: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jshortArray,
        start: jsize,
        len: jsize,
        buf: *const jshort,
    ) -> !,
    pub SetIntArrayRegion: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jintArray,
        start: jsize,
        len: jsize,
        buf: *const jint,
    ) -> !,
    pub SetLongArrayRegion: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jlongArray,
        start: jsize,
        len: jsize,
        buf: *const jlong,
    ) -> !,
    pub SetFloatArrayRegion: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jfloatArray,
        start: jsize,
        len: jsize,
        buf: *const jfloat,
    ) -> !,
    pub SetDoubleArrayRegion: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jdoubleArray,
        start: jsize,
        len: jsize,
        buf: *const jdouble,
    ) -> !,

    pub RegisterNatives: unsafe extern "system" fn(
        env: *mut JNIEnv,
        clazz: jclass,
        methods: *const JNINativeMethod,
        nMethods: jint,
    ) -> jint,
    pub UnregisterNatives: unsafe extern "system" fn(env: *mut JNIEnv, clazz: jclass) -> jint,

    pub MonitorEnter: unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject) -> jint,
    pub MonitorExit: unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject) -> jint,

    pub GetJavaVM: unsafe extern "system" fn(env: *mut JNIEnv, vm: *mut *mut JavaVM) -> jint,

    pub GetStringRegion: unsafe extern "system" fn(
        env: *mut JNIEnv,
        str: jstring,
        start: jsize,
        len: jsize,
        buf: *mut jchar,
    ) -> !,
    pub GetStringUTFRegion: unsafe extern "system" fn(
        env: *mut JNIEnv,
        str: jstring,
        start: jsize,
        len: jsize,
        buf: *mut c_char,
    ) -> !,

    pub GetPrimitiveArrayCritical: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jarray,
        isCopy: *mut jboolean,
    ) -> *mut c_void,
    pub ReleasePrimitiveArrayCritical: unsafe extern "system" fn(
        env: *mut JNIEnv,
        array: jarray,
        carray: *mut c_void,
        mode: jint,
    ) -> !,

    pub GetStringCritical: unsafe extern "system" fn(
        env: *mut JNIEnv,
        string: jstring,
        isCopy: *mut jboolean,
    ) -> *const jchar,
    pub ReleaseStringCritical:
        unsafe extern "system" fn(env: *mut JNIEnv, string: jstring, cstring: *const jchar) -> !,

    pub NewWeakGlobalRef: unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject) -> jweak,
    pub DeleteWeakGlobalRef: unsafe extern "system" fn(env: *mut JNIEnv, gref: jweak) -> !,

    pub ExceptionCheck: unsafe extern "system" fn(env: *mut JNIEnv) -> jboolean,

    pub NewDirectByteBuffer: unsafe extern "system" fn(
        env: *mut JNIEnv,
        address: *mut c_void,
        capacity: jlong,
    ) -> jobject,
    pub GetDirectBufferAddress:
        unsafe extern "system" fn(env: *mut JNIEnv, buf: jobject) -> *mut c_void,
    pub GetDirectBufferCapacity: unsafe extern "system" fn(env: *mut JNIEnv, buf: jobject) -> jlong,

    /// New JNI 1.6 Features
    ///
    /// `JNI_VERSION` >= `JNI_VERSION_1_6` can be used normally
    pub GetObjectRefType:
        unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject) -> jobjectRefType,
    /// Module Features
    ///
    /// `JNI_VERSION` >= `JNI_VERSION_9` can be used normally
    pub GetModule: unsafe extern "system" fn(env: *mut JNIEnv, clazz: jclass) -> jobject,
}

/// JNI invocation interface
#[repr(C)]
pub struct JNIInvokeInterface {
    reserved0: *mut c_void,
    reserved1: *mut c_void,
    reserved2: *mut c_void,

    pub DestroyJavaVM: unsafe extern "system" fn(vm: *mut JavaVM) -> jint,

    pub AttachCurrentThread: unsafe extern "system" fn(
        vm: *mut JavaVM,
        penv: *mut *mut c_void,
        args: *mut c_void,
    ) -> jint,

    pub DetachCurrentThread: unsafe extern "system" fn(vm: *mut JavaVM) -> jint,

    pub GetEnv:
        unsafe extern "system" fn(vm: *mut JavaVM, penv: *mut *mut c_void, version: jint) -> jint,

    pub AttachCurrentThreadAsDaemon: unsafe extern "system" fn(
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

/*
 * possible return values for JNI functions
 */

/// no error
pub const JNI_OK: jint = 0;
/// generic error
pub const JNI_ERR: jint = -1;
/// thread detached from the VM
pub const JNI_EDETACHED: jint = -2;
/// JNI version error
pub const JNI_EVERSION: jint = -3;
/// Out of memory
pub const JNI_ENOMEM: jint = -4;
/// VM already created
pub const JNI_EEXIST: jint = -5;
/// Invalid argument
pub const JNI_EINVAL: jint = -6;

/*
 * used in ReleaseScalarArrayElements
 */

/// copy content, do not free buffer
pub const JNI_COMMIT: jint = 1;
/// free buffer w/o copying back
pub const JNI_ABORT: jint = 2;
