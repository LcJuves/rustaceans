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

/// # Examples
///
/// ```rust
/// unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: jclass, methodID: jmethodID) -> jobject)
/// ```
/// expand to
/// ```rust
/// Option<unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jobject>
/// ```
#[macro_export]
macro_rules! unsafe_extern_c_var_fn {
    (($($param_name:tt: $param_type:ty), *) -> $ret_ty:ty) => {
        Option<unsafe extern "C" fn($($param_name: $param_type, )* ...) -> $ret_ty>
    }
}

/// # Such as
///
/// ```rust
/// jni_fn_def!(
///     JNI_OnLoad,
///     (vm: *mut JavaVM, reserved: *mut c_void),
///     jint,
///     {
///         /* code */
///         /* The return value must be >= JNI_VERSION_1_1 */
///     }
/// );
/// ```
/// expand to
/// ```rust
/// #[no_mangle]
/// pub extern "system" fn JNI_OnLoad(vm: *mut JavaVM, reserved: *mut c_void) -> jint {
///     /* code */
///     /* The return value must be >= JNI_VERSION_1_1 */
/// }
/// ```
#[macro_export]
macro_rules! jni_fn_def {
    ($name:tt, ($($ident:tt: $ty:ty), *), $ret_ty:ty, $code:block) => {
        #[no_mangle]
        pub extern "system" fn $name($($ident: $ty, )*) -> $ret_ty $code
    };
}

/// # Such as
///
/// ```rust
/// unsafe_jni_fn_def!(
///     JNI_OnUnload,
///     (vm: *mut JavaVM, reserved: *mut c_void),
///     (),
///     { /* code */ }
/// );
/// ```
/// expand to
/// ```rust
/// #[no_mangle]
/// pub unsafe extern "system" fn JNI_OnUnload(vm: *mut JavaVM, reserved: *mut c_void) {
///     /* code */
/// }
/// ```
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
/// pub extern "system" fn JNI_OnLoad(vm: *mut JavaVM, reserved: *mut c_void) -> jint {
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
/// pub extern "system" fn JNI_OnUnload(vm: *mut JavaVM, reserved: *mut c_void) {
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
    pub GetVersion: unsafe_extern_system_fn!((env: *mut JNIEnv) -> jint),

    pub DefineClass: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        name: *const c_char,
        loader: jobject,
        buf: *const jbyte,
        len: jsize
    ) -> jclass),
    pub FindClass: unsafe_extern_system_fn!((env: *mut JNIEnv, name: *const c_char) -> jclass),

    pub FromReflectedMethod: unsafe_extern_system_fn!((env: *mut JNIEnv, method: jobject) -> jmethodID),
    pub FromReflectedField: unsafe_extern_system_fn!((env: *mut JNIEnv, field: jobject) -> jfieldID),

    /// spec doesn't show jboolean parameter
    pub ToReflectedMethod: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        cls: jclass,
        methodID: jmethodID,
        isStatic: jboolean
    ) -> jobject),

    pub GetSuperclass: unsafe_extern_system_fn!((env: *mut JNIEnv, sub: jclass) -> jclass),
    pub IsAssignableFrom: unsafe_extern_system_fn!((env: *mut JNIEnv, sub: jclass, sup: jclass) -> jboolean),

    /// spec doesn't show jboolean parameter
    pub ToReflectedField: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        cls: jclass,
        fieldID: jfieldID,
        isStatic: jboolean
    ) -> jobject),

    pub Throw: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: jthrowable) -> jint),
    pub ThrowNew: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: jclass, msg: *const c_char) -> jint),
    pub ExceptionOccurred: unsafe_extern_system_fn!((env: *mut JNIEnv) -> jthrowable),
    pub ExceptionDescribe: unsafe_extern_system_fn!((env: *mut JNIEnv) -> !),
    pub ExceptionClear: unsafe_extern_system_fn!((env: *mut JNIEnv) -> !),
    pub FatalError: unsafe_extern_system_fn!((env: *mut JNIEnv, msg: *const c_char) -> !),

    pub PushLocalFrame: unsafe_extern_system_fn!((env: *mut JNIEnv, capacity: jint) -> jint),
    pub PopLocalFrame: unsafe_extern_system_fn!((env: *mut JNIEnv, result: jobject) -> jobject),

    pub NewGlobalRef: unsafe_extern_system_fn!((env: *mut JNIEnv, gref: jobject) -> jobject),
    pub DeleteGlobalRef: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: jobject) -> !),
    pub DeleteLocalRef: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: jobject) -> !),
    pub IsSameObject: unsafe_extern_system_fn!((env: *mut JNIEnv, obj1: jobject, obj2: jobject) -> jboolean),
    pub NewLocalRef: unsafe_extern_system_fn!((env: *mut JNIEnv, lref: jobject) -> jobject),
    pub EnsureLocalCapacity: unsafe_extern_system_fn!((env: *mut JNIEnv, capacity: jint) -> jint),

    pub AllocObject: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: jclass) -> jobject),
    pub NewObject: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: jclass, methodID: jmethodID) -> jobject),
    pub NewObjectV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list
    ) -> jobject),
    pub NewObjectA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jobject),

    pub GetObjectClass: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: jobject) -> jclass),
    pub IsInstanceOf: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: jobject, clazz: jclass) -> jboolean),

    pub GetMethodID: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        name: *const c_char,
        sig: *const c_char
    ) -> jmethodID),

    pub CallObjectMethod: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: jobject, methodID: jmethodID) -> jobject),
    pub CallObjectMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list
    ) -> jobject),
    pub CallObjectMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jobject),

    pub CallBooleanMethod: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: jobject, methodID: jmethodID) -> jboolean),
    pub CallBooleanMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list
    ) -> jboolean),
    pub CallBooleanMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jboolean),

    pub CallByteMethod: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: jobject, methodID: jmethodID) -> jbyte),
    pub CallByteMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list
    ) -> jbyte),
    pub CallByteMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jbyte),

    pub CallCharMethod: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: jobject, methodID: jmethodID) -> jchar),
    pub CallCharMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list
    ) -> jchar),
    pub CallCharMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jchar),

    pub CallShortMethod: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: jobject, methodID: jmethodID) -> jshort),
    pub CallShortMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list
    ) -> jshort),
    pub CallShortMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jshort),

    pub CallIntMethod: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: jobject, methodID: jmethodID) -> jint),
    pub CallIntMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list
    ) -> jint),
    pub CallIntMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jint),

    pub CallLongMethod: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: jobject, methodID: jmethodID) -> jlong),
    pub CallLongMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list
    ) -> jlong),
    pub CallLongMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jlong),

    pub CallFloatMethod: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: jobject, methodID: jmethodID) -> jfloat),
    pub CallFloatMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list
    ) -> jfloat),
    pub CallFloatMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jfloat),

    pub CallDoubleMethod: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: jobject, methodID: jmethodID) -> jdouble),
    pub CallDoubleMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list
    ) -> jdouble),
    pub CallDoubleMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jdouble),

    pub CallVoidMethod: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: jobject, methodID: jmethodID) -> !),
    pub CallVoidMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: va_list
    ) -> !),
    pub CallVoidMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue
    ) -> !),

    pub CallNonvirtualObjectMethod: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID) -> jobject),
    pub CallNonvirtualObjectMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list
    ) -> jobject),
    pub CallNonvirtualObjectMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jobject),

    pub CallNonvirtualBooleanMethod: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID) -> jboolean),
    pub CallNonvirtualBooleanMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list
    ) -> jboolean),
    pub CallNonvirtualBooleanMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jboolean),

    pub CallNonvirtualByteMethod: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID) -> jbyte),
    pub CallNonvirtualByteMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list
    ) -> jbyte),
    pub CallNonvirtualByteMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jbyte),

    pub CallNonvirtualCharMethod: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID) -> jchar),
    pub CallNonvirtualCharMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list
    ) -> jchar),
    pub CallNonvirtualCharMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jchar),

    pub CallNonvirtualShortMethod: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID) -> jshort),
    pub CallNonvirtualShortMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list
    ) -> jshort),
    pub CallNonvirtualShortMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jshort),

    pub CallNonvirtualIntMethod: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID) -> jint),
    pub CallNonvirtualIntMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list
    ) -> jint),
    pub CallNonvirtualIntMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jint),

    pub CallNonvirtualLongMethod: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID) -> jlong),
    pub CallNonvirtualLongMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list
    ) -> jlong),
    pub CallNonvirtualLongMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jlong),

    pub CallNonvirtualFloatMethod: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID) -> jfloat),
    pub CallNonvirtualFloatMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list
    ) -> jfloat),
    pub CallNonvirtualFloatMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jfloat),

    pub CallNonvirtualDoubleMethod: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID) -> jdouble),
    pub CallNonvirtualDoubleMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list
    ) -> jdouble),
    pub CallNonvirtualDoubleMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jdouble),

    pub CallNonvirtualVoidMethod: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID) -> !),
    pub CallNonvirtualVoidMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list
    ) -> !),
    pub CallNonvirtualVoidMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue
    ) -> !),

    pub GetFieldID: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        name: *const c_char,
        sig: *const c_char
    ) -> jfieldID),

    pub GetObjectField: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jobject),
    pub GetBooleanField: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jboolean),
    pub GetByteField: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jbyte),
    pub GetCharField: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jchar),
    pub GetShortField: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jshort),
    pub GetIntField: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jint),
    pub GetLongField: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jlong),
    pub GetFloatField: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jfloat),
    pub GetDoubleField: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jdouble),

    pub SetObjectField: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        fieldID: jfieldID,
        val: jobject
    ) -> !),
    pub SetBooleanField: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        fieldID: jfieldID,
        val: jboolean
    ) -> !),
    pub SetByteField: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        fieldID: jfieldID,
        val: jbyte
    ) -> !),
    pub SetCharField: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        fieldID: jfieldID,
        val: jchar
    ) -> !),
    pub SetShortField: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        fieldID: jfieldID,
        val: jshort
    ) -> !),
    pub SetIntField: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        fieldID: jfieldID,
        val: jint
    ) -> !),
    pub SetLongField: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        fieldID: jfieldID,
        val: jlong
    ) -> !),
    pub SetFloatField: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        fieldID: jfieldID,
        val: jfloat
    ) -> !),
    pub SetDoubleField: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: jobject,
        fieldID: jfieldID,
        val: jdouble
    ) -> !),

    pub GetStaticMethodID: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        name: *const c_char,
        sig: *const c_char
    ) -> jmethodID),

    pub CallStaticObjectMethod: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: jclass, methodID: jmethodID) -> jobject),
    pub CallStaticObjectMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list
    ) -> jobject),
    pub CallStaticObjectMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jobject),

    pub CallStaticBooleanMethod: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: jclass, methodID: jmethodID) -> jboolean),
    pub CallStaticBooleanMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list
    ) -> jboolean),
    pub CallStaticBooleanMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jboolean),

    pub CallStaticByteMethod: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: jclass, methodID: jmethodID) -> jbyte),
    pub CallStaticByteMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list
    ) -> jbyte),
    pub CallStaticByteMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jbyte),

    pub CallStaticCharMethod: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: jclass, methodID: jmethodID) -> jchar),
    pub CallStaticCharMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list
    ) -> jchar),
    pub CallStaticCharMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jchar),

    pub CallStaticShortMethod: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: jclass, methodID: jmethodID) -> jshort),
    pub CallStaticShortMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list
    ) -> jshort),
    pub CallStaticShortMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jshort),

    pub CallStaticIntMethod: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: jclass, methodID: jmethodID) -> jint),
    pub CallStaticIntMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list
    ) -> jint),
    pub CallStaticIntMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jint),

    pub CallStaticLongMethod: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: jclass, methodID: jmethodID) -> jlong),
    pub CallStaticLongMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list
    ) -> jlong),
    pub CallStaticLongMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jlong),

    pub CallStaticFloatMethod: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: jclass, methodID: jmethodID) -> jfloat),
    pub CallStaticFloatMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list
    ) -> jfloat),
    pub CallStaticFloatMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jfloat),

    pub CallStaticDoubleMethod: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: jclass, methodID: jmethodID) -> jdouble),
    pub CallStaticDoubleMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list
    ) -> jdouble),
    pub CallStaticDoubleMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue
    ) -> jdouble),

    pub CallStaticVoidMethod: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: jclass, methodID: jmethodID) -> !),
    pub CallStaticVoidMethodV: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: va_list
    ) -> !),
    pub CallStaticVoidMethodA: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue
    ) -> !),

    pub GetStaticFieldID: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        name: *const c_char,
        sig: *const c_char
    ) -> jfieldID),
    pub GetStaticObjectField: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jobject),
    pub GetStaticBooleanField: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jboolean),
    pub GetStaticByteField: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jbyte),
    pub GetStaticCharField: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jchar),
    pub GetStaticShortField: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jshort),
    pub GetStaticIntField: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jint),
    pub GetStaticLongField: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jlong),
    pub GetStaticFloatField: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jfloat),
    pub GetStaticDoubleField: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jdouble),

    pub SetStaticObjectField: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        fieldID: jfieldID,
        value: jobject
    ) -> !),
    pub SetStaticBooleanField: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        fieldID: jfieldID,
        value: jboolean
    ) -> !),
    pub SetStaticByteField: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        fieldID: jfieldID,
        value: jbyte
    ) -> !),
    pub SetStaticCharField: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        fieldID: jfieldID,
        value: jchar
    ) -> !),
    pub SetStaticShortField: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        fieldID: jfieldID,
        value: jshort
    ) -> !),
    pub SetStaticIntField: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        fieldID: jfieldID,
        value: jint
    ) -> !),
    pub SetStaticLongField: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        fieldID: jfieldID,
        value: jlong
    ) -> !),
    pub SetStaticFloatField: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        fieldID: jfieldID,
        value: jfloat
    ) -> !),
    pub SetStaticDoubleField: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        fieldID: jfieldID,
        value: jdouble
    ) -> !),

    pub NewString: unsafe_extern_system_fn!((env: *mut JNIEnv, unicode: *const jchar, len: jsize) -> jstring),
    pub GetStringLength: unsafe_extern_system_fn!((env: *mut JNIEnv, str: jstring) -> jsize),
    pub GetStringChars: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        str: jstring,
        isCopy: *mut jboolean
    ) -> *const jchar),
    pub ReleaseStringChars: unsafe_extern_system_fn!((env: *mut JNIEnv, str: jstring, chars: *const jchar) -> !),

    pub NewStringUTF: unsafe_extern_system_fn!((env: *mut JNIEnv, utf: *const c_char) -> jstring),
    pub GetStringUTFLength: unsafe_extern_system_fn!((env: *mut JNIEnv, str: jstring) -> jsize),
    pub GetStringUTFChars: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        str: jstring,
        isCopy: *mut jboolean
    ) -> *const c_char),
    pub ReleaseStringUTFChars: unsafe_extern_system_fn!((env: *mut JNIEnv, str: jstring, chars: *const c_char) -> !),

    pub GetArrayLength: unsafe_extern_system_fn!((env: *mut JNIEnv, array: jarray) -> jsize),

    pub NewObjectArray: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        len: jsize,
        clazz: jclass,
        init: jobject
    ) -> jobjectArray),
    pub GetObjectArrayElement: unsafe_extern_system_fn!((env: *mut JNIEnv, array: jobjectArray, index: jsize) -> jobject),
    pub SetObjectArrayElement: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jobjectArray,
        index: jsize,
        val: jobject
    ) -> !),

    pub NewBooleanArray: unsafe_extern_system_fn!((env: *mut JNIEnv, len: jsize) -> jbooleanArray),
    pub NewByteArray: unsafe_extern_system_fn!((env: *mut JNIEnv, len: jsize) -> jbyteArray),
    pub NewCharArray: unsafe_extern_system_fn!((env: *mut JNIEnv, len: jsize) -> jcharArray),
    pub NewShortArray: unsafe_extern_system_fn!((env: *mut JNIEnv, len: jsize) -> jshortArray),
    pub NewIntArray: unsafe_extern_system_fn!((env: *mut JNIEnv, len: jsize) -> jintArray),
    pub NewLongArray: unsafe_extern_system_fn!((env: *mut JNIEnv, len: jsize) -> jlongArray),
    pub NewFloatArray: unsafe_extern_system_fn!((env: *mut JNIEnv, len: jsize) -> jfloatArray),
    pub NewDoubleArray: unsafe_extern_system_fn!((env: *mut JNIEnv, len: jsize) -> jdoubleArray),

    pub GetBooleanArrayElements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jbooleanArray,
        isCopy: *mut jboolean
    ) -> *mut jboolean),
    pub GetByteArrayElements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jbyteArray,
        isCopy: *mut jboolean
    ) -> *mut jbyte),
    pub GetCharArrayElements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jcharArray,
        isCopy: *mut jboolean
    ) -> *mut jchar),
    pub GetShortArrayElements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jshortArray,
        isCopy: *mut jboolean
    ) -> *mut jshort),
    pub GetIntArrayElements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jintArray,
        isCopy: *mut jboolean
    ) -> *mut jint),
    pub GetLongArrayElements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jlongArray,
        isCopy: *mut jboolean
    ) -> *mut jlong),
    pub GetFloatArrayElements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jfloatArray,
        isCopy: *mut jboolean
    ) -> *mut jfloat),
    pub GetDoubleArrayElements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jdoubleArray,
        isCopy: *mut jboolean
    ) -> *mut jdouble),

    pub ReleaseBooleanArrayElements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jbooleanArray,
        elems: *mut jboolean,
        mode: jint
    ) -> !),
    pub ReleaseByteArrayElements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jbyteArray,
        elems: *mut jbyte,
        mode: jint
    ) -> !),
    pub ReleaseCharArrayElements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jcharArray,
        elems: *mut jchar,
        mode: jint
    ) -> !),
    pub ReleaseShortArrayElements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jshortArray,
        elems: *mut jshort,
        mode: jint
    ) -> !),
    pub ReleaseIntArrayElements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jintArray,
        elems: *mut jint,
        mode: jint
    ) -> !),
    pub ReleaseLongArrayElements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jlongArray,
        elems: *mut jlong,
        mode: jint
    ) -> !),
    pub ReleaseFloatArrayElements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jfloatArray,
        elems: *mut jfloat,
        mode: jint
    ) -> !),
    pub ReleaseDoubleArrayElements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jdoubleArray,
        elems: *mut jdouble,
        mode: jint
    ) -> !),

    pub GetBooleanArrayRegion: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jbooleanArray,
        start: jsize,
        len: jsize,
        buf: *mut jboolean
    ) -> !),
    pub GetByteArrayRegion: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jbyteArray,
        start: jsize,
        len: jsize,
        buf: *mut jbyte
    ) -> !),
    pub GetCharArrayRegion: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jcharArray,
        start: jsize,
        len: jsize,
        buf: *mut jchar
    ) -> !),
    pub GetShortArrayRegion: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jshortArray,
        start: jsize,
        len: jsize,
        buf: *mut jshort
    ) -> !),
    pub GetIntArrayRegion: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jintArray,
        start: jsize,
        len: jsize,
        buf: *mut jint
    ) -> !),
    pub GetLongArrayRegion: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jlongArray,
        start: jsize,
        len: jsize,
        buf: *mut jlong
    ) -> !),
    pub GetFloatArrayRegion: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jfloatArray,
        start: jsize,
        len: jsize,
        buf: *mut jfloat
    ) -> !),
    pub GetDoubleArrayRegion: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jdoubleArray,
        start: jsize,
        len: jsize,
        buf: *mut jdouble
    ) -> !),

    pub SetBooleanArrayRegion: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jbooleanArray,
        start: jsize,
        len: jsize,
        buf: *const jboolean
    ) -> !),
    pub SetByteArrayRegion: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jbyteArray,
        start: jsize,
        len: jsize,
        buf: *const jbyte
    ) -> !),
    pub SetCharArrayRegion: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jcharArray,
        start: jsize,
        len: jsize,
        buf: *const jchar
    ) -> !),
    pub SetShortArrayRegion: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jshortArray,
        start: jsize,
        len: jsize,
        buf: *const jshort
    ) -> !),
    pub SetIntArrayRegion: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jintArray,
        start: jsize,
        len: jsize,
        buf: *const jint
    ) -> !),
    pub SetLongArrayRegion: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jlongArray,
        start: jsize,
        len: jsize,
        buf: *const jlong
    ) -> !),
    pub SetFloatArrayRegion: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jfloatArray,
        start: jsize,
        len: jsize,
        buf: *const jfloat
    ) -> !),
    pub SetDoubleArrayRegion: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jdoubleArray,
        start: jsize,
        len: jsize,
        buf: *const jdouble
    ) -> !),

    pub RegisterNatives: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: jclass,
        methods: *const JNINativeMethod,
        nMethods: jint
    ) -> jint),
    pub UnregisterNatives: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: jclass) -> jint),

    pub MonitorEnter: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: jobject) -> jint),
    pub MonitorExit: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: jobject) -> jint),

    pub GetJavaVM: unsafe_extern_system_fn!((env: *mut JNIEnv, vm: *mut *mut JavaVM) -> jint),

    pub GetStringRegion: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        str: jstring,
        start: jsize,
        len: jsize,
        buf: *mut jchar
    ) -> !),
    pub GetStringUTFRegion: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        str: jstring,
        start: jsize,
        len: jsize,
        buf: *mut c_char
    ) -> !),

    pub GetPrimitiveArrayCritical: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jarray,
        isCopy: *mut jboolean
    ) -> *mut c_void),
    pub ReleasePrimitiveArrayCritical: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: jarray,
        carray: *mut c_void,
        mode: jint
    ) -> !),

    pub GetStringCritical: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        string: jstring,
        isCopy: *mut jboolean
    ) -> *const jchar),
    pub ReleaseStringCritical: unsafe_extern_system_fn!((env: *mut JNIEnv, string: jstring, cstring: *const jchar) -> !),

    pub NewWeakGlobalRef: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: jobject) -> jweak),
    pub DeleteWeakGlobalRef: unsafe_extern_system_fn!((env: *mut JNIEnv, gref: jweak) -> !),

    pub ExceptionCheck: unsafe_extern_system_fn!((env: *mut JNIEnv) -> jboolean),

    pub NewDirectByteBuffer: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        address: *mut c_void,
        capacity: jlong
    ) -> jobject),
    pub GetDirectBufferAddress: unsafe_extern_system_fn!((env: *mut JNIEnv, buf: jobject) -> *mut c_void),
    pub GetDirectBufferCapacity: unsafe_extern_system_fn!((env: *mut JNIEnv, buf: jobject) -> jlong),

    /// New JNI 1.6 Features
    ///
    /// `JNI_VERSION` >= `JNI_VERSION_1_6` can be used normally
    pub GetObjectRefType: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: jobject) -> jobjectRefType),
    /// Module Features
    ///
    /// `JNI_VERSION` >= `JNI_VERSION_9` can be used normally
    pub GetModule: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: jclass) -> jobject),
}

/// JNI invocation interface
#[repr(C)]
pub struct JNIInvokeInterface {
    reserved0: *mut c_void,
    reserved1: *mut c_void,
    reserved2: *mut c_void,

    pub DestroyJavaVM: unsafe extern "system" fn(vm: *mut JavaVM) -> jint,

    pub AttachCurrentThread: unsafe_extern_system_fn!((
        vm: *mut JavaVM,
        penv: *mut *mut c_void,
        args: *mut c_void
    ) -> jint),

    pub DetachCurrentThread: unsafe extern "system" fn(vm: *mut JavaVM) -> jint,

    pub GetEnv: unsafe_extern_system_fn!((vm: *mut JavaVM, penv: *mut *mut c_void, version: jint) -> jint),

    pub AttachCurrentThreadAsDaemon: unsafe_extern_system_fn!((
        vm: *mut JavaVM,
        penv: *mut *mut c_void,
        args: *mut c_void
    ) -> jint),
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
