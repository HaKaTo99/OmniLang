import Foundation

/// Wrapper keamanan tingkat tinggi (Military-Grade) untuk engine native OmniLang.
/// Didesain khusus untuk integrasi lancar dengan Apple ARC (Automatic Reference Counting)
/// mencegah kebocoran memori atau kerentanan Double-Free pada aplikasi iOS/iPadOS.
public class OmniLangBridge {
    
    /// Status struktur hasil eksekusi OmniLang
    public enum ExecutionResult {
        case success(String)
        case error(String)
    }

    /// Mengambil versi native Rust dari engine OmniLang yang terangkai (*linked*).
    ///
    /// - Returns: String berisi versi core engine, e.g., "OmniLang Engine Core v2.3.5".
    public static func getNativeVersion() -> String {
        let cVersionPtr = omnilang_get_native_version()
        guard let ptr = cVersionPtr else { return "Unknown Version" }
        
        let versionStr = String(cString: ptr)
        // Keamanan Memori C-ABI: Mengembalikan kepemilikan memori ke Rust untuk di-drop
        omnilang_free_string(ptr)
        
        return versionStr
    }

    /// Mengeksekusi secara sinkronus (*Synchronous*) script berbahasa OmniLang murni.
    /// Dilindungi dari *Unwinding Panic* lintas-bahasa oleh Rust C-ABI Guard.
    ///
    /// - Parameter script: Kode sumber OmniLang.
    /// - Returns: Objek `ExecutionResult` yang membungkus nilai bersih (*Safe Type*).
    public static func evaluate(script: String) -> ExecutionResult {
        // Mengamankan string Swift ke C-String secara sementara (Scoped allocation)
        return script.withCString { cScriptPtr in
            // Menembus FFI (Foreign Function Interface) menuju mesin virtual Rust
            let cResultPtr = omnilang_eval(cScriptPtr)
            
            // Validasi Null-Pointer
            guard let ptr = cResultPtr else {
                return .error("Fatal Error: Engine returned null pointer.")
            }
            
            // Rehidrasi C-String ke High-level Swift String
            let resultStr = String(cString: ptr)
            
            // Fase Pembersihan Absolut: Hindari Memory Leak akibat beda Allocator
            omnilang_free_string(ptr)
            
            if resultStr.starts(with: "Error:") || resultStr.starts(with: "Parser Error:") || resultStr.starts(with: "Lexer Error:") || resultStr.contains("Fatal C-ABI Rust Panic") {
                return .error(resultStr)
            }
            
            return .success(resultStr)
        }
    }
}
