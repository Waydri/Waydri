#include <jni.h>
#include <android/log.h>
#include <string>
#include <atomic>

#define LOG_TAG "WaydriNative"
#define LOGI(...) __android_log_print(ANDROID_LOG_INFO, LOG_TAG, __VA_ARGS__)
#define LOGE(...) __android_log_print(ANDROID_LOG_ERROR, LOG_TAG, __VA_ARGS__)

extern "C" {
extern void rust_compositor_start();
extern void rust_compositor_stop();
extern void rust_compositor_pause();
extern void rust_compositor_resume();
extern bool rust_compositor_is_running();
}

static std::atomic<bool> g_initialized{false};

static void ensure_log() {
    if (!g_initialized.exchange(true)) {
        LOGI("waydri native layer initialized");
    }
}

extern "C" JNIEXPORT jboolean JNICALL
Java_com_waydri_compositor_NativeBridge_startCompositor(JNIEnv*, jobject) {
    ensure_log();
    LOGI("startCompositor");
    rust_compositor_start();
    return JNI_TRUE;
}

extern "C" JNIEXPORT jboolean JNICALL
Java_com_waydri_compositor_NativeBridge_stopCompositor(JNIEnv*, jobject) {
    ensure_log();
    LOGI("stopCompositor");
    rust_compositor_stop();
    return JNI_TRUE;
}

extern "C" JNIEXPORT jboolean JNICALL
Java_com_waydri_compositor_NativeBridge_pauseCompositor(JNIEnv*, jobject) {
    ensure_log();
    LOGI("pauseCompositor");
    rust_compositor_pause();
    return JNI_TRUE;
}

extern "C" JNIEXPORT jboolean JNICALL
Java_com_waydri_compositor_NativeBridge_resumeCompositor(JNIEnv*, jobject) {
    ensure_log();
    LOGI("resumeCompositor");
    rust_compositor_resume();
    return JNI_TRUE;
}

extern "C" JNIEXPORT jboolean JNICALL
Java_com_waydri_compositor_NativeBridge_isCompositorRunning(JNIEnv*, jobject) {
    return rust_compositor_is_running() ? JNI_TRUE : JNI_FALSE;
}

extern "C" JNIEXPORT jstring JNICALL
Java_com_waydri_compositor_NativeBridge_nativeVersion(JNIEnv* env, jobject) {
    return env->NewStringUTF("0.11.0");
}

extern "C" JNIEXPORT void JNICALL
Java_com_waydri_compositor_MainActivity_startCompositor(JNIEnv*, jobject) {
    ensure_log();
    rust_compositor_start();
}

extern "C" JNIEXPORT void JNICALL
Java_com_waydri_compositor_MainActivity_stopCompositor(JNIEnv*, jobject) {
    ensure_log();
    rust_compositor_stop();
}

extern "C" JNIEXPORT void JNICALL
Java_com_waydri_compositor_MainActivity_pauseCompositor(JNIEnv*, jobject) {
    ensure_log();
    rust_compositor_pause();
}

extern "C" JNIEXPORT void JNICALL
Java_com_waydri_compositor_MainActivity_resumeCompositor(JNIEnv*, jobject) {
    ensure_log();
    rust_compositor_resume();
}

extern "C" JNIEXPORT void JNICALL
Java_com_waydri_compositor_InputHandler_nativePointer(JNIEnv* env, jobject,
                                                       jint action, jfloat x, jfloat y,
                                                       jint button, jlong time_ms) {
    (void)env; (void)action; (void)x; (void)y; (void)button; (void)time_ms;
    ensure_log();
}

extern "C" JNIEXPORT void JNICALL
Java_com_waydri_compositor_WaydriSurfaceView_nativeSurfaceCreated(JNIEnv*, jobject,
                                                                   jobject surface, jint w, jint h) {
    (void)surface; (void)w; (void)h;
    ensure_log();
    LOGI("surfaceCreated %dx%d", w, h);
}

extern "C" JNIEXPORT void JNICALL
Java_com_waydri_compositor_WaydriSurfaceView_nativeSurfaceChanged(JNIEnv*, jobject,
                                                                   jint w, jint h) {
    (void)w; (void)h;
}

extern "C" JNIEXPORT void JNICALL
Java_com_waydri_compositor_WaydriSurfaceView_nativeSurfaceDestroyed(JNIEnv*, jobject) {
    ensure_log();
    LOGI("surfaceDestroyed");
}

extern "C" JNIEXPORT jboolean JNICALL
Java_com_waydri_compositor_PluginManager_nativeLoadPlugin(JNIEnv* env, jobject,
                                                           jstring path) {
    const char* cpath = env->GetStringUTFChars(path, nullptr);
    if (!cpath) return JNI_FALSE;
    LOGI("loadPlugin %s", cpath);
    env->ReleaseStringUTFChars(path, cpath);
    return JNI_FALSE;
}

extern "C" JNIEXPORT void JNICALL
Java_com_waydri_compositor_PluginManager_nativeUnloadPlugin(JNIEnv*, jobject, jlong handle) {
    (void)handle;
}

extern "C" JNIEXPORT void JNICALL
Java_com_waydri_compositor_CompositorManager_nativeSetConfig(JNIEnv* env, jobject,
                                                              jstring key, jstring value) {
    const char* ckey = env->GetStringUTFChars(key, nullptr);
    const char* cval = env->GetStringUTFChars(value, nullptr);
    if (ckey && cval) {
        LOGI("setConfig %s=%s", ckey, cval);
    }
    if (ckey) env->ReleaseStringUTFChars(key, ckey);
    if (cval) env->ReleaseStringUTFChars(value, cval);
}

extern "C" JNIEXPORT void JNICALL
Java_com_waydri_compositor_ThemeManager_nativeApplyTheme(JNIEnv* env, jobject,
                                                          jstring name) {
    const char* cname = env->GetStringUTFChars(name, nullptr);
    if (cname) {
        LOGI("applyTheme %s", cname);
        env->ReleaseStringUTFChars(name, cname);
    }
}
