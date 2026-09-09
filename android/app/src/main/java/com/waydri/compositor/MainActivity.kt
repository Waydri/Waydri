package com.waydri.compositor

import android.app.Activity
import android.os.Bundle
import android.view.View
import android.view.WindowManager
import androidx.appcompat.app.AppCompatActivity
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat

class MainActivity : AppCompatActivity() {
    private var surfaceView: WaydriSurfaceView? = null
    private val compositorManager = CompositorManager()
    private val inputHandler = InputHandler()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        window.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
        enterImmersive()
        surfaceView = WaydriSurfaceView(this)
        surfaceView!!.inputHandler = inputHandler
        surfaceView!!.setOnTouchListener(inputHandler)
        setContentView(surfaceView!!)
        compositorManager.start()
    }

    override fun onWindowFocusChanged(hasFocus: Boolean) {
        super.onWindowFocusChanged(hasFocus)
        if (hasFocus) {
            enterImmersive()
            compositorManager.resume()
        } else {
            compositorManager.pause()
        }
    }

    override fun onPause() {
        super.onPause()
        compositorManager.pause()
    }

    override fun onDestroy() {
        compositorManager.stop()
        super.onDestroy()
    }

    private fun enterImmersive() {
        val controller = WindowInsetsControllerCompat(window, window.decorView)
        controller.hide(WindowInsetsCompat.Type.systemBars())
        controller.systemBarsBehavior =
            WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
        window.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
    }
}
