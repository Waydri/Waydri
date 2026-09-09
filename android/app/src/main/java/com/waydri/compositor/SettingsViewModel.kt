package com.waydri.compositor

import android.content.Context
import android.content.SharedPreferences

class SettingsViewModel(context: Context) {
    private val prefs: SharedPreferences =
        context.getSharedPreferences("waydri_settings", Context.MODE_PRIVATE)

    fun getString(key: String, default: String): String = prefs.getString(key, default) ?: default
    fun setString(key: String, value: String) { prefs.edit().putString(key, value).apply() }

    fun getInt(key: String, default: Int): Int = prefs.getInt(key, default)
    fun setInt(key: String, value: Int) { prefs.edit().putInt(key, value).apply() }

    fun getFloat(key: String, default: Float): Float = prefs.getFloat(key, default)
    fun setFloat(key: String, value: Float) { prefs.edit().putFloat(key, value).apply() }

    fun getBool(key: String, default: Boolean): Boolean = prefs.getBoolean(key, default)
    fun setBool(key: String, value: Boolean) { prefs.edit().putBoolean(key, value).apply() }

    fun resetAll() { prefs.edit().clear().apply() }
}
