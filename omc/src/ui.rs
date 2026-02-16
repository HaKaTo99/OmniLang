pub struct UI;

impl UI {
    // ANSI Colors & Styles
    const RESET: &'static str = "\x1b[0m";
    const BOLD: &'static str = "\x1b[1m";
    const DIM: &'static str = "\x1b[2m";
    const CYAN: &'static str = "\x1b[36m";
    const GREEN: &'static str = "\x1b[32m";
    const YELLOW: &'static str = "\x1b[33m";
    const RED: &'static str = "\x1b[31m";
    const BLUE: &'static str = "\x1b[34m";
    const GREY: &'static str = "\x1b[90m";
    const WHITE: &'static str = "\x1b[97m";

    pub fn banner() {
        println!("{}{}", Self::BOLD, Self::CYAN);
        println!(r#"
  ╔═══════════════════════════════════════════════════════════╗
  ║                                                           ║
  ║    ██████╗ ███╗   ███╗ ██████╗                            ║
  ║   ██╔═══██╗████╗ ████║██╔════╝                            ║
  ║   ██║   ██║██╔████╔██║██║                                 ║
  ║   ██║   ██║██║╚██╔╝██║██║                                 ║
  ║   ╚██████╔╝██║ ╚═╝ ██║╚██████╗                            ║
  ║    ╚═════╝ ╚═╝     ╚═╝ ╚═════╝                            ║
  ║                                                           ║
  ║   {}OmniLang Compiler{} {}v1.2.0-dev{}                            ║
  ║   {}Universal Intent Language for Autonomous Systems{}        ║
  ║                                                           ║
  ╚═══════════════════════════════════════════════════════════╝
        "#, Self::WHITE, Self::CYAN, Self::GREY, Self::CYAN, Self::DIM, Self::CYAN);
        println!("{}", Self::RESET);
    }

    pub fn section(title: &str) {
        println!("\n{}{}── {} ──{}", Self::BOLD, Self::BLUE, title, Self::RESET);
    }

    pub fn info(msg: &str) {
        println!("  {}{}ℹ{} {}", Self::BOLD, Self::BLUE, Self::RESET, msg);
    }

    pub fn success(msg: &str) {
        println!("  {}{}✔{} {}{}{}", Self::BOLD, Self::GREEN, Self::RESET, Self::GREEN, msg, Self::RESET);
    }

    pub fn warning(msg: &str) {
        println!("  {}{}⚠{} {}{}{}", Self::BOLD, Self::YELLOW, Self::RESET, Self::YELLOW, msg, Self::RESET);
    }

    pub fn error(msg: &str) {
        println!("  {}{}✖{} {}{}{}", Self::BOLD, Self::RED, Self::RESET, Self::RED, msg, Self::RESET);
    }

    pub fn step(current: usize, total: usize, name: &str) {
        let progress = "█".repeat(current).to_string() + &"░".repeat(total - current);
        println!("  {}[{}]{} {}{} {}{}", 
            Self::DIM, progress, Self::RESET,
            Self::BOLD, name, Self::RESET, Self::DIM);
    }
    
    pub fn code_block(code: &str) {
        println!("{}{}┌─────────────────────────────────{}", Self::DIM, Self::GREY, Self::RESET);
        for (i, line) in code.lines().enumerate() {
            println!("{}{}{:3} │{} {}", Self::DIM, Self::GREY, i + 1, Self::RESET, line);
        }
        println!("{}{}└─────────────────────────────────{}", Self::DIM, Self::GREY, Self::RESET);
    }

}
