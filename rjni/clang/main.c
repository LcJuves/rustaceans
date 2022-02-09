#include <stdarg.h>
#include <stdio.h>

#include "CallJNI.h"
#include "jni.h"
#include "rustdef.h"

#ifdef DEBUG
int main(int argc, char const *argv[]) {
    /* code */
    return 0;
}
#endif

JNIEXPORT jint JNICALL JNI_OnLoad(JavaVM *vm, void *reserved) {
    println("%s", "JNI >>> OnLoad");
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

JNIEXPORT jobject JNICALL Java_CallJNI_getSystemOut(JNIEnv *env, jclass _) {
    jclass jcls_System = (*env)->FindClass(env, "java/lang/System");
    jfieldID jfid_out = (*env)->GetStaticFieldID(env, jcls_System, "out",
                                                 "Ljava/io/PrintStream;");
    return (*env)->GetStaticObjectField(env, jcls_System, jfid_out);
}
