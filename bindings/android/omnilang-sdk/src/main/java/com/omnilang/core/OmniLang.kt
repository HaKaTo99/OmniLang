package com.omnilang.core

/**
 * Antarmuka FFI (Foreign Function Interface) Native Kotlin menuju Mesin Rust OmniLang.
 * Menautkan fungsionalitas deklarator `eval` ke `Java_com_omnilang_core_OmniLang_eval`
 * pada Shared Library `.so` yang telah difabrikasi oleh Cargo/NDK.
 */
class OmniLang {
    companion object {
        init {
            // Memuat pustaka JNI Native Rust `libomnilang_core.so`
            System.loadLibrary("omnilang_core")
        }
    }

    /**
     * Mengevaluasi skrip OmniLang murni dari Kotlin dan mengambil output string
     * akhir hasil eksekusi native.
     */
    external fun eval(code: String): String
}
