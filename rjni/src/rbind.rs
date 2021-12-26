#![allow(dead_code)]

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
/// unsafe_extern_system_fn!((env: *mut JNIEnv) -> Jint)
/// ```
/// expand to
/// ```rust
/// Option<unsafe extern "system" fn(env: *mut JNIEnv) -> Jint>
/// ```
#[macro_export]
macro_rules! unsafe_extern_system_fn {
    (($($param_name:tt: $param_ty:ty), *) -> $ret_ty:ty) => {
        Option<unsafe extern "system" fn($($param_name: $param_ty, )*) -> $ret_ty>
    }
}

/// # Examples
///
/// ```rust
/// unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, methodID: JmethodID) -> Jobject)
/// ```
/// expand to
/// ```rust
/// Option<unsafe extern "C" fn(env: *mut JNIEnv, clazz: Jclass, methodID: JmethodID, ...) -> Jobject>
/// ```
#[macro_export]
macro_rules! unsafe_extern_c_var_fn {
    (($($param_name:tt: $param_ty:ty), *) -> $ret_ty:ty) => {
        Option<unsafe extern "C" fn($($param_name: $param_ty, )* ...) -> $ret_ty>
    }
}

/// # Such as
///
/// ```rust
/// jni_fn_def!(
///     JNI_OnLoad,
///     (vm: *mut JavaVM, reserved: *mut c_void),
///     Jint,
///     {
///         /* code */
///         /* The return value must be >= JNI_VERSION_1_1 */
///     }
/// );
/// ```
/// expand to
/// ```rust
/// #[no_mangle]
/// pub extern "system" fn JNI_OnLoad(vm: *mut JavaVM, reserved: *mut c_void) -> Jint {
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
/// pub extern "system" fn JNI_OnLoad(vm: *mut JavaVM, reserved: *mut c_void) -> Jint {
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
            ($param_vm_name: *mut JavaVM, $param_reserved_name: *mut c_void),
            Jint,
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
            ($param_vm_name: *mut JavaVM, $param_reserved_name: *mut c_void),
            (),
            $code
        );
    };
}

use std::ffi::c_void;
use std::os::raw::c_char;

pub type VaList = *mut c_void;

/* Primitive types that match up with Java equivalents */

/// unsigned 8 bits
pub type Jboolean = u8;
/// signed 8 bits
pub type Jbyte = i8;
/// unsigned 16 bits
pub type Jchar = u16;
/// signed 16 bits
pub type Jshort = i16;
/// signed 32 bits
pub type Jint = i32;
/// signed 64 bits
pub type Jlong = i64;
/// 32-bit IEEE 754
pub type Jfloat = f32;
/// 64-bit IEEE 754
pub type Jdouble = f64;

/// "cardinal indices and sizes"
pub type Jsize = Jint;

/* Reference types that match up with Java equivalents */

pub type Jobject = *mut c_void;
pub type Jclass = Jobject;
pub type Jthrowable = Jobject;
pub type Jstring = Jobject;

/* Array types that match up with Java equivalents */

pub type Jarray = Jobject;
pub type JbooleanArray = Jarray;
pub type JbyteArray = Jarray;
pub type JcharArray = Jarray;
pub type JshortArray = Jarray;
pub type JintArray = Jarray;
pub type JlongArray = Jarray;
pub type JfloatArray = Jarray;
pub type JdoubleArray = Jarray;
pub type JobjectArray = Jarray;

pub type Jweak = Jobject;

/// When passing arguments from Rust to a Java method, the jvalue union is used
#[repr(C)]
pub union jvalue {
    // Primitive types
    pub z: Jboolean,
    pub b: Jbyte,
    pub c: Jchar,
    pub s: Jshort,
    pub i: Jint,
    pub j: Jlong,
    pub f: Jfloat,
    pub d: Jdouble,
    // Reference types
    pub l: Jobject,
}

/// field IDs
pub type JfieldID = *mut c_void;

/// method IDs
pub type JmethodID = *mut c_void;

/// Return values from JobjectRefType
#[repr(C)]
pub enum JobjectRefType {
    JNIInvalidRefType = 0,
    JNILocalRefType = 1,
    JNIGlobalRefType = 2,
    JNIWeakGlobalRefType = 3,
}

#[repr(C)]
pub struct JNINativeMethod {
    pub name: *const c_char,
    pub signature: *const c_char,
    pub fn_ptr: *mut c_void,
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
    pub get_version: unsafe_extern_system_fn!((env: *mut JNIEnv) -> Jint),

    pub define_class: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        name: *const c_char,
        loader: Jobject,
        buf: *const Jbyte,
        len: Jsize
    ) -> Jclass),
    pub find_class: unsafe_extern_system_fn!((env: *mut JNIEnv, name: *const c_char) -> Jclass),

    pub from_reflected_method: unsafe_extern_system_fn!((env: *mut JNIEnv, method: Jobject) -> JmethodID),
    pub from_reflected_field: unsafe_extern_system_fn!((env: *mut JNIEnv, field: Jobject) -> JfieldID),

    /// spec doesn't show Jboolean parameter
    pub to_reflected_method: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        cls: Jclass,
        methodID: JmethodID,
        isStatic: Jboolean
    ) -> Jobject),

    pub get_superclass: unsafe_extern_system_fn!((env: *mut JNIEnv, sub: Jclass) -> Jclass),
    pub is_assignable_from: unsafe_extern_system_fn!((env: *mut JNIEnv, sub: Jclass, sup: Jclass) -> Jboolean),

    /// spec doesn't show Jboolean parameter
    pub to_reflected_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        cls: Jclass,
        fieldID: JfieldID,
        isStatic: Jboolean
    ) -> Jobject),

    pub throw: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jthrowable) -> Jint),
    pub throw_new: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass, msg: *const c_char) -> Jint),
    pub exception_occurred: unsafe_extern_system_fn!((env: *mut JNIEnv) -> Jthrowable),
    pub exception_describe: unsafe_extern_system_fn!((env: *mut JNIEnv) -> !),
    pub exception_clear: unsafe_extern_system_fn!((env: *mut JNIEnv) -> !),
    pub fatal_error: unsafe_extern_system_fn!((env: *mut JNIEnv, msg: *const c_char) -> !),

    pub push_local_frame: unsafe_extern_system_fn!((env: *mut JNIEnv, capacity: Jint) -> Jint),
    pub pop_local_frame: unsafe_extern_system_fn!((env: *mut JNIEnv, result: Jobject) -> Jobject),

    pub new_global_ref: unsafe_extern_system_fn!((env: *mut JNIEnv, gref: Jobject) -> Jobject),
    pub delete_global_ref: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject) -> !),
    pub delete_local_ref: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject) -> !),
    pub is_same_object: unsafe_extern_system_fn!((env: *mut JNIEnv, obj1: Jobject, obj2: Jobject) -> Jboolean),
    pub new_local_ref: unsafe_extern_system_fn!((env: *mut JNIEnv, lref: Jobject) -> Jobject),
    pub ensure_local_capacity: unsafe_extern_system_fn!((env: *mut JNIEnv, capacity: Jint) -> Jint),

    pub alloc_object: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass) -> Jobject),
    pub new_object: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, methodID: JmethodID) -> Jobject),
    pub new_object_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methodID: JmethodID,
        args: VaList
    ) -> Jobject),
    pub new_object_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jobject),

    pub get_object_class: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject) -> Jclass),
    pub is_instance_of: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, clazz: Jclass) -> Jboolean),

    pub get_method_id: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        name: *const c_char,
        sig: *const c_char
    ) -> JmethodID),

    pub call_object_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: Jobject, methodID: JmethodID) -> Jobject),
    pub call_object_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        methodID: JmethodID,
        args: VaList
    ) -> Jobject),
    pub call_object_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jobject),

    pub call_boolean_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: Jobject, methodID: JmethodID) -> Jboolean),
    pub call_boolean_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        methodID: JmethodID,
        args: VaList
    ) -> Jboolean),
    pub call_boolean_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jboolean),

    pub call_byte_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: Jobject, methodID: JmethodID) -> Jbyte),
    pub call_byte_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        methodID: JmethodID,
        args: VaList
    ) -> Jbyte),
    pub call_byte_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jbyte),

    pub call_char_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: Jobject, methodID: JmethodID) -> Jchar),
    pub call_char_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        methodID: JmethodID,
        args: VaList
    ) -> Jchar),
    pub call_char_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jchar),

    pub call_short_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: Jobject, methodID: JmethodID) -> Jshort),
    pub call_short_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        methodID: JmethodID,
        args: VaList
    ) -> Jshort),
    pub call_short_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jshort),

    pub call_int_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: Jobject, methodID: JmethodID) -> Jint),
    pub call_int_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        methodID: JmethodID,
        args: VaList
    ) -> Jint),
    pub call_int_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jint),

    pub call_long_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: Jobject, methodID: JmethodID) -> Jlong),
    pub call_long_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        methodID: JmethodID,
        args: VaList
    ) -> Jlong),
    pub call_long_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jlong),

    pub call_float_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: Jobject, methodID: JmethodID) -> Jfloat),
    pub call_float_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        methodID: JmethodID,
        args: VaList
    ) -> Jfloat),
    pub call_float_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jfloat),

    pub call_double_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: Jobject, methodID: JmethodID) -> Jdouble),
    pub call_double_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        methodID: JmethodID,
        args: VaList
    ) -> Jdouble),
    pub call_double_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jdouble),

    pub call_void_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: Jobject, methodID: JmethodID) -> !),
    pub call_void_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        methodID: JmethodID,
        args: VaList
    ) -> !),
    pub call_void_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        methodID: JmethodID,
        args: *const jvalue
    ) -> !),

    pub call_nonvirtual_object_method: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID) -> Jobject),
    pub call_nonvirtual_object_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID,
        args: VaList
    ) -> Jobject),
    pub call_nonvirtual_object_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jobject),

    pub call_nonvirtual_boolean_method: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID) -> Jboolean),
    pub call_nonvirtual_boolean_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID,
        args: VaList
    ) -> Jboolean),
    pub call_nonvirtual_boolean_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jboolean),

    pub call_nonvirtual_byte_method: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID) -> Jbyte),
    pub call_nonvirtual_byte_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID,
        args: VaList
    ) -> Jbyte),
    pub call_nonvirtual_byte_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jbyte),

    pub call_nonvirtual_char_method: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID) -> Jchar),
    pub call_nonvirtual_char_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID,
        args: VaList
    ) -> Jchar),
    pub call_nonvirtual_char_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jchar),

    pub call_nonvirtual_short_method: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID) -> Jshort),
    pub call_nonvirtual_short_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID,
        args: VaList
    ) -> Jshort),
    pub call_nonvirtual_short_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jshort),

    pub call_nonvirtual_int_method: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID) -> Jint),
    pub call_nonvirtual_int_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID,
        args: VaList
    ) -> Jint),
    pub call_nonvirtual_int_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jint),

    pub call_nonvirtual_long_method: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID) -> Jlong),
    pub call_nonvirtual_long_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID,
        args: VaList
    ) -> Jlong),
    pub call_nonvirtual_long_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jlong),

    pub call_nonvirtual_float_method: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID) -> Jfloat),
    pub call_nonvirtual_float_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID,
        args: VaList
    ) -> Jfloat),
    pub call_nonvirtual_float_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jfloat),

    pub call_nonvirtual_double_method: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID) -> Jdouble),
    pub call_nonvirtual_double_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID,
        args: VaList
    ) -> Jdouble),
    pub call_nonvirtual_double_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jdouble),

    pub call_nonvirtual_void_method: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID) -> !),
    pub call_nonvirtual_void_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID,
        args: VaList
    ) -> !),
    pub call_nonvirtual_void_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        methodID: JmethodID,
        args: *const jvalue
    ) -> !),

    pub get_field_id: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        name: *const c_char,
        sig: *const c_char
    ) -> JfieldID),

    pub get_object_field: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, fieldID: JfieldID) -> Jobject),
    pub get_boolean_field: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, fieldID: JfieldID) -> Jboolean),
    pub get_byte_field: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, fieldID: JfieldID) -> Jbyte),
    pub get_char_field: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, fieldID: JfieldID) -> Jchar),
    pub get_short_field: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, fieldID: JfieldID) -> Jshort),
    pub get_int_field: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, fieldID: JfieldID) -> Jint),
    pub get_long_field: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, fieldID: JfieldID) -> Jlong),
    pub get_float_field: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, fieldID: JfieldID) -> Jfloat),
    pub get_double_field: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, fieldID: JfieldID) -> Jdouble),

    pub set_object_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        fieldID: JfieldID,
        val: Jobject
    ) -> !),
    pub set_boolean_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        fieldID: JfieldID,
        val: Jboolean
    ) -> !),
    pub set_byte_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        fieldID: JfieldID,
        val: Jbyte
    ) -> !),
    pub set_char_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        fieldID: JfieldID,
        val: Jchar
    ) -> !),
    pub set_short_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        fieldID: JfieldID,
        val: Jshort
    ) -> !),
    pub set_int_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        fieldID: JfieldID,
        val: Jint
    ) -> !),
    pub set_long_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        fieldID: JfieldID,
        val: Jlong
    ) -> !),
    pub set_float_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        fieldID: JfieldID,
        val: Jfloat
    ) -> !),
    pub set_double_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        fieldID: JfieldID,
        val: Jdouble
    ) -> !),

    pub get_static_method_id: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        name: *const c_char,
        sig: *const c_char
    ) -> JmethodID),

    pub call_static_object_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, methodID: JmethodID) -> Jobject),
    pub call_static_object_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methodID: JmethodID,
        args: VaList
    ) -> Jobject),
    pub call_static_object_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jobject),

    pub call_static_boolean_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, methodID: JmethodID) -> Jboolean),
    pub call_static_boolean_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methodID: JmethodID,
        args: VaList
    ) -> Jboolean),
    pub call_static_boolean_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jboolean),

    pub call_static_byte_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, methodID: JmethodID) -> Jbyte),
    pub call_static_byte_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methodID: JmethodID,
        args: VaList
    ) -> Jbyte),
    pub call_static_byte_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jbyte),

    pub call_static_char_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, methodID: JmethodID) -> Jchar),
    pub call_static_char_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methodID: JmethodID,
        args: VaList
    ) -> Jchar),
    pub call_static_char_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jchar),

    pub call_static_short_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, methodID: JmethodID) -> Jshort),
    pub call_static_short_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methodID: JmethodID,
        args: VaList
    ) -> Jshort),
    pub call_static_short_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jshort),

    pub call_static_int_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, methodID: JmethodID) -> Jint),
    pub call_static_int_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methodID: JmethodID,
        args: VaList
    ) -> Jint),
    pub call_static_int_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jint),

    pub call_static_long_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, methodID: JmethodID) -> Jlong),
    pub call_static_long_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methodID: JmethodID,
        args: VaList
    ) -> Jlong),
    pub call_static_long_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jlong),

    pub call_static_float_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, methodID: JmethodID) -> Jfloat),
    pub call_static_float_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methodID: JmethodID,
        args: VaList
    ) -> Jfloat),
    pub call_static_float_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jfloat),

    pub call_static_double_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, methodID: JmethodID) -> Jdouble),
    pub call_static_double_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methodID: JmethodID,
        args: VaList
    ) -> Jdouble),
    pub call_static_double_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methodID: JmethodID,
        args: *const jvalue
    ) -> Jdouble),

    pub call_static_void_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, methodID: JmethodID) -> !),
    pub call_static_void_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methodID: JmethodID,
        args: VaList
    ) -> !),
    pub call_static_void_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methodID: JmethodID,
        args: *const jvalue
    ) -> !),

    pub get_static_field_id: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        name: *const c_char,
        sig: *const c_char
    ) -> JfieldID),
    pub get_static_object_field: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass, fieldID: JfieldID) -> Jobject),
    pub get_static_boolean_field: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass, fieldID: JfieldID) -> Jboolean),
    pub get_static_byte_field: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass, fieldID: JfieldID) -> Jbyte),
    pub get_static_char_field: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass, fieldID: JfieldID) -> Jchar),
    pub get_static_short_field: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass, fieldID: JfieldID) -> Jshort),
    pub get_static_int_field: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass, fieldID: JfieldID) -> Jint),
    pub get_static_long_field: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass, fieldID: JfieldID) -> Jlong),
    pub get_static_float_field: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass, fieldID: JfieldID) -> Jfloat),
    pub get_static_double_field: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass, fieldID: JfieldID) -> Jdouble),

    pub set_static_object_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        fieldID: JfieldID,
        value: Jobject
    ) -> !),
    pub set_static_boolean_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        fieldID: JfieldID,
        value: Jboolean
    ) -> !),
    pub set_static_byte_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        fieldID: JfieldID,
        value: Jbyte
    ) -> !),
    pub set_static_char_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        fieldID: JfieldID,
        value: Jchar
    ) -> !),
    pub set_static_short_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        fieldID: JfieldID,
        value: Jshort
    ) -> !),
    pub set_static_int_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        fieldID: JfieldID,
        value: Jint
    ) -> !),
    pub set_static_long_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        fieldID: JfieldID,
        value: Jlong
    ) -> !),
    pub set_static_float_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        fieldID: JfieldID,
        value: Jfloat
    ) -> !),
    pub set_static_double_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        fieldID: JfieldID,
        value: Jdouble
    ) -> !),

    pub new_string: unsafe_extern_system_fn!((env: *mut JNIEnv, unicode: *const Jchar, len: Jsize) -> Jstring),
    pub get_string_length: unsafe_extern_system_fn!((env: *mut JNIEnv, str: Jstring) -> Jsize),
    pub get_string_chars: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        str: Jstring,
        isCopy: *mut Jboolean
    ) -> *const Jchar),
    pub release_string_chars: unsafe_extern_system_fn!((env: *mut JNIEnv, str: Jstring, chars: *const Jchar) -> !),

    pub new_string_utf: unsafe_extern_system_fn!((env: *mut JNIEnv, utf: *const c_char) -> Jstring),
    pub get_string_utflength: unsafe_extern_system_fn!((env: *mut JNIEnv, str: Jstring) -> Jsize),
    pub get_string_utfchars: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        str: Jstring,
        isCopy: *mut Jboolean
    ) -> *const c_char),
    pub release_string_utfchars: unsafe_extern_system_fn!((env: *mut JNIEnv, str: Jstring, chars: *const c_char) -> !),

    pub get_array_length: unsafe_extern_system_fn!((env: *mut JNIEnv, array: Jarray) -> Jsize),

    pub new_object_array: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        len: Jsize,
        clazz: Jclass,
        init: Jobject
    ) -> JobjectArray),
    pub get_object_array_element: unsafe_extern_system_fn!((env: *mut JNIEnv, array: JobjectArray, index: Jsize) -> Jobject),
    pub set_object_array_element: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JobjectArray,
        index: Jsize,
        val: Jobject
    ) -> !),

    pub new_boolean_array: unsafe_extern_system_fn!((env: *mut JNIEnv, len: Jsize) -> JbooleanArray),
    pub new_byte_array: unsafe_extern_system_fn!((env: *mut JNIEnv, len: Jsize) -> JbyteArray),
    pub new_char_array: unsafe_extern_system_fn!((env: *mut JNIEnv, len: Jsize) -> JcharArray),
    pub new_short_array: unsafe_extern_system_fn!((env: *mut JNIEnv, len: Jsize) -> JshortArray),
    pub new_int_array: unsafe_extern_system_fn!((env: *mut JNIEnv, len: Jsize) -> JintArray),
    pub new_long_array: unsafe_extern_system_fn!((env: *mut JNIEnv, len: Jsize) -> JlongArray),
    pub new_float_array: unsafe_extern_system_fn!((env: *mut JNIEnv, len: Jsize) -> JfloatArray),
    pub new_double_array: unsafe_extern_system_fn!((env: *mut JNIEnv, len: Jsize) -> JdoubleArray),

    pub get_boolean_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JbooleanArray,
        isCopy: *mut Jboolean
    ) -> *mut Jboolean),
    pub get_byte_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JbyteArray,
        isCopy: *mut Jboolean
    ) -> *mut Jbyte),
    pub get_char_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JcharArray,
        isCopy: *mut Jboolean
    ) -> *mut Jchar),
    pub get_short_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JshortArray,
        isCopy: *mut Jboolean
    ) -> *mut Jshort),
    pub get_int_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JintArray,
        isCopy: *mut Jboolean
    ) -> *mut Jint),
    pub get_long_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JlongArray,
        isCopy: *mut Jboolean
    ) -> *mut Jlong),
    pub get_float_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JfloatArray,
        isCopy: *mut Jboolean
    ) -> *mut Jfloat),
    pub get_double_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JdoubleArray,
        isCopy: *mut Jboolean
    ) -> *mut Jdouble),

    pub release_boolean_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JbooleanArray,
        elems: *mut Jboolean,
        mode: Jint
    ) -> !),
    pub release_byte_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JbyteArray,
        elems: *mut Jbyte,
        mode: Jint
    ) -> !),
    pub release_char_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JcharArray,
        elems: *mut Jchar,
        mode: Jint
    ) -> !),
    pub release_short_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JshortArray,
        elems: *mut Jshort,
        mode: Jint
    ) -> !),
    pub release_int_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JintArray,
        elems: *mut Jint,
        mode: Jint
    ) -> !),
    pub release_long_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JlongArray,
        elems: *mut Jlong,
        mode: Jint
    ) -> !),
    pub release_float_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JfloatArray,
        elems: *mut Jfloat,
        mode: Jint
    ) -> !),
    pub release_double_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JdoubleArray,
        elems: *mut Jdouble,
        mode: Jint
    ) -> !),

    pub get_boolean_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JbooleanArray,
        start: Jsize,
        len: Jsize,
        buf: *mut Jboolean
    ) -> !),
    pub get_byte_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JbyteArray,
        start: Jsize,
        len: Jsize,
        buf: *mut Jbyte
    ) -> !),
    pub get_char_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JcharArray,
        start: Jsize,
        len: Jsize,
        buf: *mut Jchar
    ) -> !),
    pub get_short_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JshortArray,
        start: Jsize,
        len: Jsize,
        buf: *mut Jshort
    ) -> !),
    pub get_int_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JintArray,
        start: Jsize,
        len: Jsize,
        buf: *mut Jint
    ) -> !),
    pub get_long_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JlongArray,
        start: Jsize,
        len: Jsize,
        buf: *mut Jlong
    ) -> !),
    pub get_float_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JfloatArray,
        start: Jsize,
        len: Jsize,
        buf: *mut Jfloat
    ) -> !),
    pub get_double_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JdoubleArray,
        start: Jsize,
        len: Jsize,
        buf: *mut Jdouble
    ) -> !),

    pub set_boolean_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JbooleanArray,
        start: Jsize,
        len: Jsize,
        buf: *const Jboolean
    ) -> !),
    pub set_byte_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JbyteArray,
        start: Jsize,
        len: Jsize,
        buf: *const Jbyte
    ) -> !),
    pub set_char_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JcharArray,
        start: Jsize,
        len: Jsize,
        buf: *const Jchar
    ) -> !),
    pub set_short_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JshortArray,
        start: Jsize,
        len: Jsize,
        buf: *const Jshort
    ) -> !),
    pub set_int_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JintArray,
        start: Jsize,
        len: Jsize,
        buf: *const Jint
    ) -> !),
    pub set_long_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JlongArray,
        start: Jsize,
        len: Jsize,
        buf: *const Jlong
    ) -> !),
    pub set_float_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JfloatArray,
        start: Jsize,
        len: Jsize,
        buf: *const Jfloat
    ) -> !),
    pub set_double_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JdoubleArray,
        start: Jsize,
        len: Jsize,
        buf: *const Jdouble
    ) -> !),

    pub register_natives: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methods: *const JNINativeMethod,
        nMethods: Jint
    ) -> Jint),
    pub unregister_natives: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass) -> Jint),

    pub monitor_enter: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject) -> Jint),
    pub monitor_exit: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject) -> Jint),

    pub get_java_vm: unsafe_extern_system_fn!((env: *mut JNIEnv, vm: *mut *mut JavaVM) -> Jint),

    pub get_string_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        str: Jstring,
        start: Jsize,
        len: Jsize,
        buf: *mut Jchar
    ) -> !),
    pub get_string_utfregion: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        str: Jstring,
        start: Jsize,
        len: Jsize,
        buf: *mut c_char
    ) -> !),

    pub get_primitive_array_critical: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: Jarray,
        isCopy: *mut Jboolean
    ) -> *mut c_void),
    pub release_primitive_array_critical: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: Jarray,
        carray: *mut c_void,
        mode: Jint
    ) -> !),

    pub get_string_critical: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        string: Jstring,
        isCopy: *mut Jboolean
    ) -> *const Jchar),
    pub release_string_critical: unsafe_extern_system_fn!((env: *mut JNIEnv, string: Jstring, cstring: *const Jchar) -> !),

    pub new_weak_global_ref: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject) -> Jweak),
    pub delete_weak_global_ref: unsafe_extern_system_fn!((env: *mut JNIEnv, gref: Jweak) -> !),

    pub exception_check: unsafe_extern_system_fn!((env: *mut JNIEnv) -> Jboolean),

    pub new_direct_byte_buffer: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        address: *mut c_void,
        capacity: Jlong
    ) -> Jobject),
    pub get_direct_buffer_address: unsafe_extern_system_fn!((env: *mut JNIEnv, buf: Jobject) -> *mut c_void),
    pub get_direct_buffer_capacity: unsafe_extern_system_fn!((env: *mut JNIEnv, buf: Jobject) -> Jlong),

    /// New JNI 1.6 Features
    ///
    /// `JNI_VERSION` >= `JNI_VERSION_1_6` can be used normally
    pub get_object_ref_type: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject) -> JobjectRefType),
    /// Module Features
    ///
    /// `JNI_VERSION` >= `JNI_VERSION_9` can be used normally
    pub get_module: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass) -> Jobject),
}

/// JNI invocation interface
#[repr(C)]
pub struct JNIInvokeInterface {
    reserved0: *mut c_void,
    reserved1: *mut c_void,
    reserved2: *mut c_void,

    pub destroy_java_vm: unsafe_extern_system_fn!((vm: *mut JavaVM) -> Jint),

    pub attach_current_thread: unsafe_extern_system_fn!((
        vm: *mut JavaVM,
        penv: *mut *mut c_void,
        args: *mut c_void
    ) -> Jint),

    pub detach_current_thread: unsafe_extern_system_fn!((vm: *mut JavaVM) -> Jint),

    pub get_env: unsafe_extern_system_fn!((vm: *mut JavaVM, penv: *mut *mut c_void, version: Jint) -> Jint),

    pub attach_current_thread_as_daemon: unsafe_extern_system_fn!((
        vm: *mut JavaVM,
        penv: *mut *mut c_void,
        args: *mut c_void
    ) -> Jint),
}

#[repr(C)]
pub struct JavaVMAttachArgs {
    version: Jint,       /* must be >= JNI_VERSION_1_1 */
    name: *const c_char, /* NULL or name of thread as modified UTF-8 str */
    group: Jobject,      /* global ref of a ThreadGroup object, or NULL */
}

/// JNI 1.2+ initialization (As of 1.6, the pre-1.2 structures are no longer supported)
#[repr(C)]
pub struct JavaVMOption {
    option_string: *const c_char,
    extra_info: *mut c_void,
}

#[repr(C)]
pub struct JavaVMInitArgs {
    version: Jint, /* use JNI_VERSION_1_1 or later */

    n_options: Jint,
    options: *mut JavaVMOption,
    ignore_unrecognized: Jboolean,
}

extern "system" {
    /*
     * VM initialization functions
     *
     * Note these are the only symbols exported for JNI by the VM
     */
    pub fn JNI_GetDefaultJavaVMInitArgs(args: *mut c_void) -> Jint;
    pub fn JNI_CreateJavaVM(
        pvm: *mut *mut JavaVM,
        penv: *mut *mut c_void,
        args: *mut c_void,
    ) -> Jint;
    pub fn JNI_GetCreatedJavaVMs(
        vm_buf: *mut *mut JavaVM,
        buf_len: Jsize,
        num_vms: *mut Jsize,
    ) -> Jint;
}

/*
 * Manifest constants
 */

pub const JNI_FALSE: Jboolean = 0;
pub const JNI_TRUE: Jboolean = 1;

pub const JNI_VERSION_1_1: Jint = 0x00010001;
pub const JNI_VERSION_1_2: Jint = 0x00010002;
pub const JNI_VERSION_1_4: Jint = 0x00010004;
pub const JNI_VERSION_1_6: Jint = 0x00010006;
pub const JNI_VERSION_1_8: Jint = 0x00010008;
pub const JNI_VERSION_9: Jint = 0x00090000;
pub const JNI_VERSION_10: Jint = 0x000a0000;

/*
 * possible return values for JNI functions
 */

/// no error
pub const JNI_OK: Jint = 0;
/// generic error
pub const JNI_ERR: Jint = -1;
/// thread detached from the VM
pub const JNI_EDETACHED: Jint = -2;
/// JNI version error
pub const JNI_EVERSION: Jint = -3;
/// Out of memory
pub const JNI_ENOMEM: Jint = -4;
/// VM already created
pub const JNI_EEXIST: Jint = -5;
/// Invalid argument
pub const JNI_EINVAL: Jint = -6;

/*
 * used in ReleaseScalarArrayElements
 */

/// copy content, do not free buffer
pub const JNI_COMMIT: Jint = 1;
/// free buffer w/o copying back
pub const JNI_ABORT: Jint = 2;
