package com.example.nlog

class NativeLib {

    /**
     * A native method that is implemented by the 'golog' native library,
     * which is packaged with this application.
     */
    external fun stringFromJNI(): String

    external fun initLog(dir: String)
    external fun closeLogger()

    external fun logDebug(tag: String, message: String)
    external fun logInfo(tag: String, message: String)
    external fun logWarn(tag: String, message: String)
    external fun logError(tag: String, message: String)

    companion object {
        init {
//            System.loadLibrary("rustlog")
            System.loadLibrary("nlog")
        }
    }
}