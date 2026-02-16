pub struct UI;

impl UI {
    // ANSI Colors
    const RESET: &'static str = "\x1b[0m";
    const BOLD: &'static str = "\x1b[1m";
    const CYAN: &'static str = "\x1b[36m";
    const GREEN: &'static str = "\x1b[32m";
    const YELLOW: &'static str = "\x1b[33m";
    const RED: &'static str = "\x1b[31m";
    const MAGENTA: &'static str = "\x1b[35m";
    const BLUE: &'static str = "\x1b[34m";
    const GREY: &'static str = "\x1b[90m";

    pub fn banner() {
        println!("{}", Self::CYAN);
        println!(r#"
   ____  __  __  _   _  ___  _          _    _   _   ____ 
  / __ \|  \/  || \ | ||_ _|| |        / \  | \ | | / ___|
 | |  | | |\/| ||  \| | | | | |       / _ \ |  \| || |  _ 
 | |__| | |  | || |\  | | | | |___   / ___ \| |\  || |_| |
  \____/|_|  |_||_| \_||___||_____| /_/   \_\_| \_| \____|
                                                          
      :: v1.0.0-rc1 :: Military Grade Compiler ::
        "#);
        println!("{}", Self::RESET);
    }

    pub fn section(title: &str) {
        println!("\n{}>> {} {}{}", Self::MAGENTA, title, Self::RESET, Self::GREY);
        println!("----------------------------------------{}", Self::RESET);
    }

    pub fn info(msg: &str) {
        println!("  {}ℹ {}{}", Self::BLUE, Self::RESET, msg);
    }

    pub fn success(msg: &str) {
        println!("  {}✔ {}{}", Self::GREEN, Self::RESET, msg);
    }

    pub fn warning(msg: &str) {
        println!("  {}⚠ {}{}", Self::YELLOW, Self::RESET, msg);
    }

    pub fn error(msg: &str) {
        println!("  {}✖ {}{}", Self::RED, Self::RESET, msg);
    }

    pub fn step(current: usize, total: usize, name: &str) {
        println!("{} [{}/{}] {}{}", Self::BOLD, current, total, name, Self::RESET);
    }
    
    pub fn code_block(code: &str) {
        println!("{}", Self::GREY);
        for line in code.lines() {
            println!("  | {}", line);
        }
        println!("{}", Self::RESET);
    }
}
