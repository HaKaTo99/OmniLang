#include "napi/native_api.h"
#include <string>
#include <vector>

// Forward declarations for the Rust C-ABI FFI library
extern "C" {
    char* omnilang_eval(const char* code);
    void omnilang_free_string(char* s);
}

// Wrapping C++ evaluation logic
static napi_value EvalOmniLang(napi_env env, napi_callback_info info) {
    size_t argc = 1;
    napi_value args[1];
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);

    if (argc < 1) {
        napi_throw_error(env, nullptr, "Missing argument: expected OmniLang code string.");
        return nullptr;
    }

    // Convert ArkTS string to C++ string
    size_t str_len;
    napi_get_value_string_utf8(env, args[0], nullptr, 0, &str_len);
    std::vector<char> str_buf(str_len + 1);
    napi_get_value_string_utf8(env, args[0], str_buf.data(), str_len + 1, &str_len);

    const char* c_code = str_buf.data();

    // Call Rust Core
    char* c_result = omnilang_eval(c_code);

    // Convert Result back to ArkTS string
    napi_value result;
    napi_create_string_utf8(env, c_result, NAPI_AUTO_LENGTH, &result);

    // Free Rust Memory
    omnilang_free_string(c_result);

    return result;
}

// NAPI module registration
static napi_value Init(napi_env env, napi_value exports) {
    napi_property_descriptor desc[] = {
        { "eval", nullptr, EvalOmniLang, nullptr, nullptr, nullptr, napi_default, nullptr }
    };
    napi_define_properties(env, exports, sizeof(desc) / sizeof(desc[0]), desc);
    return exports;
}

// Module descriptor
static napi_module omnilangModule = {
    .nm_version = 1,
    .nm_flags = 0,
    .nm_filename = nullptr,
    .nm_register_func = Init,
    .nm_modname = "omnilang_napi",
    .nm_priv = nullptr,
    .reserved = { 0 },
};

extern "C" __attribute__((constructor)) void RegisterOmniLangModule() {
    napi_module_register(&omnilangModule);
}
