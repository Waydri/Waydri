cmake_minimum_required(VERSION 3.22)
project("waydri")

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(waydri_jni SHARED
    src/main/jni/waydri_native.cpp
)

target_link_libraries(waydri_jni
    log
    android
)

target_include_directories(waydri_jni PRIVATE
    src/main/jni
)
