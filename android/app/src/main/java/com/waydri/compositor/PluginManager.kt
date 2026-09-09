package com.waydri.compositor

import android.util.Log
import java.io.File

class PluginManager {
    private external fun nativeLoadPlugin(path: String): Boolean
    private external fun nativeUnloadPlugin(handle: Long)

    private val plugins = mutableMapOf<String, Long>()

    fun loadAll(dir: File): Int {
        if (!dir.exists()) return 0
        val soFiles = dir.listFiles { f -> f.extension == "so" } ?: return 0
        var count = 0
        for (file in soFiles) {
            if (load(file.absolutePath)) count++
        }
        Log.i("Waydri", "loaded $count plugin(s)")
        return count
    }

    fun load(path: String): Boolean {
        if (plugins.containsKey(path)) return false
        val handle = nativeLoadPlugin(path)
        if (handle) {
            Log.i("Waydri", "plugin loaded $path")
            return true
        }
        return false
    }

    fun unload(path: String) {
        val handle = plugins.remove(path) ?: return
        nativeUnloadPlugin(handle)
        Log.i("Waydri", "plugin unloaded $path")
    }

    fun loadedCount(): Int = plugins.size

    fun loadedPaths(): List<String> = plugins.keys.toList()
}
