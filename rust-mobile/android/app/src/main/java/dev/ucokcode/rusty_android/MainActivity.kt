package dev.ucokcode.rusty_android

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.widget.TextView

import dev.ucokcode.rusty_android_lib.*

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        loadRustyLib()
        findViewById<TextView>(R.id.txt).let {
            it?.text = hello("Rob")
        }

    }
}