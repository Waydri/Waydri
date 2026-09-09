package com.waydri.compositor

import android.os.Binder
import android.os.IBinder
import android.util.Log
import android.app.Service
import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.Intent
import android.os.Build

class WaydriService : Service() {
    private val binder = WaydriBinder()
    private val compositorManager = CompositorManager()
    private val pluginManager = PluginManager()

    inner class WaydriBinder : Binder() {
        fun getService(): WaydriService = this@WaydriService
    }

    override fun onBind(intent: Intent?): IBinder = binder

    override fun onCreate() {
        super.onCreate()
        createNotificationChannel()
        Log.i("Waydri", "service created")
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        startForeground(NOTIFICATION_ID, buildNotification())
        compositorManager.start()
        pluginManager.loadAll(filesDir.resolve("plugins"))
        return START_STICKY
    }

    override fun onDestroy() {
        compositorManager.stop()
        super.onDestroy()
        Log.i("Waydri", "service destroyed")
    }

    fun compositorManager(): CompositorManager = compositorManager
    fun pluginManager(): PluginManager = pluginManager

    private fun createNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                CHANNEL_ID,
                "Waydri Compositor",
                NotificationManager.IMPORTANCE_LOW
            ).apply {
                description = "Compositor running"
                setShowBadge(false)
            }
            val nm = getSystemService(NotificationManager::class.java)
            nm.createNotificationChannel(channel)
        }
    }

    private fun buildNotification(): Notification {
        val builder = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            Notification.Builder(this, CHANNEL_ID)
        } else {
            @Suppress("DEPRECATION")
            Notification.Builder(this)
        }
        return builder
            .setContentTitle("Waydri")
            .setContentText("Compositor active")
            .setSmallIcon(android.R.drawable.ic_menu_manage)
            .setOngoing(true)
            .build()
    }

    companion object {
        private const val CHANNEL_ID = "waydri_compositor"
        private const val NOTIFICATION_ID = 1
    }
}
