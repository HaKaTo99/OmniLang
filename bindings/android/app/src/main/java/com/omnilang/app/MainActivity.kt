package com.omnilang.app

import android.app.Activity
import android.os.Bundle
import android.widget.TextView
import com.omnilang.core.OmniLang

class MainActivity : Activity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        val tv = TextView(this)
        
        try {
            // Membangkitkan Kelas JNI dari OmniLang SDK
            val omni = OmniLang()
            
            // Evaluasi Kode Skrip Sederhana OmniLang (Demonstrasi)
            val script = """
            const main: i32 = {
                print("🚀 Menjalankan OmniLang Native Engine di atas Android (JNI)!");
                print("Menghitung OODA Loop tiruan: 5 * 2 =");
                print(5 * 2);
                0
            };
            """.trimIndent()

            // Menjalankan fungsi Native Rust dan Menangkap Hasilnya sebagai String Kotlin
            val result = omni.eval(script)
            tv.text = "--- [Output JVM Bridge] ---\n$result"
            
        } catch (e: Exception) {
            tv.text = "Gagal memuat pustaka Native (libomnilang_core.so belum di-compile untuk arsitektur ini):\n${e.message}"
        }
        
        setContentView(tv)
    }
}
