#ifndef OMNILANG_QNX_H
#define OMNILANG_QNX_H

#ifdef __cplusplus
extern "C" {
#endif

/* Native Engine C-ABI Bindings for RTOS (Real-Time Operating Systems) */
char* omnilang_get_native_version();
char* omnilang_eval(const char* code);
void omnilang_free_string(char* s);

#ifdef __cplusplus
}
#endif

// QNX-specific C++ Smart Wrapper
#ifdef __cplusplus
#include <string>
#include <stdexcept>

namespace OmniLang {

    /**
     * Military-Grade C++ Wrapper untuk BlackBerry QNX System.
     * Menggunakan pendekatan RAII (Resource Acquisition Is Initialization)
     * untuk menjamin bahwa memori pointer Native FFI selalu didealokasi 
     * secara deterministik pada sistem kendaraan masa depan atau ruang industri IoT.
     */
    class QNXBridge {
    public:
        static std::string get_version();
        static std::string evaluate(const std::string& script);
    };

} // namespace OmniLang
#endif

#endif // OMNILANG_QNX_H
