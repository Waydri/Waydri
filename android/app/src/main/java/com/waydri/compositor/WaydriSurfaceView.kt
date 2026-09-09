package com.waydri.compositor

import android.util.Log
import android.view.SurfaceHolder
import android.view.SurfaceView

class WaydriSurfaceView(context: android.content.Context) : SurfaceView(context),
    SurfaceHolder.Callback {

    var inputHandler: InputHandler? = null

    private external fun nativeSurfaceCreated(surface: Any?, w: Int, h: Int)
    private external fun nativeSurfaceChanged(w: Int, h: Int)
    private external fun nativeSurfaceDestroyed()

    init {
        holder.addCallback(this)
    }

    override fun surfaceCreated(holder: SurfaceHolder) {
        val s = holder.surface
        nativeSurfaceCreated(s, width, height)
        Log.i("Waydri", "surfaceCreated ${width}x${height}")
    }

    override fun surfaceChanged(holder: SurfaceHolder, format: Int, w: Int, h: Int) {
        nativeSurfaceChanged(w, h)
    }

    override fun surfaceDestroyed(holder: SurfaceHolder) {
        nativeSurfaceDestroyed()
        Log.i("Waydri", "surfaceDestroyed")
    }
}
