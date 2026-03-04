#ifndef OMNILANG_IOS_H
#define OMNILANG_IOS_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Mendapatkan versi native dari OmniLang Engine (Rust Core)
 * @return C-String yang berisi versi engine. 
 * Memori dikelola oleh Rust, Swift harus memanggil `omnilang_free_string` setelah selesai.
 */
char* omnilang_get_native_version();

/**
 * Mengeksekusi script OmniLang via C-ABI.
 * Dilengkapi dengan Military-Grade Panic Guard dari modul Rust.
 * 
 * @param code String null-terminated berisi kode OmniLang.
 * @return C-String berisi hasil kompilasi/eksekusi atau pesan error.
 * Memori dikelola oleh Rust, Swift harus memanggil `omnilang_free_string` setelah selesai.
 */
char* omnilang_eval(const char* code);

/**
 * Melepaskan memori string yang dialokasikan oleh engine Rust.
 * Wajib dipanggil untuk mencegah kebocoran memori (Memory Leak) pada perangkat Apple (iOS/macOS).
 * 
 * @param s Pointer C-String yang dikembalikan oleh OmniLang.
 */
void omnilang_free_string(char* s);

#ifdef __cplusplus
}
#endif

#endif // OMNILANG_IOS_H
