#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <sys/time.h>

typedef enum {
    ANDROID_LOG_UNKNOWN = 0,
    ANDROID_LOG_DEFAULT,
    ANDROID_LOG_VERBOSE,
    ANDROID_LOG_DEBUG,
    ANDROID_LOG_INFO,
    ANDROID_LOG_WARN,
    ANDROID_LOG_ERROR,
    ANDROID_LOG_FATAL,
    ANDROID_LOG_SILENT
} android_LogPriority;

static pthread_mutex_t log_mutex = PTHREAD_MUTEX_INITIALIZER;
static int log_filter = ANDROID_LOG_VERBOSE;

__attribute__((visibility("default")))
int __android_log_write(int prio, const char* tag, const char* msg) {
    if (prio < log_filter) return 0;
    const char* level;
    switch (prio) {
        case ANDROID_LOG_VERBOSE: level = "V"; break;
        case ANDROID_LOG_DEBUG:   level = "D"; break;
        case ANDROID_LOG_INFO:    level = "I"; break;
        case ANDROID_LOG_WARN:    level = "W"; break;
        case ANDROID_LOG_ERROR:   level = "E"; break;
        case ANDROID_LOG_FATAL:   level = "F"; break;
        default:                  level = "?";
    }
    pthread_mutex_lock(&log_mutex);
    struct timeval tv;
    gettimeofday(&tv, 0);
    fprintf(stderr, "%02d/%02d %02d:%02d:%02d.%03ld %ld %-6s %s: %s\n",
            (int)(tv.tv_sec / 86400 % 31 + 1),
            1,
            (int)(tv.tv_sec % 86400 / 3600),
            (int)(tv.tv_sec % 3600 / 60),
            (int)(tv.tv_sec % 60),
            tv.tv_usec / 1000,
            (long)pthread_self(),
            tag ? level : "?",
            tag ? tag : "",
            msg ? msg : "");
    pthread_mutex_unlock(&log_mutex);
    return msg ? (int)strlen(msg) : 0;
}

__attribute__((visibility("default")))
int __android_log_print(int prio, const char* tag, const char* fmt, ...) {
    va_list ap;
    va_start(ap, fmt);
    char buf[2048];
    vsnprintf(buf, sizeof(buf), fmt, ap);
    va_end(ap);
    return __android_log_write(prio, tag, buf);
}

__attribute__((visibility("default")))
int __android_log_vprint(int prio, const char* tag, const char* fmt, va_list ap) {
    char buf[2048];
    vsnprintf(buf, sizeof(buf), fmt, ap);
    return __android_log_write(prio, tag, buf);
}

__attribute__((visibility("default")))
int __android_log_buf_write(int _buf_id, int prio, const char* tag, const char* msg) {
    (void)_buf_id;
    return __android_log_write(prio, tag, msg);
}

__attribute__((visibility("default")))
int __android_log_is_loggable(int prio, const char* _tag, int default_prio) {
    (void)_tag;
    if (prio >= log_filter) return 1;
    return prio >= default_prio ? 1 : 0;
}

__attribute__((visibility("default")))
void __android_log_assert(const char* cond, const char* tag, const char* fmt, ...) {
    char buf[2048];
    va_list ap;
    va_start(ap, fmt);
    vsnprintf(buf, sizeof(buf), fmt, ap);
    va_end(ap);
    __android_log_write(ANDROID_LOG_FATAL, tag, buf);
    if (cond) __android_log_write(ANDROID_LOG_FATAL, tag, cond);
    abort();
}

__attribute__((visibility("default")))
void __android_log_set_minimum_priority(int prio) {
    log_filter = prio;
}

__attribute__((visibility("default")))
int __android_log_get_minimum_priority(void) {
    return log_filter;
}