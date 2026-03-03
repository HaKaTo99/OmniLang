// OmniLang.ets
// ArkTS interface for the HarmonyOS Native C++ Bridge

import omnilang_napi from 'libomnilang_napi.so';

export class OmniLang {
    /**
     * Executes an OmniLang source code string through the Native C++ Bridge
     * which forwards the call to the Rust Evaluation Engine.
     * 
     * @param code The OmniLang script to evaluate.
     * @returns The evaluated output string from the execution core.
     */
    static eval(code: string): string {
        try {
            // Under the hood, this calls the `EvalOmniLang` C++ function
            // defined in napi_init.cpp
            return omnilang_napi.eval(code);
        } catch (e) {
            return `Native Execution Error: ${e}`;
        }
    }
}
