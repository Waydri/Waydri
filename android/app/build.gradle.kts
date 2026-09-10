plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

tasks.register<Exec>("setupRust") {
    workingDir = rootProject.projectDir
    commandLine(
        "rustup", "target", "add",
        "aarch64-linux-android",
        "armv7-linux-androideabi",
        "x86_64-linux-android",
        "i686-linux-android"
    )
    isIgnoreExitValue = true
}

tasks.register<Exec>("buildRust") {
    workingDir = rootProject.projectDir
    dependsOn(tasks.named("setupRust"))
    environment("LIBDRM_INCLUDE_PATH", "/usr/include/libdrm")
    val buildType = if (project.gradle.startParameter.taskRequests.toString().contains("Release")) "release" else "debug"
    val command = mutableListOf("cargo", "build")
    command.addAll(listOf("--target", "aarch64-linux-android"))
    command.addAll(listOf("--target", "armv7-linux-androideabi"))
    command.addAll(listOf("--target", "x86_64-linux-android"))
    command.addAll(listOf("--target", "i686-linux-android"))
    if (buildType == "release") {
        command.add("--release")
    }
    commandLine(command)
}
tasks.preBuild {
    dependsOn(tasks.named("buildRust"))
}

android {
    ndkVersion = "26.3.11579264"
    namespace = "com.waydri.compositor"
    compileSdk = 36

    defaultConfig {
        applicationId = "com.waydri.compositor"
        minSdk = 24
        targetSdk = 36
        versionCode = 11
        versionName = "0.11"

        ndk {
            abiFilters.addAll(listOf("armeabi-v7a", "arm64-v8a", "x86", "x86_64"))
        }

        externalNativeBuild {
            cmake {
                cppFlags("-std=c++17")
                arguments(
                    "-DANDROID_STL=c++_shared",
                    "-DRUST_LIB_DIR=${rootProject.projectDir}/target"
                )
            }
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
            signingConfig = signingConfigs.getByName("debug")
        }
        debug {
            isDebuggable = true
            isJniDebuggable = true
        }
    }

    externalNativeBuild {
        cmake {
            path = file("CMakeLists.txt")
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }

    buildFeatures {
        viewBinding = true
        aidl = true
    }

    packaging {
        resources {
            excludes += "/META-INF/{AL2.0,LGPL2.1}"
        }
        jniLibs {
            useLegacyPackaging = true
        }
    }

    splits {
        abi {
            isEnable = true
            reset()
            include("armeabi-v7a", "arm64-v8a", "x86", "x86_64")
            isUniversalApk = true
        }
    }
}

dependencies {
    implementation("androidx.core:core-ktx:1.13.1")
    implementation("androidx.appcompat:appcompat:1.7.0")
    implementation("com.google.android.material:material:1.12.0")
    implementation("androidx.constraintlayout:constraintlayout:2.1.4")
    implementation("androidx.lifecycle:lifecycle-runtime-ktx:2.8.6")
    implementation("androidx.activity:activity-ktx:1.9.2")
}
