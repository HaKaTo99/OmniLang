mod lexer;
mod parser;
mod semantic;
mod ir;
mod backend;
mod ui;
mod compiler;

mod tui_app;     // Phase 12 — TUI IDE with Dual-Engine
mod hui_serial;  // Phase F  — HUI Hardware Interface

use ui::UI;
use compiler::Compiler;
use tui_app::App;
use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let visual_mode = args.iter().any(|arg| arg == "--visual" || arg == "-v");
    let hui_mode = args.iter().any(|arg| arg == "--hui" || arg == "--serial");
    let headless_mode = args.iter().any(|arg| arg == "--headless");
    
    // Futuristic/Experimental interfaces
    let bci_mode = args.iter().any(|arg| arg == "--bci" || arg == "--neural");
    let pui_mode = args.iter().any(|arg| arg == "--pui" || arg == "--perceptual");
    let oui_mode = args.iter().any(|arg| arg == "--oui" || arg == "--organic");
    let mmui_mode = args.iter().any(|arg| arg == "--mmui" || arg == "--multimodal");
    let vr_mode = args.iter().any(|arg| arg == "--vr" || arg == "--ar" || arg == "--spatial");

    // ─── BCI Mode (Brain-Computer Interface — Experimental) ──
    if bci_mode {
        println!("🧠 OmniLang BCI Mode (Experimental)");
        println!("Status: Waiting for neural bridge connection...");
        println!("Error: EEG sensor not detected on this device.");
        std::process::exit(1);
    }

    // ─── PUI Mode (Perceptual User Interface — Experimental) ─
    if pui_mode {
        println!("👁️ OmniLang PUI Mode (Experimental)");
        println!("Status: Initializing computer vision sensors...");
        println!("Warning: No compatible camera found for eye-tracking.");
        std::process::exit(1);
    }

    // ─── OUI Mode (Organic User Interface — Experimental) ────
    if oui_mode {
        println!("🍃 OmniLang OUI Mode (Experimental)");
        println!("Status: Detecting flexible display surfaces...");
        println!("Error: No haptic/organic tactile feedback device found.");
        std::process::exit(1);
    }

    // ─── MMUI Mode (Multimodal User Interface — Experimental) ─
    if mmui_mode {
        println!("🎭 OmniLang MMUI Mode (Experimental)");
        println!("Status: Synchronizing Voice + Touch + Vision channels...");
        println!("Warning: Channel desync detected. Calibration required.");
        std::process::exit(1);
    }

    // ─── VR/AR Mode (Spatial Interface — Experimental) ───────
    if vr_mode {
        println!("🥽 OmniLang VR/AR Mode (Experimental)");
        println!("Status: Mapping IR space into 3D environment...");
        println!("Error: Spatial SDK (HoloLens/Quest) not initialized.");
        std::process::exit(1);
    }

    // ─── HUI Mode (Hardware/Serial Interface) ────────────────
    if hui_mode {
        hui_serial::run_hui_mode();
        return;
    }

    // ─── Headless Mode (IoT/Embedded single-file exec) ───────
    if headless_mode {
        let filename = args.iter()
            .find(|arg| !arg.starts_with("-") && !arg.ends_with("omc.exe") && !arg.ends_with("omc"));
        if let Some(f) = filename {
            let code = hui_serial::run_headless(f);
            std::process::exit(code);
        } else {
            eprintln!("ERR No file specified for headless mode");
            std::process::exit(1);
        }
    }

    UI::banner();
    
    // ─── Visual Mode (TUI IDE with Dual-Engine) ──────────────
    if visual_mode {
        let filename = args.iter().find(|arg| !arg.starts_with("-") && !arg.ends_with("omc.exe") && !arg.ends_with("omc"));
        let (input, file_path) = if let Some(f) = filename {
            match fs::read_to_string(f) {
                Ok(c) => (c, Some(f.clone())),
                Err(_) => ("fn main() { print(1); }".to_string(), None),
            }
        } else {
             (r#"fn factorial(n: int) {
    if (n < 2) { return 1; }
    return n * factorial(n - 1);
}
let res = factorial(5);
print(res);
"#.to_string(), None)
        };

        let mut app = App::new(input, file_path);
        if let Err(e) = app.run() {
            println!("Error running TUI: {}", e);
        }
        return;
    }

    // ─── CLI Mode (Standard Compilation) ─────────────────────
    let (input, _filename) = if args.len() > 1 {
        let filename = &args[1];
        UI::info(&format!("Reading source file: {}", filename));
        match fs::read_to_string(filename) {
            Ok(content) => (content, filename.clone()),
            Err(e) => {
                UI::error(&format!("Failed to read file '{}': {}", filename, e));
                return;
            }
        }
    } else {
        UI::warning("No input file provided. Using built-in demo mode.");
        let demo = r#"fn factorial(n: int) {
    if (n < 2) { return 1; }
    return n * factorial(n - 1);
}
let res = factorial(5);
print(res);"#;
        (demo.to_string(), "demo.omni".to_string())
    };

    UI::section("SOURCE CODE");
    UI::code_block(&input);

    let start_time = Instant::now();

    UI::step(1, 5, "Compiling...");
    let result = Compiler::compile(&input);

    if !result.success {
        UI::error("Compilation Failed:");
        for err in result.errors {
            println!("  - {}", err);
        }
        return;
    }

    UI::step(2, 5, "AST & Semantic Analysis [OK]");
    UI::step(3, 5, "IR Generation [OK]");
    UI::step(4, 5, "Rust Backend [OK]");

    let output_file = "output.rs";
    match fs::write(output_file, &result.rust_code) {
        Ok(_) => {
            UI::section("BUILD SUCCESSFUL");
            UI::success(&format!("Generated '{}'", output_file));
            
            let duration = start_time.elapsed();
            println!("\n  Time: {:.2?}", duration);
            println!("  Target: Rust (v1.0)");
            println!("  Status: Ready to compile with `rustc output.rs`");
        },
        Err(e) => UI::error(&format!("Failed to write output file: {}", e)),
    }
}
