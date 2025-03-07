#include <stdarg.h>
#include <stdio.h>

#include "CallJNI.h"
#include "jni.h"
#include "rdef.h"

#ifdef DEBUG
int main(int argc, char const *argv[]) {
    /* code */
    return 0;
}
#endif

JNIEXPORT jint JNICALL JNI_OnLoad(JavaVM *vm, void *reserved) {
    JNIEnv *env;
    (*vm)->GetEnv(vm, (void **)&env, JNI_VERSION_1_1);
    jclass jcls_CallJNI = (*env)->FindClass(env, "CallJNI");
    jfieldID jfid_loadStatus = (*env)->GetStaticFieldID(
        env, jcls_CallJNI, "loadStatus", "Ljava/lang/String;");
    jstring jstr_loadStatus = (*env)->NewStringUTF(env, "Loaded");
    (*env)->SetStaticObjectField(env, jcls_CallJNI, jfid_loadStatus,
                                 jstr_loadStatus);
    return JNI_VERSION_1_1;
}

JNIEXPORT void JNICALL JNI_OnUnload(JavaVM *vm, void *reserved) {
    println("%s", "JNI >>> OnUnload");
}

JNIEXPORT jint JNICALL Java_CallJNI_getVersion(JNIEnv *env, jclass _) {
    return (*env)->GetVersion(env);
}

JNIEXPORT jclass JNICALL Java_CallJNI_findClass(JNIEnv *env, jclass _,
                                                jstring name) {
    char const *c_str_name = (*env)->GetStringUTFChars(env, name, JNI_FALSE);
    return (*env)->FindClass(env, c_str_name);
}

JNIEXPORT jstring JNICALL Java_CallJNI_fromReflectedMethod(JNIEnv *env,
                                                           jclass _,
                                                           jobject method) {
    jclass jcls_String = (*env)->FindClass(env, "java/lang/String");
    jmethodID method_id = (*env)->FromReflectedMethod(env, method);
    return (jstring)(*env)->CallStaticObjectMethod(env, jcls_String, method_id,
                                                   JNI_FALSE);
}

JNIEXPORT jobject JNICALL Java_CallJNI_fromReflectedField(JNIEnv *env, jclass _,
                                                          jobject field) {
    jclass jcls_System = (*env)->FindClass(env, "java/lang/System");
    jfieldID field_id = (*env)->FromReflectedField(env, field);
    return (*env)->GetStaticObjectField(env, jcls_System, field_id);
}

JNIEXPORT jobject JNICALL Java_CallJNI_toReflectedMethod(JNIEnv *env,
                                                         jclass _) {
    jclass jcls_String = (*env)->FindClass(env, "java/lang/String");
    jmethodID jmid_valueOf = (*env)->GetStaticMethodID(
        env, jcls_String, "valueOf", "(Z)Ljava/lang/String;");
    return (*env)->ToReflectedMethod(env, jcls_String, jmid_valueOf, JNI_TRUE);
}

JNIEXPORT jobject JNICALL Java_CallJNI_getSystemOut(JNIEnv *env, jclass _) {
    jclass jcls_System = (*env)->FindClass(env, "java/lang/System");
    jfieldID jfid_out = (*env)->GetStaticFieldID(env, jcls_System, "out",
                                                 "Ljava/io/PrintStream;");
    return (*env)->GetStaticObjectField(env, jcls_System, jfid_out);
}
