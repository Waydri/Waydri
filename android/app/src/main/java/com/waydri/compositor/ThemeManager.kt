package com.waydri.compositor

import android.util.Log

class ThemeManager {
    private external fun nativeApplyTheme(name: String)

    private var currentTheme: String = "default"

    fun apply(name: String) {
        currentTheme = name
        nativeApplyTheme(name)
        Log.i("Waydri", "theme applied: $name")
    }

    fun current(): String = currentTheme

    fun themes(): List<String> = listOf(
        "default", "dark", "light", "oled",
        "nord", "dracula", "gruvbox", "solarized"
    )
}
