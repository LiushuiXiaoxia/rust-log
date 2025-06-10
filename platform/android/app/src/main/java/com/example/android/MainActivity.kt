package com.example.android

import android.os.Bundle
import android.util.Log
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.example.android.ui.theme.AndroidTheme
import com.example.nlog.NativeLib
import java.io.File
import kotlin.concurrent.thread

class MainActivity : ComponentActivity() {

    companion object {
        private const val TAG = "MainActivity"
    }

    val na = NativeLib()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            AndroidTheme {
                Scaffold(modifier = Modifier.fillMaxSize()) { innerPadding ->
                    Greeting(
                        name = "Android ${na.stringFromJNI()}",
                        modifier = Modifier.padding(innerPadding)
                    )
                }
            }
        }

        thread {
            val s = na.stringFromJNI()
            Log.i(TAG, "stringFromJNI: $s")

            val d = File(this.dataDir, "log").apply { mkdirs() }
            Log.e(TAG, "onCreate: d = ${d.absolutePath}")
            na.initLog(d.absolutePath)
            repeat(10) {
                na.logDebug(TAG, "Hello from thread $it")
                na.logInfo(TAG, "Hello from thread $it")
                na.logWarn(TAG, "Hello from thread $it")
                na.logError(TAG, "Hello from thread $it")
            }
            na.closeLogger()

            val f = d.listFiles()?.minByOrNull { it.lastModified() }
            Log.e(TAG, "onCreate: ${d.listFiles()?.map { it.name }}")
            val liens = f?.readLines()?.size ?: 0
            runOnUiThread {
                Toast.makeText(this, "Logs test success. lines = $liens", Toast.LENGTH_LONG).show()
            }
        }
    }
}

@Composable
fun Greeting(name: String, modifier: Modifier = Modifier) {
    Text(
        text = "Hello $name!",
        modifier = modifier
    )
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    AndroidTheme {
        Greeting("Android")
    }
}