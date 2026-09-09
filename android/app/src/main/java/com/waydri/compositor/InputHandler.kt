package com.waydri.compositor

import android.view.MotionEvent
import android.view.View

class InputHandler : View.OnTouchListener {
    private var pointerId = -1
    private var lastX = 0f
    private var lastY = 0f

    private external fun nativePointer(action: Int, x: Float, y: Float, button: Int, timeMs: Long)

    override fun onTouch(v: View, event: MotionEvent): Boolean {
        when (event.actionMasked) {
            MotionEvent.ACTION_DOWN, MotionEvent.ACTION_POINTER_DOWN -> {
                val idx = event.actionIndex
                pointerId = event.getPointerId(idx)
                lastX = event.getX(idx)
                lastY = event.getY(idx)
                nativePointer(0, lastX, lastY, 1, event.eventTime)
                return true
            }
            MotionEvent.ACTION_MOVE -> {
                for (i in 0 until event.pointerCount) {
                    val pid = event.getPointerId(i)
                    if (pid == pointerId) {
                        val x = event.getX(i)
                        val y = event.getY(i)
                        if (x != lastX || y != lastY) {
                            nativePointer(1, x, y, 1, event.eventTime)
                            lastX = x
                            lastY = y
                        }
                        break
                    }
                }
                return true
            }
            MotionEvent.ACTION_UP, MotionEvent.ACTION_POINTER_UP -> {
                val idx = event.actionIndex
                val pid = event.getPointerId(idx)
                if (pid == pointerId) {
                    nativePointer(2, event.getX(idx), event.getY(idx), 0, event.eventTime)
                    pointerId = -1
                }
                return true
            }
            MotionEvent.ACTION_CANCEL -> {
                nativePointer(3, 0f, 0f, 0, event.eventTime)
                pointerId = -1
                return true
            }
        }
        return false
    }
}
