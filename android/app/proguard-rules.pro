# Keep native methods
-keepclasseswithmembernames class * {
    native <methods>;
}

# Keep NativeActivity
-keep class android.app.NativeActivity

# Keep our package
-keep class com.waydri.compositor.** { *; }
