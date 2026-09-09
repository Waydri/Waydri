package com.waydri.compositor

import android.util.Log

class CompositorManager {
    private val bridge = NativeBridge
    private var running = false
    private val config = mutableMapOf<String, String>()

    private external fun nativeSetConfig(key: String, value: String)

    fun start() {
        if (running) return
        bridge.startCompositor()
        running = true
        Log.i("Waydri", "compositor started v${bridge.nativeVersion()}")
    }

    fun stop() {
        if (!running) return
        bridge.stopCompositor()
        running = false
        Log.i("Waydri", "compositor stopped")
    }

    fun pause() {
        if (!running) return
        bridge.pauseCompositor()
        Log.i("Waydri", "compositor paused")
    }

    fun resume() {
        if (!running) return
        bridge.resumeCompositor()
        Log.i("Waydri", "compositor resumed")
    }

    fun isRunning(): Boolean = running && bridge.isCompositorRunning()

    fun setConfig(key: String, value: String) {
        config[key] = value
        nativeSetConfig(key, value)
    }

    fun getConfig(key: String): String? = config[key]

    fun version(): String = bridge.nativeVersion()
}
