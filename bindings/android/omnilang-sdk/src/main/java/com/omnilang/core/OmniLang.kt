package com.omnilang.core

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import org.json.JSONObject

/**
 * OmniLang Result Wrapper for Type-Safe handling.
 */
sealed class OmniResult {
    data class Success(val output: String) : OmniResult()
    data class Error(val message: String, val type: String) : OmniResult()
}

/**
 * Antarmuka FFI (Foreign Function Interface) Native Kotlin menuju Mesin Rust OmniLang.
 * Versi 2.3.0 - High Stability & Async Support.
 */
class OmniLang {
    companion object {
        init {
            System.loadLibrary("omnilang_core")
        }
    }

    /**
     * Inisialisasi logging native ke Logcat.
     */
    external fun initLogging()

    /**
     * Mengambil versi native engine.
     */
    external fun getNativeVersion(): String

    /**
     * FFI murni (Private)
     */
    private external fun eval(code: String): String

    /**
     * Mengevaluasi skrip secara sinkronus dan mengembalikan objek terstruktur.
     */
    fun evaluate(code: String): OmniResult {
        val jsonStr = eval(code)
        return try {
            val json = JSONObject(jsonStr)
            val status = json.optString("status")
            if (status == "success") {
                OmniResult.Success(json.optString("result"))
            } else {
                OmniResult.Error(
                    json.optString("message", "UNKNOWN_ERROR"),
                    json.optString("type", "internal")
                )
            }
        } catch (e: Exception) {
            OmniResult.Error(e.message ?: "JSON_PARSE_FAILED", "sdk")
        }
    }

    /**
     * Mengevaluasi skrip secara asinkronus menggunakan Kotlin Coroutines.
     * Mencegah pemblokiran Main UI Thread (High Performance).
     */
    suspend fun evaluateAsync(code: String): OmniResult = withContext(Dispatchers.Default) {
        evaluate(code)
    }
}
