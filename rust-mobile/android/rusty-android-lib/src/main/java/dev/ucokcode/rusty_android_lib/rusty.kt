package dev.ucokcode.rusty_android_lib


external fun hello(to: String): String
external fun helloDirect(to: String): String

fun loadRustyLib() {
    println("hallo world")
    System.loadLibrary("rustylib")
}