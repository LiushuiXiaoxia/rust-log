#include <jni.h>
#include <string>
#include <rustlog.h>

extern "C" JNIEXPORT jstring JNICALL
Java_com_example_nlog_NativeLib_stringFromJNI(
        JNIEnv *env,
        jobject /* this */) {
    std::string hello = "Hello from C++";
    return env->NewStringUTF(hello.c_str());
}

const char *jstringToCString(JNIEnv *env, jstring jstr) {
    const char *c = env->GetStringUTFChars(jstr, nullptr);
//    return const_cast<char*>(c);
    return c;
}

extern "C" JNIEXPORT void JNICALL
Java_com_example_nlog_NativeLib_initLog(
        JNIEnv *env,
        jobject thiz,
        jstring path) {
    const char *logDir = jstringToCString(env, path);
    printf("logDir: %s\n", logDir);

    log_init(env, logDir);
}

extern "C" JNIEXPORT void JNICALL
Java_com_example_nlog_NativeLib_closeLogger(
        JNIEnv *env,
        jobject thiz) {
//    CloseLogger();
}

extern "C" JNIEXPORT void JNICALL
Java_com_example_nlog_NativeLib_logDebug(
        JNIEnv *env,
        jobject thiz,
        jstring tag,
        jstring msg
) {
    const char *level = "debug";
    const char *ctag = jstringToCString(env, tag);
    const char *cmsg = jstringToCString(env, msg);
    log_write(level, ctag, cmsg);
}

extern "C" JNIEXPORT void JNICALL
Java_com_example_nlog_NativeLib_logInfo(
        JNIEnv *env,
        jobject thiz,
        jstring tag,
        jstring msg
) {
    const char *level = "info";
    const char *ctag = jstringToCString(env, tag);
    const char *cmsg = jstringToCString(env, msg);
    log_write(level, ctag, cmsg);
}

extern "C" JNIEXPORT void JNICALL
Java_com_example_nlog_NativeLib_logWarn(
        JNIEnv *env,
        jobject thiz,
        jstring tag,
        jstring msg
) {
    const char *level = "warn";
    const char *ctag = jstringToCString(env, tag);
    const char *cmsg = jstringToCString(env, msg);
    log_write(level, ctag, cmsg);
}

extern "C" JNIEXPORT void JNICALL
Java_com_example_nlog_NativeLib_logError(
        JNIEnv *env,
        jobject thiz,
        jstring tag,
        jstring msg
) {
    const char *level = "error";
    const char *ctag = jstringToCString(env, tag);
    const char *cmsg = jstringToCString(env, msg);
    log_write(level, ctag, cmsg);
}