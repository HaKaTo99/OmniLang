#include "omnilang_qnx.h"

namespace OmniLang {

    std::string QNXBridge::get_version() {
        char* c_version = omnilang_get_native_version();
        if (!c_version) {
            return "Unknown OmniLang Native Version";
        }
        
        std::string version(c_version);
        
        // Deterministik de-alokasi QNX
        omnilang_free_string(c_version);
        
        return version;
    }

    std::string QNXBridge::evaluate(const std::string& script) {
        if (script.empty()) {
            return "Error: Script is empty";
        }
        
        // Panggil FFI ke Mesin Rust melalui antarmuka C
        char* c_result = omnilang_eval(script.c_str());
        
        // Cek anomali alokasi lintas-sistem
        if (!c_result) {
            return "Fatal Error: Evaluator returned null string format.";
        }
        
        // Rehidrasi data aman sebagai std::string
        std::string result(c_result);
        
        // Mencegah memory leak secara langsung
        omnilang_free_string(c_result);
        
        return result;
    }

} // namespace OmniLang
