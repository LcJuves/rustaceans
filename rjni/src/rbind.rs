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
#[macro_export(local_inner_macros)]
macro_rules! unsafe_extern_system_fn {
    (($($param_name:ident: $param_ty:ty), *)$( -> $ret_ty:ty)?) => {
        Option<unsafe extern "system" fn($($param_name: $param_ty), *)$( -> $ret_ty)?>
    };
}

/// # Examples
///
/// ```rust
/// unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, method_id: JmethodID) -> Jobject)
/// ```
/// expand to
/// ```rust
/// Option<unsafe extern "C" fn(env: *mut JNIEnv, clazz: Jclass, method_id: JmethodID, ...) -> Jobject>
/// ```
#[macro_export(local_inner_macros)]
macro_rules! unsafe_extern_c_var_fn {
    (($($param_name:ident: $param_ty:ty), *)$( -> $ret_ty:ty)?) => {
        Option<unsafe extern "C" fn($($param_name: $param_ty, )*...)$( -> $ret_ty)?>
    };
}

/// # Such as
///
/// ```rust
/// jni_fn!(
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
#[macro_export(local_inner_macros)]
macro_rules! jni_fn {
    ($name:tt, ($($ident:tt: $ty:ty), +),$( $ret_ty:ty,)? $code:block) => {
        #[no_mangle]
        pub extern "system" fn $name($($ident: $ty, )+)$( -> $ret_ty)? $code
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
#[macro_export(local_inner_macros)]
macro_rules! impl_jni_on_load {
    ($param_vm_name:tt, $param_reserved_name:tt, $code:block) => {
        jni_fn!(
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
#[macro_export(local_inner_macros)]
macro_rules! impl_jni_on_unload {
    ($param_vm_name:tt, $param_reserved_name:tt, $code:block) => {
        jni_fn!(
            JNI_OnUnload,
            ($param_vm_name: *mut JavaVM, $param_reserved_name: *mut c_void),
            (),
            $code
        );
    };
}

#[macro_export(local_inner_macros)]
macro_rules! env_call {
    ($env:expr, $method_name:ident$(, $($arg:expr), *)?) => {
        (|| {
            unsafe { (*(*$env)).$method_name.unwrap()($env$(, $($arg, )*)?) }
        })()
    };
}

#[macro_export(local_inner_macros)]
macro_rules! vm_call {
    ($vm:expr, $method_name:ident$(, $($arg:expr), *)?) => {
        (|| {
            unsafe { (*(*$vm)).$method_name.unwrap()($vm$(, $($arg, )*)?) }
        })()
    };
}

#[macro_export(local_inner_macros)]
macro_rules! char_const_ptr {
    ($str:expr) => {
        (|| CString::new($str).unwrap().into_raw())()
    };
}

#[macro_export(local_inner_macros)]
macro_rules! jstring {
    ($env:expr, $str:expr) => {
        (|| env_call!($env, new_string_utf, char_const_ptr!($str)))()
    };
}

#[macro_export(local_inner_macros)]
macro_rules! env_from_vm {
    ($vm:expr) => {
        (|| {
            let mut env = core::ptr::null_mut::<c_void>();
            core::assert!(
                vm_call!($vm, get_env, &mut env as *mut *mut c_void, JNI_VERSION_1_1) == JNI_OK
            );
            env as *mut JNIEnv
        })()
    };
}

use std::os::raw::{c_char, c_void};

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
/// 32 bits
pub type Jfloat = f32;
/// 64 bits
pub type Jdouble = f64;

/// The [`Jsize`] integer type is used to describe cardinal indices and sizes.
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

/// When passing arguments from Rust to a Java method, the [`Jvalue`] union is used.
/// The [`Jvalue`] union type is used as the element type in argument arrays.
#[repr(C)]
pub union Jvalue {
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
    pub name: *mut c_char,
    pub signature: *mut c_char,
    pub fn_ptr: *mut c_void,
}

/// [`JNIEnv`] implements the "Java Native Inferface", and contains most of what you'll use to interact with Java from Rust
pub type JNIEnv = *const JNINativeInterface;
/// [`JavaVM`] (along with a handful of global functions) implements the "Java Invocation Interface",
/// which allow you to create and destroy a Java Virtual Machine
pub type JavaVM = *const JNIInvokeInterface;

/// Table of interface function pointers
#[repr(C)]
pub struct JNINativeInterface {
    reserved0: *mut c_void,
    reserved1: *mut c_void,
    reserved2: *mut c_void,

    reserved3: *mut c_void,
    /// Returns the version of the native method interface. For Java SE Platform 10 and later, it returns [`JNI_VERSION_10`].
    pub get_version: unsafe_extern_system_fn!((env: *mut JNIEnv) -> Jint),

    /// Loads a class from a buffer of raw class data. The buffer containing the raw class data is not referenced by the VM after the `define_class` call returns, and it may be discarded if desired.
    pub define_class: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        name: *const c_char,
        loader: Jobject,
        buf: *const Jbyte,
        buf_len: Jsize
    ) -> Jclass),
    /// In JDK release 1.1, this function loads a locally-defined class. It searches the directories and zip files specified by the `CLASSPATH` environment variable for the class with the specified name.
    ///
    /// Since JDK 1.2, the Java security model allows non-system classes to load and call native methods. `find_class` locates the class loader associated with the current native method; that is, the class loader of the class
    /// that declared the native method. If the native method belongs to a system class, no class loader will be involved. Otherwise, the proper class loader will be invoked to load, link, and initialize, the named class.
    ///
    /// Since JDK 1.2, when `find_class` is called through the Invocation Interface, there is no current native method or its associated class loader. In that case, the result of [ClassLoader.getSystemClassLoader] is used. This
    /// is the class loader the virtual machine creates for applications, and is able to locate classes listed in the `java.class.path` property.
    ///
    /// [ClassLoader.getSystemClassLoader]: https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/lang/ClassLoader.html#getSystemClassLoader()
    pub find_class: unsafe_extern_system_fn!((env: *mut JNIEnv, name: *const c_char) -> Jclass),

    /// Converts a [java.lang.reflect.Method] or [java.lang.reflect.Constructor] object to a method ID.
    ///
    /// [java.lang.reflect.Method]: https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/lang/reflect/Method.html
    /// [java.lang.reflect.Constructor]: https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/lang/reflect/Constructor.html
    pub from_reflected_method: unsafe_extern_system_fn!((env: *mut JNIEnv, method: Jobject) -> JmethodID),
    /// Converts a [java.lang.reflect.Field] to a field ID.
    ///
    /// [java.lang.reflect.Field]: https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/lang/reflect/Field.html
    pub from_reflected_field: unsafe_extern_system_fn!((env: *mut JNIEnv, field: Jobject) -> JfieldID),

    /// Converts a method ID derived from `cls` to a [java.lang.reflect.Method] or [java.lang.reflect.Constructor] object. `is_static` must be set to [`JNI_TRUE`] if the method ID refers to a static field, and [`JNI_FALSE`]
    /// otherwise.
    ///
    /// Throws [OutOfMemoryError] and returns 0 if fails.
    ///
    /// [java.lang.reflect.Method]: https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/lang/reflect/Method.html
    /// [java.lang.reflect.Constructor]: https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/lang/reflect/Constructor.html
    /// [OutOfMemoryError]: https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/lang/OutOfMemoryError.html
    pub to_reflected_method: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        cls: Jclass,
        method_id: JmethodID,
        is_static: Jboolean
    ) -> Jobject),

    /// If `clazz` represents any class other than the class [Object], then this function returns the object that represents the superclass of the class specified by `clazz`.
    ///
    /// If `clazz` specifies the class [Object], or `clazz` represents an interface, this function returns `NULL`.
    ///
    /// [Object]: https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/lang/Object.html
    pub get_superclass: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass) -> Jclass),
    /// Determines whether an object of `clazz1` can be safely cast to `clazz2`.
    pub is_assignable_from: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz1: Jclass, clazz2: Jclass) -> Jboolean),

    /// Converts a field ID derived from `cls` to a [java.lang.reflect.Field] object. `is_static` must be set to [`JNI_TRUE`] if `field_id` refers to a static field, and [`JNI_FALSE`] otherwise.
    ///
    /// Throws [OutOfMemoryError] and returns 0 if fails.
    ///
    /// [java.lang.reflect.Field]: https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/lang/reflect/Field.html
    /// [OutOfMemoryError]: https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/lang/OutOfMemoryError.html
    pub to_reflected_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        cls: Jclass,
        field_id: JfieldID,
        is_static: Jboolean
    ) -> Jobject),

    /// Causes a [java.lang.Throwable] object to be thrown.
    ///
    /// [java.lang.Throwable]: https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/lang/Throwable.html
    pub throw: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jthrowable) -> Jint),
    /// Constructs an exception object from the specified class with the message specified by `message` and causes that exception to be thrown.
    pub throw_new: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass, message: *const c_char) -> Jint),
    /// Determines if an exception is being thrown. The exception stays being thrown until either the native code calls `exception_clear`, or the Java code handles the exception.
    pub exception_occurred: unsafe_extern_system_fn!((env: *mut JNIEnv) -> Jthrowable),
    /// Prints an exception and a backtrace of the stack to a system error-reporting channel, such as `stderr`. The pending exception is cleared as a side-effect of calling this function. This is a convenience routine provided for
    /// debugging.
    pub exception_describe: unsafe_extern_system_fn!((env: *mut JNIEnv)),
    /// Clears any exception that is currently being thrown. If no exception is currently being thrown, this routine has no effect.
    pub exception_clear: unsafe_extern_system_fn!((env: *mut JNIEnv)),
    /// Raises a fatal error and does not expect the VM to recover. This function does not return.
    pub fatal_error: unsafe_extern_system_fn!((env: *mut JNIEnv, msg: *const c_char)),

    /// Creates a new local reference frame, in which at least a given number of local references can be created. Returns 0 on success, a negative number and a pending [OutOfMemoryError] on failure.
    ///
    /// Note that local references already created in previous local frames are still valid in the current local frame.
    ///
    /// As with `ensure_local_capacity`, some Java Virtual Machine implementations may choose to limit the maximum `capacity`, which may cause the function to return an error.
    ///
    /// [OutOfMemoryError]: https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/lang/OutOfMemoryError.html
    pub push_local_frame: unsafe_extern_system_fn!((env: *mut JNIEnv, capacity: Jint) -> Jint),
    /// Pops off the current local reference frame, frees all the local references, and returns a local reference in the previous local reference frame for the given `result` object.
    ///
    /// Pass `NULL` as `result` if you do not need to return a reference to the previous frame.
    pub pop_local_frame: unsafe_extern_system_fn!((env: *mut JNIEnv, result: Jobject) -> Jobject),

    pub new_global_ref: unsafe_extern_system_fn!((env: *mut JNIEnv, gref: Jobject) -> Jobject),
    pub delete_global_ref: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject)),
    pub delete_local_ref: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject)),
    pub is_same_object: unsafe_extern_system_fn!((env: *mut JNIEnv, obj1: Jobject, obj2: Jobject) -> Jboolean),
    pub new_local_ref: unsafe_extern_system_fn!((env: *mut JNIEnv, lref: Jobject) -> Jobject),
    pub ensure_local_capacity: unsafe_extern_system_fn!((env: *mut JNIEnv, capacity: Jint) -> Jint),

    pub alloc_object: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass) -> Jobject),
    pub new_object: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, method_id: JmethodID) -> Jobject),
    pub new_object_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        method_id: JmethodID,
        args: VaList
    ) -> Jobject),
    pub new_object_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jobject),

    pub get_object_class: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject) -> Jclass),
    pub is_instance_of: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, clazz: Jclass) -> Jboolean),

    pub get_method_id: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        name: *const c_char,
        sig: *const c_char
    ) -> JmethodID),

    pub call_object_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: Jobject, method_id: JmethodID) -> Jobject),
    pub call_object_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        method_id: JmethodID,
        args: VaList
    ) -> Jobject),
    pub call_object_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jobject),

    pub call_boolean_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: Jobject, method_id: JmethodID) -> Jboolean),
    pub call_boolean_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        method_id: JmethodID,
        args: VaList
    ) -> Jboolean),
    pub call_boolean_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jboolean),

    pub call_byte_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: Jobject, method_id: JmethodID) -> Jbyte),
    pub call_byte_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        method_id: JmethodID,
        args: VaList
    ) -> Jbyte),
    pub call_byte_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jbyte),

    pub call_char_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: Jobject, method_id: JmethodID) -> Jchar),
    pub call_char_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        method_id: JmethodID,
        args: VaList
    ) -> Jchar),
    pub call_char_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jchar),

    pub call_short_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: Jobject, method_id: JmethodID) -> Jshort),
    pub call_short_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        method_id: JmethodID,
        args: VaList
    ) -> Jshort),
    pub call_short_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jshort),

    pub call_int_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: Jobject, method_id: JmethodID) -> Jint),
    pub call_int_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        method_id: JmethodID,
        args: VaList
    ) -> Jint),
    pub call_int_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jint),

    pub call_long_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: Jobject, method_id: JmethodID) -> Jlong),
    pub call_long_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        method_id: JmethodID,
        args: VaList
    ) -> Jlong),
    pub call_long_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jlong),

    pub call_float_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: Jobject, method_id: JmethodID) -> Jfloat),
    pub call_float_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        method_id: JmethodID,
        args: VaList
    ) -> Jfloat),
    pub call_float_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jfloat),

    pub call_double_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: Jobject, method_id: JmethodID) -> Jdouble),
    pub call_double_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        method_id: JmethodID,
        args: VaList
    ) -> Jdouble),
    pub call_double_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jdouble),

    pub call_void_method:
        unsafe_extern_c_var_fn!((env: *mut JNIEnv, obj: Jobject, method_id: JmethodID)),
    pub call_void_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        method_id: JmethodID,
        args: VaList
    )),
    pub call_void_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        method_id: JmethodID,
        args: *const Jvalue
    )),

    pub call_nonvirtual_object_method: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID) -> Jobject),
    pub call_nonvirtual_object_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID,
        args: VaList
    ) -> Jobject),
    pub call_nonvirtual_object_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jobject),

    pub call_nonvirtual_boolean_method: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID) -> Jboolean),
    pub call_nonvirtual_boolean_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID,
        args: VaList
    ) -> Jboolean),
    pub call_nonvirtual_boolean_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jboolean),

    pub call_nonvirtual_byte_method: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID) -> Jbyte),
    pub call_nonvirtual_byte_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID,
        args: VaList
    ) -> Jbyte),
    pub call_nonvirtual_byte_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jbyte),

    pub call_nonvirtual_char_method: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID) -> Jchar),
    pub call_nonvirtual_char_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID,
        args: VaList
    ) -> Jchar),
    pub call_nonvirtual_char_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jchar),

    pub call_nonvirtual_short_method: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID) -> Jshort),
    pub call_nonvirtual_short_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID,
        args: VaList
    ) -> Jshort),
    pub call_nonvirtual_short_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jshort),

    pub call_nonvirtual_int_method: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID) -> Jint),
    pub call_nonvirtual_int_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID,
        args: VaList
    ) -> Jint),
    pub call_nonvirtual_int_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jint),

    pub call_nonvirtual_long_method: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID) -> Jlong),
    pub call_nonvirtual_long_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID,
        args: VaList
    ) -> Jlong),
    pub call_nonvirtual_long_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jlong),

    pub call_nonvirtual_float_method: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID) -> Jfloat),
    pub call_nonvirtual_float_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID,
        args: VaList
    ) -> Jfloat),
    pub call_nonvirtual_float_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jfloat),

    pub call_nonvirtual_double_method: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID) -> Jdouble),
    pub call_nonvirtual_double_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID,
        args: VaList
    ) -> Jdouble),
    pub call_nonvirtual_double_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jdouble),

    pub call_nonvirtual_void_method: unsafe_extern_c_var_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID
    )),
    pub call_nonvirtual_void_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID,
        args: VaList
    )),
    pub call_nonvirtual_void_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        clazz: Jclass,
        method_id: JmethodID,
        args: *const Jvalue
    )),

    pub get_field_id: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        name: *const c_char,
        sig: *const c_char
    ) -> JfieldID),

    pub get_object_field: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, field_id: JfieldID) -> Jobject),
    pub get_boolean_field: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, field_id: JfieldID) -> Jboolean),
    pub get_byte_field: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, field_id: JfieldID) -> Jbyte),
    pub get_char_field: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, field_id: JfieldID) -> Jchar),
    pub get_short_field: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, field_id: JfieldID) -> Jshort),
    pub get_int_field: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, field_id: JfieldID) -> Jint),
    pub get_long_field: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, field_id: JfieldID) -> Jlong),
    pub get_float_field: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, field_id: JfieldID) -> Jfloat),
    pub get_double_field: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, field_id: JfieldID) -> Jdouble),

    pub set_object_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        field_id: JfieldID,
        val: Jobject
    )),
    pub set_boolean_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        field_id: JfieldID,
        val: Jboolean
    )),
    pub set_byte_field:
        unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, field_id: JfieldID, val: Jbyte)),
    pub set_char_field:
        unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, field_id: JfieldID, val: Jchar)),
    pub set_short_field:
        unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, field_id: JfieldID, val: Jshort)),
    pub set_int_field:
        unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, field_id: JfieldID, val: Jint)),
    pub set_long_field:
        unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, field_id: JfieldID, val: Jlong)),
    pub set_float_field:
        unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject, field_id: JfieldID, val: Jfloat)),
    pub set_double_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        obj: Jobject,
        field_id: JfieldID,
        val: Jdouble
    )),

    pub get_static_method_id: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        name: *const c_char,
        sig: *const c_char
    ) -> JmethodID),

    pub call_static_object_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, method_id: JmethodID) -> Jobject),
    pub call_static_object_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        method_id: JmethodID,
        args: VaList
    ) -> Jobject),
    pub call_static_object_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jobject),

    pub call_static_boolean_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, method_id: JmethodID) -> Jboolean),
    pub call_static_boolean_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        method_id: JmethodID,
        args: VaList
    ) -> Jboolean),
    pub call_static_boolean_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jboolean),

    pub call_static_byte_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, method_id: JmethodID) -> Jbyte),
    pub call_static_byte_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        method_id: JmethodID,
        args: VaList
    ) -> Jbyte),
    pub call_static_byte_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jbyte),

    pub call_static_char_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, method_id: JmethodID) -> Jchar),
    pub call_static_char_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        method_id: JmethodID,
        args: VaList
    ) -> Jchar),
    pub call_static_char_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jchar),

    pub call_static_short_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, method_id: JmethodID) -> Jshort),
    pub call_static_short_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        method_id: JmethodID,
        args: VaList
    ) -> Jshort),
    pub call_static_short_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jshort),

    pub call_static_int_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, method_id: JmethodID) -> Jint),
    pub call_static_int_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        method_id: JmethodID,
        args: VaList
    ) -> Jint),
    pub call_static_int_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jint),

    pub call_static_long_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, method_id: JmethodID) -> Jlong),
    pub call_static_long_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        method_id: JmethodID,
        args: VaList
    ) -> Jlong),
    pub call_static_long_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jlong),

    pub call_static_float_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, method_id: JmethodID) -> Jfloat),
    pub call_static_float_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        method_id: JmethodID,
        args: VaList
    ) -> Jfloat),
    pub call_static_float_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jfloat),

    pub call_static_double_method: unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, method_id: JmethodID) -> Jdouble),
    pub call_static_double_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        method_id: JmethodID,
        args: VaList
    ) -> Jdouble),
    pub call_static_double_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        method_id: JmethodID,
        args: *const Jvalue
    ) -> Jdouble),

    pub call_static_void_method:
        unsafe_extern_c_var_fn!((env: *mut JNIEnv, clazz: Jclass, method_id: JmethodID)),
    pub call_static_void_method_v: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        method_id: JmethodID,
        args: VaList
    )),
    pub call_static_void_method_a: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        method_id: JmethodID,
        args: *const Jvalue
    )),

    pub get_static_field_id: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        name: *const c_char,
        sig: *const c_char
    ) -> JfieldID),
    pub get_static_object_field: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass, field_id: JfieldID) -> Jobject),
    pub get_static_boolean_field: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass, field_id: JfieldID) -> Jboolean),
    pub get_static_byte_field: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass, field_id: JfieldID) -> Jbyte),
    pub get_static_char_field: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass, field_id: JfieldID) -> Jchar),
    pub get_static_short_field: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass, field_id: JfieldID) -> Jshort),
    pub get_static_int_field: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass, field_id: JfieldID) -> Jint),
    pub get_static_long_field: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass, field_id: JfieldID) -> Jlong),
    pub get_static_float_field: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass, field_id: JfieldID) -> Jfloat),
    pub get_static_double_field: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass, field_id: JfieldID) -> Jdouble),

    pub set_static_object_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        field_id: JfieldID,
        value: Jobject
    )),
    pub set_static_boolean_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        field_id: JfieldID,
        value: Jboolean
    )),
    pub set_static_byte_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        field_id: JfieldID,
        value: Jbyte
    )),
    pub set_static_char_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        field_id: JfieldID,
        value: Jchar
    )),
    pub set_static_short_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        field_id: JfieldID,
        value: Jshort
    )),
    pub set_static_int_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        field_id: JfieldID,
        value: Jint
    )),
    pub set_static_long_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        field_id: JfieldID,
        value: Jlong
    )),
    pub set_static_float_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        field_id: JfieldID,
        value: Jfloat
    )),
    pub set_static_double_field: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        field_id: JfieldID,
        value: Jdouble
    )),

    pub new_string: unsafe_extern_system_fn!((env: *mut JNIEnv, unicode: *const Jchar, len: Jsize) -> Jstring),
    pub get_string_length: unsafe_extern_system_fn!((env: *mut JNIEnv, str: Jstring) -> Jsize),
    pub get_string_chars: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        str: Jstring,
        is_copy: *mut Jboolean
    ) -> *const Jchar),
    pub release_string_chars:
        unsafe_extern_system_fn!((env: *mut JNIEnv, str: Jstring, chars: *const Jchar)),

    pub new_string_utf: unsafe_extern_system_fn!((env: *mut JNIEnv, utf: *const c_char) -> Jstring),
    pub get_string_utflength: unsafe_extern_system_fn!((env: *mut JNIEnv, str: Jstring) -> Jsize),
    pub get_string_utfchars: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        str: Jstring,
        is_copy: *mut Jboolean
    ) -> *const c_char),
    pub release_string_utfchars:
        unsafe_extern_system_fn!((env: *mut JNIEnv, str: Jstring, chars: *const c_char)),

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
    )),

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
        is_copy: *mut Jboolean
    ) -> *mut Jboolean),
    pub get_byte_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JbyteArray,
        is_copy: *mut Jboolean
    ) -> *mut Jbyte),
    pub get_char_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JcharArray,
        is_copy: *mut Jboolean
    ) -> *mut Jchar),
    pub get_short_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JshortArray,
        is_copy: *mut Jboolean
    ) -> *mut Jshort),
    pub get_int_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JintArray,
        is_copy: *mut Jboolean
    ) -> *mut Jint),
    pub get_long_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JlongArray,
        is_copy: *mut Jboolean
    ) -> *mut Jlong),
    pub get_float_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JfloatArray,
        is_copy: *mut Jboolean
    ) -> *mut Jfloat),
    pub get_double_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JdoubleArray,
        is_copy: *mut Jboolean
    ) -> *mut Jdouble),

    pub release_boolean_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JbooleanArray,
        elems: *mut Jboolean,
        mode: Jint
    )),
    pub release_byte_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JbyteArray,
        elems: *mut Jbyte,
        mode: Jint
    )),
    pub release_char_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JcharArray,
        elems: *mut Jchar,
        mode: Jint
    )),
    pub release_short_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JshortArray,
        elems: *mut Jshort,
        mode: Jint
    )),
    pub release_int_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JintArray,
        elems: *mut Jint,
        mode: Jint
    )),
    pub release_long_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JlongArray,
        elems: *mut Jlong,
        mode: Jint
    )),
    pub release_float_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JfloatArray,
        elems: *mut Jfloat,
        mode: Jint
    )),
    pub release_double_array_elements: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JdoubleArray,
        elems: *mut Jdouble,
        mode: Jint
    )),

    pub get_boolean_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JbooleanArray,
        start: Jsize,
        len: Jsize,
        buf: *mut Jboolean
    )),
    pub get_byte_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JbyteArray,
        start: Jsize,
        len: Jsize,
        buf: *mut Jbyte
    )),
    pub get_char_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JcharArray,
        start: Jsize,
        len: Jsize,
        buf: *mut Jchar
    )),
    pub get_short_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JshortArray,
        start: Jsize,
        len: Jsize,
        buf: *mut Jshort
    )),
    pub get_int_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JintArray,
        start: Jsize,
        len: Jsize,
        buf: *mut Jint
    )),
    pub get_long_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JlongArray,
        start: Jsize,
        len: Jsize,
        buf: *mut Jlong
    )),
    pub get_float_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JfloatArray,
        start: Jsize,
        len: Jsize,
        buf: *mut Jfloat
    )),
    pub get_double_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JdoubleArray,
        start: Jsize,
        len: Jsize,
        buf: *mut Jdouble
    )),

    pub set_boolean_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JbooleanArray,
        start: Jsize,
        len: Jsize,
        buf: *const Jboolean
    )),
    pub set_byte_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JbyteArray,
        start: Jsize,
        len: Jsize,
        buf: *const Jbyte
    )),
    pub set_char_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JcharArray,
        start: Jsize,
        len: Jsize,
        buf: *const Jchar
    )),
    pub set_short_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JshortArray,
        start: Jsize,
        len: Jsize,
        buf: *const Jshort
    )),
    pub set_int_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JintArray,
        start: Jsize,
        len: Jsize,
        buf: *const Jint
    )),
    pub set_long_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JlongArray,
        start: Jsize,
        len: Jsize,
        buf: *const Jlong
    )),
    pub set_float_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JfloatArray,
        start: Jsize,
        len: Jsize,
        buf: *const Jfloat
    )),
    pub set_double_array_region: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: JdoubleArray,
        start: Jsize,
        len: Jsize,
        buf: *const Jdouble
    )),

    pub register_natives: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        clazz: Jclass,
        methods: *const JNINativeMethod,
        n_methods: Jint
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
    )),
    pub get_string_utfregion: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        str: Jstring,
        start: Jsize,
        len: Jsize,
        buf: *mut c_char
    )),

    pub get_primitive_array_critical: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: Jarray,
        is_copy: *mut Jboolean
    ) -> *mut c_void),
    pub release_primitive_array_critical: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        array: Jarray,
        carray: *mut c_void,
        mode: Jint
    )),

    pub get_string_critical: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        string: Jstring,
        is_copy: *mut Jboolean
    ) -> *const Jchar),
    pub release_string_critical:
        unsafe_extern_system_fn!((env: *mut JNIEnv, string: Jstring, cstring: *const Jchar)),

    pub new_weak_global_ref: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject) -> Jweak),
    pub delete_weak_global_ref: unsafe_extern_system_fn!((env: *mut JNIEnv, gref: Jweak)),

    pub exception_check: unsafe_extern_system_fn!((env: *mut JNIEnv) -> Jboolean),

    pub new_direct_byte_buffer: unsafe_extern_system_fn!((
        env: *mut JNIEnv,
        address: *mut c_void,
        capacity: Jlong
    ) -> Jobject),
    pub get_direct_buffer_address: unsafe_extern_system_fn!((env: *mut JNIEnv, buf: Jobject) -> *mut c_void),
    /// Fetches and returns the capacity of the memory region referenced by the given direct [java.nio.Buffer]. The capacity is the number of *elements* that the memory region contains.
    ///
    /// [java.nio.Buffer]: https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/nio/Buffer.html
    pub get_direct_buffer_capacity: unsafe_extern_system_fn!((env: *mut JNIEnv, buf: Jobject) -> Jlong),

    /// Returns the type of the object referred to by the `obj` argument. The argument `obj` can either be a local, global or weak global reference, or `NULL`.
    ///
    /// `JNI_VERSION` >= [`JNI_VERSION_1_6`] can be used normally
    pub get_object_ref_type: unsafe_extern_system_fn!((env: *mut JNIEnv, obj: Jobject) -> JobjectRefType),
    /// Returns the [java.lang.Module] object for the module that the class is a member of. If the class is not in a named module then the unnamed module of the class loader for the class is returned. If the class represents
    /// an array type then this function returns the [Module] object for the element type. If the class represents a primitive type or `void`, then the [Module] object for the [java.base] module is returned.
    ///
    /// `JNI_VERSION` >= [`JNI_VERSION_9`] can be used normally
    ///
    /// [java.lang.Module]: https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/lang/Module.html
    /// [Module]: https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/lang/Module.html
    /// [java.base]: https://docs.oracle.com/en/java/javase/17/docs/api/java.base/module-summary.html
    pub get_module: unsafe_extern_system_fn!((env: *mut JNIEnv, clazz: Jclass) -> Jobject),
}

/// JNI invocation interface
#[repr(C)]
pub struct JNIInvokeInterface {
    reserved0: *mut c_void,
    reserved1: *mut c_void,
    reserved2: *mut c_void,

    /// Unloads a Java VM and reclaims its resources.
    ///
    /// Any thread, whether attached or not, can invoke this function. If the current thread is attached, the VM waits until the current thread is the only non-daemon user-level Java thread. If the current thread is not
    /// attached, the VM attaches the current thread and then waits until the current thread is the only non-daemon user-level thread.
    pub destroy_java_vm: unsafe_extern_system_fn!((vm: *mut JavaVM) -> Jint),

    /// Attaches the current thread to a Java VM. Returns a JNI interface pointer in the [`JNIEnv`] argument.
    ///
    /// Trying to attach a thread that is already attached is a no-op.
    ///
    /// A native thread cannot be attached simultaneously to two Java VMs.
    ///
    /// When a thread is attached to the VM, the context class loader is the bootstrap loader.
    pub attach_current_thread: unsafe_extern_system_fn!((
        vm: *mut JavaVM,
        p_env: *mut *mut c_void,
        thr_args: *mut c_void
    ) -> Jint),

    /// Detaches the current thread from a Java VM. All Java monitors held by this thread are released. All Java threads waiting for this thread to die are notified.
    ///
    /// The main thread can be detached from the VM.
    ///
    /// Trying to detach a thread that is not attached is a no-op.
    ///
    /// If an exception is pending when `detach_current_thread` is called, the VM may choose to report its existence.
    pub detach_current_thread: unsafe_extern_system_fn!((vm: *mut JavaVM) -> Jint),

    pub get_env: unsafe_extern_system_fn!((vm: *mut JavaVM, env: *mut *mut c_void, version: Jint) -> Jint),

    /// Same semantics as `attach_current_thread`, but the newly-created [java.lang.Thread] instance is a daemon.
    ///
    /// If the thread has already been attached via either `attach_current_thread` or `attach_current_thread_as_daemon`, this routine simply sets the value pointed to by `penv` to the [`JNIEnv`] of the current thread. In this case neither `attach_current_thread` nor this routine have any effect on the *daemon* status of the thread.
    ///
    /// [java.lang.Thread]: https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/lang/Thread.html
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
    option_string: *const c_char, /* the option as a string in the default platform encoding */
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

    /// Returns a default configuration for the Java VM. Before calling this function, native code must set the vm_args->version field to the JNI version it expects the VM to support. After this function returns, vm_args-
    /// >version will be set to the actual JNI version the VM supports.
    #[link_name = "JNI_GetDefaultJavaVMInitArgs"]
    pub fn jni_get_default_java_vm_init_args(vm_args: *mut c_void) -> Jint;
    /// Loads and initializes a Java VM. The current thread becomes the main thread. Sets the `p_env` argument to the JNI interface pointer of the main thread.
    ///
    /// Creation of multiple VMs in a single process is not supported.
    #[link_name = "JNI_CreateJavaVM"]
    pub fn jni_create_java_vm(
        p_vm: *mut *mut JavaVM,
        p_env: *mut *mut c_void,
        vm_args: *mut c_void,
    ) -> Jint;
    /// Returns all Java VMs that have been created. Pointers to VMs are written in the buffer `vm_buf` in the order they are created. At most `buf_len` number of entries will be written. The total number of created VMs is
    /// returned in `n_vms`.
    ///
    /// Creation of multiple VMs in a single process is not supported.
    #[link_name = "JNI_GetCreatedJavaVMs"]
    pub fn jni_get_created_java_vms(
        vm_buf: *mut *mut JavaVM,
        buf_len: Jsize,
        n_vms: *mut Jsize,
    ) -> Jint;
}

/*
 * Manifest constants
 */

pub const JNI_FALSE: Jboolean = 0;
pub const JNI_TRUE: Jboolean = 1;

/// Java SE Platform 1.1
pub const JNI_VERSION_1_1: Jint = 0x00010001;
/// Java SE Platform 1.2 & 1.3
pub const JNI_VERSION_1_2: Jint = 0x00010002;
/// Java SE Platform 1.4 & 5.0
pub const JNI_VERSION_1_4: Jint = 0x00010004;
/// Java SE Platform 6 & 7
pub const JNI_VERSION_1_6: Jint = 0x00010006;
/// Java SE Platform 8
pub const JNI_VERSION_1_8: Jint = 0x00010008;
/// Java SE Platform 9
pub const JNI_VERSION_9: Jint = 0x00090000;
/// Java SE Platform 10+
pub const JNI_VERSION_10: Jint = 0x000a0000;

/*
 * General return value constants for JNI functions.
 */

/// success
pub const JNI_OK: Jint = 0;
/// unknown error
pub const JNI_ERR: Jint = -1;
/// thread detached from the VM
pub const JNI_EDETACHED: Jint = -2;
/// JNI version error
pub const JNI_EVERSION: Jint = -3;
/// not enough memory
pub const JNI_ENOMEM: Jint = -4;
/// VM already created
pub const JNI_EEXIST: Jint = -5;
/// invalid arguments
pub const JNI_EINVAL: Jint = -6;

/*
 * Primitive Array Release Modes
 *
 * 0: copy back the content and free the elems buffer
 */

/// copy back the content but do not free the elems buffer
pub const JNI_COMMIT: Jint = 1;
/// free the buffer without copying back the possible changes
pub const JNI_ABORT: Jint = 2;
