pub const va_list = ?*anyopaque;

// Primitive types that match up with Java equivalents

/// unsigned 8 bits
pub const jboolean = u8;
/// signed 8 bits
pub const jbyte = i8;
/// unsigned 16 bits
pub const jchar = u16;
/// signed 16 bits
pub const jshort = i16;
/// signed 32 bits
pub const jint = i32;
/// signed 64 bits
pub const jlong = i64;
/// 32 bits
pub const jfloat = f32;
/// 64 bits
pub const jdouble = f64;

/// The `jsize` integer type is used to describe cardinal indices and sizes.
pub const jsize = jint;

// Reference types that match up with Java equivalents

pub const jobject = ?*anyopaque;
pub const jclass = jobject;
pub const jthrowable = jobject;
pub const jstring = jobject;

// Array types that match up with Java equivalents

pub const jarray = jobject;
pub const jboolean_array = jarray;
pub const jbyte_array = jarray;
pub const jchar_array = jarray;
pub const jshort_array = jarray;
pub const jint_array = jarray;
pub const jlong_array = jarray;
pub const jfloat_array = jarray;
pub const jdouble_array = jarray;
pub const jobjectArray = jarray;

pub const jweak = jobject;

/// When passing arguments from ZIG to a Java method, the `Jvalue` union is used.
/// The `Jvalue` union type is used as the element type in argument arrays.
pub const Jvalue = extern union {
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

/// field IDs
pub const jfield_id = ?*anyopaque;

/// method IDs
pub const jmethod_id = ?*anyopaque;

/// Return values from JobjectRefType
pub const JobjectRefType = enum(c_uint) {
    jni_invalid_ref_type = 0,
    jni_local_ref_type = 1,
    jni_global_ref_type = 2,
    jni_weak_global_ref_type = 3,
};

pub const JNINativeMethod = extern struct {
    name: [*c]u8,
    signature: [*c]u8,
    fn_ptr: ?*anyopaque,
};

/// `JNIEnv` implements the "Java Native Inferface", and contains most of what you'll use to interact with Java from ZIG
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
    /// Returns the major version number in the higher 16 bits and the minor version number in the lower 16 bits.
    get_version: ?fn (env: [*c]JNIEnv) callconv(.C) jint,

    /// Loads a class from a buffer of raw class data. The buffer containing the raw class data is not referenced by the VM after the `define_class` call returns, and it may be discarded if desired.
    define_class: ?fn (env: [*c]JNIEnv, name: [*c]const u8, loader: jobject, buf: [*c]const jbyte, buf_len: jsize) callconv(.C) jclass,
    /// Returns a class object from a fully-qualified name, or `NULL` if the `class` cannot be found.
    find_class: ?fn (env: [*c]JNIEnv, name: [*c]const u8) callconv(.C) jclass,

    /// A JNI method ID that corresponds to the given Java reflection method, or NULL if the operation fails.
    from_reflected_method: ?fn (env: [*c]JNIEnv, method: jobject) callconv(.C) jmethod_id,
    /// A JNI field ID that corresponds to the given Java reflection field, or NULL if the operation fails.
    from_reflected_field: ?fn (env: [*c]JNIEnv, field: jobject) callconv(.C) jfield_id,

    /// Returns an instance of the `java.lang.reflect.Method` or `java.lang.reflect.Constructor` which corresponds to the given `method_id`, or `NULL` if the operation fails.
    to_reflected_method: ?fn (env: [*c]JNIEnv, cls: jclass, method_id: jmethod_id, is_static: jboolean) callconv(.C) jobject,

    /// If `clazz` represents any class other than the class `Object`, then this function returns the object that represents the superclass of the class specified by `clazz`.
    /// If `clazz` specifies the class `Object`, or `clazz` represents an interface, this function returns `NULL`.
    get_superclass: ?fn (env: [*c]JNIEnv, clazz: jclass) callconv(.C) jclass,
    /// Determines whether an object of `clazz1` can be safely cast to `clazz2`.
    is_assignable_from: ?fn (env: [*c]JNIEnv, clazz1: jclass, clazz2: jclass) callconv(.C) jboolean,

    /// Converts a field ID derived from `cls` to a `java.lang.reflect.Field` object. `is_static` must be set to `jni_true` if `field_id` refers to a static field, and [`JNI_FALSE`] otherwise.
    ///
    /// Throws `OutOfMemoryError` and returns 0 if fails.
    to_reflected_field: ?fn (env: [*c]JNIEnv, cls: jclass, field_id: jfield_id, is_static: jboolean) callconv(.C) jobject,

    throw: ?fn (env: [*c]JNIEnv, jthrowable) callconv(.C) jint,
    throw_new: ?fn (env: [*c]JNIEnv, jclass, [*c]const u8) callconv(.C) jint,
    exception_occurred: ?fn (env: [*c]JNIEnv) callconv(.C) jthrowable,
    exception_describe: ?fn (env: [*c]JNIEnv) callconv(.C) void,
    exception_clear: ?fn (env: [*c]JNIEnv) callconv(.C) void,
    fatal_error: ?fn (env: [*c]JNIEnv, [*c]const u8) callconv(.C) void,

    push_local_frame: ?fn (env: [*c]JNIEnv, jint) callconv(.C) jint,
    pop_local_frame: ?fn (env: [*c]JNIEnv, jobject) callconv(.C) jobject,

    new_global_ref: ?fn (env: [*c]JNIEnv, jobject) callconv(.C) jobject,
    delete_global_ref: ?fn (env: [*c]JNIEnv, jobject) callconv(.C) void,
    delete_local_ref: ?fn (env: [*c]JNIEnv, jobject) callconv(.C) void,
    is_same_object: ?fn (env: [*c]JNIEnv, jobject, jobject) callconv(.C) jboolean,
    new_local_ref: ?fn (env: [*c]JNIEnv, jobject) callconv(.C) jobject,
    ensure_local_capacity: ?fn (env: [*c]JNIEnv, jint) callconv(.C) jint,

    alloc_object: ?fn (env: [*c]JNIEnv, jclass) callconv(.C) jobject,
    new_object: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, ...) callconv(.C) jobject,
    new_object_v: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, va_list) callconv(.C) jobject,
    new_object_a: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, [*c]const Jvalue) callconv(.C) jobject,

    get_object_class: ?fn (env: [*c]JNIEnv, jobject) callconv(.C) jclass,
    is_instance_of: ?fn (env: [*c]JNIEnv, jobject, jclass) callconv(.C) jboolean,

    get_method_id: ?fn (env: [*c]JNIEnv, jclass, [*c]const u8, [*c]const u8) callconv(.C) jmethod_id,

    call_object_method: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, ...) callconv(.C) jobject,
    call_object_method_v: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, va_list) callconv(.C) jobject,
    call_object_method_a: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, [*c]const Jvalue) callconv(.C) jobject,

    call_boolean_method: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, ...) callconv(.C) jboolean,
    call_boolean_method_v: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, va_list) callconv(.C) jboolean,
    call_boolean_method_a: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, [*c]const Jvalue) callconv(.C) jboolean,

    call_byte_method: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, ...) callconv(.C) jbyte,
    call_byte_method_v: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, va_list) callconv(.C) jbyte,
    call_byte_method_a: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, [*c]const Jvalue) callconv(.C) jbyte,
    call_char_method: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, ...) callconv(.C) jchar,
    call_char_method_v: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, va_list) callconv(.C) jchar,
    call_char_method_a: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, [*c]const Jvalue) callconv(.C) jchar,
    call_short_method: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, ...) callconv(.C) jshort,
    call_short_method_v: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, va_list) callconv(.C) jshort,
    call_short_method_a: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, [*c]const Jvalue) callconv(.C) jshort,
    call_int_method: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, ...) callconv(.C) jint,
    call_int_method_v: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, va_list) callconv(.C) jint,
    call_int_method_a: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, [*c]const Jvalue) callconv(.C) jint,
    call_long_method: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, ...) callconv(.C) jlong,
    call_long_method_v: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, va_list) callconv(.C) jlong,
    call_long_method_a: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, [*c]const Jvalue) callconv(.C) jlong,
    call_float_method: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, ...) callconv(.C) jfloat,
    call_float_method_v: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, va_list) callconv(.C) jfloat,
    call_float_method_a: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, [*c]const Jvalue) callconv(.C) jfloat,
    call_double_method: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, ...) callconv(.C) jdouble,
    call_double_method_v: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, va_list) callconv(.C) jdouble,
    call_double_method_a: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, [*c]const Jvalue) callconv(.C) jdouble,
    call_void_method: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, ...) callconv(.C) void,
    call_void_method_v: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, va_list) callconv(.C) void,
    call_void_method_a: ?fn (env: [*c]JNIEnv, jobject, jmethod_id, [*c]const Jvalue) callconv(.C) void,
    call_nonvirtual_object_method: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, ...) callconv(.C) jobject,
    call_nonvirtual_object_method_v: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, va_list) callconv(.C) jobject,
    call_nonvirtual_object_method_a: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, [*c]const Jvalue) callconv(.C) jobject,
    call_nonvirtual_boolean_method: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, ...) callconv(.C) jboolean,
    call_nonvirtual_boolean_method_v: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, va_list) callconv(.C) jboolean,
    call_nonvirtual_boolean_method_a: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, [*c]const Jvalue) callconv(.C) jboolean,
    call_nonvirtual_byte_method: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, ...) callconv(.C) jbyte,
    call_nonvirtual_byte_method_v: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, va_list) callconv(.C) jbyte,
    call_nonvirtual_byte_method_a: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, [*c]const Jvalue) callconv(.C) jbyte,
    call_nonvirtual_char_method: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, ...) callconv(.C) jchar,
    call_nonvirtual_char_method_v: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, va_list) callconv(.C) jchar,
    call_nonvirtual_char_method_a: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, [*c]const Jvalue) callconv(.C) jchar,
    call_nonvirtual_short_method: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, ...) callconv(.C) jshort,
    call_nonvirtual_short_method_v: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, va_list) callconv(.C) jshort,
    call_nonvirtual_short_method_a: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, [*c]const Jvalue) callconv(.C) jshort,
    call_nonvirtual_int_method: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, ...) callconv(.C) jint,
    call_nonvirtual_int_method_v: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, va_list) callconv(.C) jint,
    call_nonvirtual_int_method_a: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, [*c]const Jvalue) callconv(.C) jint,
    call_nonvirtual_long_method: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, ...) callconv(.C) jlong,
    call_nonvirtual_long_method_v: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, va_list) callconv(.C) jlong,
    call_nonvirtual_long_method_a: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, [*c]const Jvalue) callconv(.C) jlong,
    call_nonvirtual_float_method: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, ...) callconv(.C) jfloat,
    call_nonvirtual_float_method_v: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, va_list) callconv(.C) jfloat,
    call_nonvirtual_float_method_a: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, [*c]const Jvalue) callconv(.C) jfloat,
    call_nonvirtual_double_method: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, ...) callconv(.C) jdouble,
    call_nonvirtual_double_method_v: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, va_list) callconv(.C) jdouble,
    call_nonvirtual_double_method_a: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, [*c]const Jvalue) callconv(.C) jdouble,
    call_nonvirtual_void_method: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, ...) callconv(.C) void,
    call_nonvirtual_void_method_v: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, va_list) callconv(.C) void,
    call_nonvirtual_void_method_a: ?fn (env: [*c]JNIEnv, jobject, jclass, jmethod_id, [*c]const Jvalue) callconv(.C) void,
    get_field_id: ?fn (env: [*c]JNIEnv, jclass, [*c]const u8, [*c]const u8) callconv(.C) jfield_id,
    get_object_field: ?fn (env: [*c]JNIEnv, jobject, jfield_id) callconv(.C) jobject,
    get_boolean_field: ?fn (env: [*c]JNIEnv, jobject, jfield_id) callconv(.C) jboolean,
    get_byte_field: ?fn (env: [*c]JNIEnv, jobject, jfield_id) callconv(.C) jbyte,
    get_char_field: ?fn (env: [*c]JNIEnv, jobject, jfield_id) callconv(.C) jchar,
    get_short_field: ?fn (env: [*c]JNIEnv, jobject, jfield_id) callconv(.C) jshort,
    get_int_field: ?fn (env: [*c]JNIEnv, jobject, jfield_id) callconv(.C) jint,
    get_long_field: ?fn (env: [*c]JNIEnv, jobject, jfield_id) callconv(.C) jlong,
    get_float_field: ?fn (env: [*c]JNIEnv, jobject, jfield_id) callconv(.C) jfloat,
    get_double_field: ?fn (env: [*c]JNIEnv, jobject, jfield_id) callconv(.C) jdouble,
    set_object_field: ?fn (env: [*c]JNIEnv, jobject, jfield_id, jobject) callconv(.C) void,
    set_boolean_field: ?fn (env: [*c]JNIEnv, jobject, jfield_id, jboolean) callconv(.C) void,
    set_byte_field: ?fn (env: [*c]JNIEnv, jobject, jfield_id, jbyte) callconv(.C) void,
    set_char_field: ?fn (env: [*c]JNIEnv, jobject, jfield_id, jchar) callconv(.C) void,
    set_short_field: ?fn (env: [*c]JNIEnv, jobject, jfield_id, jshort) callconv(.C) void,
    set_int_field: ?fn (env: [*c]JNIEnv, jobject, jfield_id, jint) callconv(.C) void,
    set_long_field: ?fn (env: [*c]JNIEnv, jobject, jfield_id, jlong) callconv(.C) void,
    set_float_field: ?fn (env: [*c]JNIEnv, jobject, jfield_id, jfloat) callconv(.C) void,
    set_double_field: ?fn (env: [*c]JNIEnv, jobject, jfield_id, jdouble) callconv(.C) void,
    get_static_method_id: ?fn (env: [*c]JNIEnv, jclass, [*c]const u8, [*c]const u8) callconv(.C) jmethod_id,
    call_static_object_method: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, ...) callconv(.C) jobject,
    call_static_object_method_v: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, va_list) callconv(.C) jobject,
    call_static_object_method_a: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, [*c]const Jvalue) callconv(.C) jobject,
    call_static_boolean_method: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, ...) callconv(.C) jboolean,
    call_static_boolean_method_v: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, va_list) callconv(.C) jboolean,
    call_static_boolean_method_a: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, [*c]const Jvalue) callconv(.C) jboolean,
    call_static_byte_method: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, ...) callconv(.C) jbyte,
    call_static_byte_method_v: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, va_list) callconv(.C) jbyte,
    call_static_byte_method_a: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, [*c]const Jvalue) callconv(.C) jbyte,
    call_static_char_method: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, ...) callconv(.C) jchar,
    call_static_char_method_v: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, va_list) callconv(.C) jchar,
    call_static_char_method_a: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, [*c]const Jvalue) callconv(.C) jchar,
    call_static_short_method: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, ...) callconv(.C) jshort,
    call_static_short_method_v: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, va_list) callconv(.C) jshort,
    call_static_short_method_a: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, [*c]const Jvalue) callconv(.C) jshort,
    call_static_int_method: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, ...) callconv(.C) jint,
    call_static_int_method_v: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, va_list) callconv(.C) jint,
    call_static_int_method_a: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, [*c]const Jvalue) callconv(.C) jint,
    call_static_long_method: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, ...) callconv(.C) jlong,
    call_static_long_method_v: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, va_list) callconv(.C) jlong,
    call_static_long_method_a: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, [*c]const Jvalue) callconv(.C) jlong,
    call_static_float_method: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, ...) callconv(.C) jfloat,
    call_static_float_method_v: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, va_list) callconv(.C) jfloat,
    call_static_float_method_a: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, [*c]const Jvalue) callconv(.C) jfloat,
    call_static_double_method: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, ...) callconv(.C) jdouble,
    call_static_double_method_v: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, va_list) callconv(.C) jdouble,
    call_static_double_method_a: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, [*c]const Jvalue) callconv(.C) jdouble,
    call_static_void_method: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, ...) callconv(.C) void,
    call_static_void_method_v: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, va_list) callconv(.C) void,
    call_static_void_method_a: ?fn (env: [*c]JNIEnv, jclass, jmethod_id, [*c]const Jvalue) callconv(.C) void,
    get_static_field_id: ?fn (env: [*c]JNIEnv, jclass, [*c]const u8, [*c]const u8) callconv(.C) jfield_id,
    get_static_object_field: ?fn (env: [*c]JNIEnv, jclass, jfield_id) callconv(.C) jobject,
    get_static_boolean_field: ?fn (env: [*c]JNIEnv, jclass, jfield_id) callconv(.C) jboolean,
    get_static_byte_field: ?fn (env: [*c]JNIEnv, jclass, jfield_id) callconv(.C) jbyte,
    get_static_char_field: ?fn (env: [*c]JNIEnv, jclass, jfield_id) callconv(.C) jchar,
    get_static_short_field: ?fn (env: [*c]JNIEnv, jclass, jfield_id) callconv(.C) jshort,
    get_static_int_field: ?fn (env: [*c]JNIEnv, jclass, jfield_id) callconv(.C) jint,
    get_static_long_field: ?fn (env: [*c]JNIEnv, jclass, jfield_id) callconv(.C) jlong,
    get_static_float_field: ?fn (env: [*c]JNIEnv, jclass, jfield_id) callconv(.C) jfloat,
    get_static_double_field: ?fn (env: [*c]JNIEnv, jclass, jfield_id) callconv(.C) jdouble,
    set_static_object_field: ?fn (env: [*c]JNIEnv, jclass, jfield_id, jobject) callconv(.C) void,
    set_static_boolean_field: ?fn (env: [*c]JNIEnv, jclass, jfield_id, jboolean) callconv(.C) void,
    set_static_byte_field: ?fn (env: [*c]JNIEnv, jclass, jfield_id, jbyte) callconv(.C) void,
    set_static_char_field: ?fn (env: [*c]JNIEnv, jclass, jfield_id, jchar) callconv(.C) void,
    set_static_short_field: ?fn (env: [*c]JNIEnv, jclass, jfield_id, jshort) callconv(.C) void,
    set_static_int_field: ?fn (env: [*c]JNIEnv, jclass, jfield_id, jint) callconv(.C) void,
    set_static_long_field: ?fn (env: [*c]JNIEnv, jclass, jfield_id, jlong) callconv(.C) void,
    set_static_float_field: ?fn (env: [*c]JNIEnv, jclass, jfield_id, jfloat) callconv(.C) void,
    set_static_double_field: ?fn (env: [*c]JNIEnv, jclass, jfield_id, jdouble) callconv(.C) void,
    new_string: ?fn (env: [*c]JNIEnv, [*c]const jchar, jsize) callconv(.C) jstring,
    get_string_length: ?fn (env: [*c]JNIEnv, jstring) callconv(.C) jsize,
    get_string_chars: ?fn (env: [*c]JNIEnv, jstring, [*c]jboolean) callconv(.C) [*c]const jchar,
    release_string_chars: ?fn (env: [*c]JNIEnv, jstring, [*c]const jchar) callconv(.C) void,
    new_string_utf: ?fn (env: [*c]JNIEnv, [*c]const u8) callconv(.C) jstring,
    get_string_utf_length: ?fn (env: [*c]JNIEnv, jstring) callconv(.C) jsize,
    get_string_utf_chars: ?fn (env: [*c]JNIEnv, jstring, [*c]jboolean) callconv(.C) [*c]const u8,
    release_string_utf_chars: ?fn (env: [*c]JNIEnv, jstring, [*c]const u8) callconv(.C) void,
    get_array_length: ?fn (env: [*c]JNIEnv, jarray) callconv(.C) jsize,
    new_object_array: ?fn (env: [*c]JNIEnv, jsize, jclass, jobject) callconv(.C) jobjectArray,
    get_object_array_element: ?fn (env: [*c]JNIEnv, jobjectArray, jsize) callconv(.C) jobject,
    set_object_array_element: ?fn (env: [*c]JNIEnv, jobjectArray, jsize, jobject) callconv(.C) void,
    new_boolean_array: ?fn (env: [*c]JNIEnv, jsize) callconv(.C) jboolean_array,
    new_byte_array: ?fn (env: [*c]JNIEnv, jsize) callconv(.C) jbyte_array,
    new_char_array: ?fn (env: [*c]JNIEnv, jsize) callconv(.C) jchar_array,
    new_short_array: ?fn (env: [*c]JNIEnv, jsize) callconv(.C) jshort_array,
    new_int_array: ?fn (env: [*c]JNIEnv, jsize) callconv(.C) jint_array,
    new_long_array: ?fn (env: [*c]JNIEnv, jsize) callconv(.C) jlong_array,
    new_float_array: ?fn (env: [*c]JNIEnv, jsize) callconv(.C) jfloat_array,
    new_double_array: ?fn (env: [*c]JNIEnv, jsize) callconv(.C) jdouble_array,
    get_boolean_array_elements: ?fn (env: [*c]JNIEnv, jboolean_array, [*c]jboolean) callconv(.C) [*c]jboolean,
    get_byte_array_elements: ?fn (env: [*c]JNIEnv, jbyte_array, [*c]jboolean) callconv(.C) [*c]jbyte,
    get_char_array_elements: ?fn (env: [*c]JNIEnv, jchar_array, [*c]jboolean) callconv(.C) [*c]jchar,
    get_short_array_elements: ?fn (env: [*c]JNIEnv, jshort_array, [*c]jboolean) callconv(.C) [*c]jshort,
    get_int_array_elements: ?fn (env: [*c]JNIEnv, jint_array, [*c]jboolean) callconv(.C) [*c]jint,
    get_long_array_elements: ?fn (env: [*c]JNIEnv, jlong_array, [*c]jboolean) callconv(.C) [*c]jlong,
    get_float_array_elements: ?fn (env: [*c]JNIEnv, jfloat_array, [*c]jboolean) callconv(.C) [*c]jfloat,
    get_double_array_elements: ?fn (env: [*c]JNIEnv, jdouble_array, [*c]jboolean) callconv(.C) [*c]jdouble,
    release_boolean_array_elements: ?fn (env: [*c]JNIEnv, jboolean_array, [*c]jboolean, jint) callconv(.C) void,
    release_byte_array_elements: ?fn (env: [*c]JNIEnv, jbyte_array, [*c]jbyte, jint) callconv(.C) void,
    release_char_array_elements: ?fn (env: [*c]JNIEnv, jchar_array, [*c]jchar, jint) callconv(.C) void,
    release_short_array_elements: ?fn (env: [*c]JNIEnv, jshort_array, [*c]jshort, jint) callconv(.C) void,
    release_int_array_elements: ?fn (env: [*c]JNIEnv, jint_array, [*c]jint, jint) callconv(.C) void,
    release_long_array_elements: ?fn (env: [*c]JNIEnv, jlong_array, [*c]jlong, jint) callconv(.C) void,
    release_float_array_elements: ?fn (env: [*c]JNIEnv, jfloat_array, [*c]jfloat, jint) callconv(.C) void,
    release_double_array_elements: ?fn (env: [*c]JNIEnv, jdouble_array, [*c]jdouble, jint) callconv(.C) void,
    get_boolean_array_region: ?fn (env: [*c]JNIEnv, jboolean_array, jsize, jsize, [*c]jboolean) callconv(.C) void,
    get_byte_array_region: ?fn (env: [*c]JNIEnv, jbyte_array, jsize, jsize, [*c]jbyte) callconv(.C) void,
    get_char_array_region: ?fn (env: [*c]JNIEnv, jchar_array, jsize, jsize, [*c]jchar) callconv(.C) void,
    get_short_array_region: ?fn (env: [*c]JNIEnv, jshort_array, jsize, jsize, [*c]jshort) callconv(.C) void,
    get_int_array_region: ?fn (env: [*c]JNIEnv, jint_array, jsize, jsize, [*c]jint) callconv(.C) void,
    get_long_array_region: ?fn (env: [*c]JNIEnv, jlong_array, jsize, jsize, [*c]jlong) callconv(.C) void,
    get_float_array_region: ?fn (env: [*c]JNIEnv, jfloat_array, jsize, jsize, [*c]jfloat) callconv(.C) void,
    get_double_array_region: ?fn (env: [*c]JNIEnv, jdouble_array, jsize, jsize, [*c]jdouble) callconv(.C) void,
    set_boolean_array_region: ?fn (env: [*c]JNIEnv, jboolean_array, jsize, jsize, [*c]const jboolean) callconv(.C) void,
    set_byte_array_region: ?fn (env: [*c]JNIEnv, jbyte_array, jsize, jsize, [*c]const jbyte) callconv(.C) void,
    set_char_array_region: ?fn (env: [*c]JNIEnv, jchar_array, jsize, jsize, [*c]const jchar) callconv(.C) void,
    set_short_array_region: ?fn (env: [*c]JNIEnv, jshort_array, jsize, jsize, [*c]const jshort) callconv(.C) void,
    set_int_array_region: ?fn (env: [*c]JNIEnv, jint_array, jsize, jsize, [*c]const jint) callconv(.C) void,
    set_long_array_region: ?fn (env: [*c]JNIEnv, jlong_array, jsize, jsize, [*c]const jlong) callconv(.C) void,
    set_float_array_region: ?fn (env: [*c]JNIEnv, jfloat_array, jsize, jsize, [*c]const jfloat) callconv(.C) void,
    set_double_array_region: ?fn (env: [*c]JNIEnv, jdouble_array, jsize, jsize, [*c]const jdouble) callconv(.C) void,
    register_natives: ?fn (env: [*c]JNIEnv, jclass, [*c]const JNINativeMethod, jint) callconv(.C) jint,
    unregister_natives: ?fn (env: [*c]JNIEnv, jclass) callconv(.C) jint,
    monitor_enter: ?fn (env: [*c]JNIEnv, jobject) callconv(.C) jint,
    monitor_exit: ?fn (env: [*c]JNIEnv, jobject) callconv(.C) jint,
    get_java_vm: ?fn (env: [*c]JNIEnv, [*c][*c]JavaVM) callconv(.C) jint,
    get_string_region: ?fn (env: [*c]JNIEnv, jstring, jsize, jsize, [*c]jchar) callconv(.C) void,
    get_string_utf_region: ?fn (env: [*c]JNIEnv, jstring, jsize, jsize, [*c]u8) callconv(.C) void,
    get_primitive_array_critical: ?fn (env: [*c]JNIEnv, jarray, [*c]jboolean) callconv(.C) ?*anyopaque,
    release_primitive_array_critical: ?fn (env: [*c]JNIEnv, jarray, ?*anyopaque, jint) callconv(.C) void,
    get_string_critical: ?fn (env: [*c]JNIEnv, jstring, [*c]jboolean) callconv(.C) [*c]const jchar,
    release_string_critical: ?fn (env: [*c]JNIEnv, jstring, [*c]const jchar) callconv(.C) void,
    new_weak_global_ref: ?fn (env: [*c]JNIEnv, jobject) callconv(.C) jweak,
    delete_weak_global_ref: ?fn (env: [*c]JNIEnv, jweak) callconv(.C) void,
    exception_check: ?fn (env: [*c]JNIEnv) callconv(.C) jboolean,
    new_direct_byte_buffer: ?fn (env: [*c]JNIEnv, ?*anyopaque, jlong) callconv(.C) jobject,
    get_direct_buffer_address: ?fn (env: [*c]JNIEnv, jobject) callconv(.C) ?*anyopaque,
    /// Fetches and returns the capacity of the memory region referenced by the given direct `java.nio.Buffer`. The capacity is the number of *elements* that the memory region contains.
    get_direct_buffer_capacity: ?fn (env: [*c]JNIEnv, buf: jobject) callconv(.C) jlong,

    /// Returns the type of the object referred to by the `obj` argument. The argument `obj` can either be a local, global or weak global reference, or `NULL`.
    ///
    /// `JNI_VERSION` >= `JNI_VERSION_1_6` can be used normally
    get_object_ref_type: ?fn (env: [*c]JNIEnv, obj: jobject) callconv(.C) JobjectRefType,
    /// Returns the `java.lang.Module` object for the module that the class is a member of. If the class is not in a named module then the unnamed module of the class loader for the class is returned. If the class represents
    /// an array type then this function returns the `Module` object for the element type. If the class represents a primitive type or `void`, then the `Module` object for the `java.base` module is returned.
    ///
    /// `JNI_VERSION` >= `JNI_VERSION_9` can be used normally
    get_module: ?fn (env: [*c]JNIEnv, clazz: jclass) callconv(.C) jobject,
};

/// JNI invocation interface
pub const JNIInvokeInterface = extern struct {
    reserved0: ?*anyopaque,
    reserved1: ?*anyopaque,
    reserved2: ?*anyopaque,

    /// Unloads a Java VM and reclaims its resources.
    /// 
    /// Any thread, whether attached or not, can invoke this function. If the current thread is attached, the VM waits until the current thread is the only non-daemon user-level Java thread. If the current thread is not
    /// attached, the VM attaches the current thread and then waits until the current thread is the only non-daemon user-level thread.
    destroy_java_vm: ?fn (vm: [*c]JavaVM) callconv(.C) jint,

    /// Attaches the current thread to a Java VM. Returns a JNI interface pointer in the `JNIEnv` argument.
    /// 
    /// Trying to attach a thread that is already attached is a no-op.
    /// 
    /// A native thread cannot be attached simultaneously to two Java VMs.
    /// 
    /// When a thread is attached to the VM, the context class loader is the bootstrap loader.
    attach_current_thread: ?fn (vm: [*c]JavaVM, p_env: [*c]?*anyopaque, thr_args: ?*anyopaque) callconv(.C) jint,

    /// Detaches the current thread from a Java VM. All Java monitors held by this thread are released. All Java threads waiting for this thread to die are notified.
    ///
    /// The main thread can be detached from the VM.
    ///
    /// Trying to detach a thread that is not attached is a no-op.
    ///
    /// If an exception is pending when `detach_current_thread` is called, the VM may choose to report its existence.
    detach_current_thread: ?fn (vm: [*c]JavaVM) callconv(.C) jint,

    get_env: ?fn (vm: [*c]JavaVM, env: [*c]?*anyopaque, version: jint) callconv(.C) jint,

    /// Same semantics as `attach_current_thread`, but the newly-created `java.lang.Thread` instance is a daemon.
    ///
    /// If the thread has already been attached via either `attach_current_thread` or `attach_current_thread_as_daemon`, this routine simply sets the value pointed to by `penv` to the `JNIEnv` of the current thread. In this case neither `attach_current_thread` nor this routine have any effect on the *daemon* status of the thread.
    attach_current_thread_as_daemon: ?fn (vm: [*c]JavaVM, penv: [*c]?*anyopaque, args: ?*anyopaque) callconv(.C) jint,
};

pub const JavaVMAttachArgs = extern struct {
    version: jint, // must be >= JNI_VERSION_1_1
    name: [*c]u8, // NULL or name of thread as modified UTF-8 str
    group: jobject, // global ref of a ThreadGroup object, or NULL
};

/// JNI 1.2+ initialization (As of 1.6, the pre-1.2 structures are no longer supported)
pub const JavaVMOption = extern struct {
    option_string: [*c]u8, // the option as a string in the default platform encoding
    extraInfo: ?*anyopaque,
};

pub const JavaVMInitArgs = extern struct {
    version: jint, // use JNI_VERSION_1_1 or later

    n_options: jint,
    options: [*c]JavaVMOption,
    ignore_unrecognized: jboolean,
};

//
// VM initialization functions
//
// Note these are the only symbols exported for JNI by the VM
//

/// Returns a default configuration for the Java VM. Before calling this function, native code must set the vm_args->version field to the JNI version it expects the VM to support. After this function returns, vm_args->version will be set to the actual JNI version the VM supports.
///
/// Returns JNI_OK if the requested version is supported; returns a JNI error code (a negative number) if the requested version is not supported.
pub extern fn @"JNI_GetDefaultJavaVMInitArgs"(vm_args: ?*anyopaque) jint;
/// Loads and initializes a Java VM. The current thread becomes the main thread. Sets the env argument to the JNI interface pointer of the main thread.
/// Creation of multiple VMs in a single process is not supported.
pub extern fn @"JNI_CreateJavaVM"(p_vm: [*c][*c]JavaVM, p_env: [*c]?*anyopaque, vm_args: ?*anyopaque) jint;
/// Returns all Java VMs that have been created. Pointers to VMs are written in the buffer `vm_buf` in the order they are created. At most `buf_len` number of entries will be written. The total number of created VMs is returned in `n_vms`.
/// Creation of multiple VMs in a single process is not supported.
pub extern fn @"JNI_GetCreatedJavaVMs"(vm_buf: [*c][*c]JavaVM, buf_len: jsize, n_vms: [*c]jsize) jint;

//
// Manifest constants
//

pub const jni_false = @as(jboolean, 0);
pub const jni_true = @as(jboolean, 1);

/// Java SE Platform 1.1
pub const jni_version_1_1 = @as(jint, 0x00010001);
/// Java SE Platform 1.2 & 1.3
pub const jni_version_1_2 = @as(jint, 0x00010002);
/// Java SE Platform 1.4 & 5.0
pub const jni_version_1_4 = @as(jint, 0x00010004);
/// Java SE Platform 6 & 7
pub const jni_version_1_6 = @as(jint, 0x00010006);
/// Java SE Platform 8
pub const jni_version_1_8 = @as(jint, 0x00010008);
/// Java SE Platform 9
pub const jni_version_9 = @as(jint, 0x00090000);
/// Java SE Platform 10+
pub const jni_version_10 = @as(jint, 0x000a0000);

//
// General return value constants for JNI functions.
//

/// success
pub const jni_ok = @as(jint, 0);
/// unknown error
pub const jni_err = @as(jint, -1);
/// thread detached from the VM
pub const jni_edetached = @as(jint, -2);
/// JNI version error
pub const jni_eversion = @as(jint, -3);
/// not enough memory
pub const jni_enomem = @as(jint, -4);
/// VM already created
pub const jni_eexist = @as(jint, -5);
/// invalid arguments
pub const jni_einval = @as(jint, -6);

//
// Primitive Array Release Modes
//
// 0: copy back the content and free the elems buffer
//

/// copy back the content but do not free the elems buffer
pub const jni_commit = @as(jint, 1);
/// free the buffer without copying back the possible changes
pub const jni_abort = @as(jint, 2);
