cmake_minimum_required(VERSION 3.22)
project("waydri")

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(ANDROID_ABI STREQUAL "arm64-v8a")
    set(RUST_TARGET "aarch64-linux-android")
elseif(ANDROID_ABI STREQUAL "armeabi-v7a")
    set(RUST_TARGET "armv7-linux-androideabi")
elseif(ANDROID_ABI STREQUAL "x86_64")
    set(RUST_TARGET "x86_64-linux-android")
elseif(ANDROID_ABI STREQUAL "x86")
    set(RUST_TARGET "i686-linux-android")
else()
    message(FATAL_ERROR "Unsupported ABI: ${ANDROID_ABI}")
endif()

set(RUST_BUILD_TYPE ${CMAKE_BUILD_TYPE})
if(NOT RUST_BUILD_TYPE)
    set(RUST_BUILD_TYPE "release")
else()
    string(TOLOWER ${RUST_BUILD_TYPE} RUST_BUILD_TYPE)
endif()

set(RUST_LIB ${CMAKE_SOURCE_DIR}/../../target/${RUST_TARGET}/${RUST_BUILD_TYPE}/libwaydri_core.a)

add_library(waydri_jni SHARED
    src/main/jni/waydri_native.cpp
)

target_link_libraries(waydri_jni
    ${RUST_LIB}
    log
    android
)

target_include_directories(waydri_jni PRIVATE
    src/main/jni
)
