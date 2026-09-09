package com.waydri.compositor

object NativeBridge {
    @JvmStatic
    external fun startCompositor(): Boolean

    @JvmStatic
    external fun stopCompositor(): Boolean

    @JvmStatic
    external fun pauseCompositor(): Boolean

    @JvmStatic
    external fun resumeCompositor(): Boolean

    @JvmStatic
    external fun isCompositorRunning(): Boolean

    @JvmStatic
    external fun nativeVersion(): String

    init {
        System.loadLibrary("waydri_core")
    }
}
