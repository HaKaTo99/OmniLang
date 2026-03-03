#include <iostream>
#include <vector>

// Forward declarations for the Rust C-ABI FFI library
extern "C" {
    char* omnilang_eval(const char* code);
    void omnilang_free_string(char* s);
}

int main() {
    std::cout << "--- OmniLang C-ABI Mock Test ---" << std::endl;
    
    // Test code directly executing the OmniLang engine via C-string
    const char* code = "let greeting = \"Hello from HarmonyOS C++ Mock Test!\"; greeting";
    std::cout << "Evaluating code: " << code << std::endl;
    
    char* c_result = omnilang_eval(code);
    
    if (c_result != nullptr) {
        std::cout << "Result: " << c_result << std::endl;
        // Verify no memory leaks by calling the OmniLang memory destructor
        omnilang_free_string(c_result);
        std::cout << "Memory freed successfully." << std::endl;
    } else {
        std::cout << "Result: (null pointer error)" << std::endl;
    }
    
    return 0;
}
