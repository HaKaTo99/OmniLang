use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Tabs, List, ListItem},
    Terminal,
};
use std::io;
use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;

use crate::compiler::Compiler;

// ─── File Type Detection ─────────────────────────────────────────────
#[derive(Clone, Debug, PartialEq)]
enum FileType {
    Declarative,  // INTENT/RULE/POLICY — uses Core Engine
    Imperative,   // fn/let/match — uses omc Compiler
    Unknown,
}

fn detect_file_type(content: &str) -> FileType {
    let trimmed = content.trim();
    if trimmed.starts_with("INTENT:") || trimmed.starts_with("ACTOR:") || trimmed.starts_with("RULE:") {
        FileType::Declarative
    } else if trimmed.starts_with("fn ") || trimmed.starts_with("let ") || trimmed.starts_with("module ") {
        FileType::Imperative
    } else {
        // Scan first few lines for keywords
        for line in trimmed.lines().take(5) {
            let t = line.trim();
            if t.starts_with("INTENT:") || t.starts_with("ACTOR:") || t.starts_with("CONTEXT:") {
                return FileType::Declarative;
            }
            if t.starts_with("fn ") || t.starts_with("let ") {
                return FileType::Imperative;
            }
        }
        FileType::Unknown
    }
}

// ─── Explorer File Entry ─────────────────────────────────────────────
#[derive(Clone, Debug)]
struct FileEntry {
    display_name: String,
    path: Option<PathBuf>,
    is_dir: bool,
}

// ─── Core Engine Runner ──────────────────────────────────────────────
fn find_core_engine() -> Option<PathBuf> {
    // Try to find omnilang.exe in target/debug of parent directory
    let candidates = vec![
        PathBuf::from("..").join("target").join("debug").join("omnilang.exe"),
        PathBuf::from("..").join("target").join("release").join("omnilang.exe"),
        PathBuf::from("target").join("debug").join("omnilang.exe"),
    ];
    for c in candidates {
        if c.exists() {
            return Some(c);
        }
    }
    None
}

fn run_core_engine(file_path: &Path) -> (Vec<String>, String) {
    let engine = match find_core_engine() {
        Some(e) => e,
        None => {
            return (
                vec!["⚠ Core Engine not found".to_string(),
                     "".to_string(),
                     "Run 'cargo build' in the root OmniLang directory first.".to_string()],
                "// Core Engine binary not found".to_string(),
            );
        }
    };

    // Run exec command
    let exec_output = Command::new(&engine)
        .arg("exec")
        .arg(file_path)
        .output();

    let exec_result = match exec_output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            if out.status.success() {
                stdout
            } else {
                format!("Error:\n{}\n{}", stdout, stderr)
            }
        }
        Err(e) => format!("Failed to run Core Engine: {}", e),
    };

    // Run lint command
    let lint_output = Command::new(&engine)
        .arg("lint")
        .arg(file_path)
        .output();

    let lint_result = match lint_output {
        Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
        Err(e) => format!("Lint error: {}", e),
    };

    let runtime_lines: Vec<String> = exec_result.lines().map(|l| l.to_string()).collect();
    (runtime_lines, lint_result)
}

// ─── Main App ────────────────────────────────────────────────────────
pub struct App {
    input: String,
    current_file: String,
    current_file_path: Option<PathBuf>,
    file_type: FileType,
    // Compiler (omc) outputs
    ir_output: Vec<String>,
    rust_output: String,
    // Core Engine outputs
    runtime_output: Vec<String>,
    lint_output: String,
    // Chat (CUI) output
    chat_history: Vec<String>,
    chat_input: String,
    chat_mode: bool,
    // UI State
    status_msg: String,
    should_quit: bool,
    active_tab: usize,
    tabs: Vec<String>,
    file_entries: Vec<FileEntry>,
    selected_file_idx: usize,
    scroll_offset: u16,
}

impl App {
    pub fn new(input: String, file_path: Option<String>) -> Self {
        let current_file = file_path.as_ref()
            .map(|f| Path::new(f).file_name().unwrap_or_default().to_string_lossy().to_string())
            .unwrap_or_else(|| "untitled.omni".to_string());
        let file_type = detect_file_type(&input);
        let file_entries = Self::scan_files(&file_path);
        let current_file_path = file_path.as_ref().map(|f| PathBuf::from(f));

        let tabs = match file_type {
            FileType::Declarative => vec![
                "[1] Runtime".to_string(),
                "[2] Lint".to_string(),
                "[3] Logs".to_string(),
                "[4] Chat".to_string(),
            ],
            _ => vec![
                "[1] Omni IR".to_string(),
                "[2] Rust".to_string(),
                "[3] Logs".to_string(),
                "[4] Chat".to_string(),
            ],
        };

        let mut app = Self {
            input,
            current_file,
            current_file_path,
            file_type,
            ir_output: vec![],
            rust_output: String::new(),
            runtime_output: vec![],
            lint_output: String::new(),
            chat_history: vec![
                "🤖 OmniLang CUI v1.0".to_string(),
                "   Ketik perintah atau pertanyaan.".to_string(),
                "   Contoh: exec, lint, help, list".to_string(),
                "─────────────────────────────────".to_string(),
            ],
            chat_input: String::new(),
            chat_mode: false,
            status_msg: "READY | ↑↓: Nav | Enter: Open | r: Compile | c: Chat | q: Quit".to_string(),
            should_quit: false,
            active_tab: 0,
            tabs,
            file_entries,
            selected_file_idx: 0,
            scroll_offset: 0,
        };
        app.compile();
        app
    }

    // ─── File Scanning ───────────────────────────────────────────────
    fn scan_files(file_path: &Option<String>) -> Vec<FileEntry> {
        let mut entries = Vec::new();

        let base_dir = if let Some(fp) = file_path {
            let p = Path::new(fp);
            p.parent().map(|p| p.to_path_buf())
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
        } else {
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        };

        // Current directory
        let cwd_files = Self::find_omni_files(&base_dir);
        if !cwd_files.is_empty() {
            entries.push(FileEntry {
                display_name: format!("📂 {}", base_dir.file_name().unwrap_or_default().to_string_lossy()),
                path: None, is_dir: true,
            });
            for f in &cwd_files {
                let ft = fs::read_to_string(f).map(|c| detect_file_type(&c)).unwrap_or(FileType::Unknown);
                let icon = match ft {
                    FileType::Declarative => "📜",
                    FileType::Imperative => "⚙️",
                    FileType::Unknown => "📄",
                };
                entries.push(FileEntry {
                    display_name: format!("  {} {}", icon, f.file_name().unwrap_or_default().to_string_lossy()),
                    path: Some(f.clone()), is_dir: false,
                });
            }
        }

        // examples/
        for dir in &[base_dir.join("examples"), base_dir.join("..").join("examples")] {
            if dir.is_dir() {
                let files = Self::find_omni_files(dir);
                if !files.is_empty() {
                    entries.push(FileEntry {
                        display_name: "📂 examples".to_string(), path: None, is_dir: true,
                    });
                    for f in &files {
                        let ft = fs::read_to_string(f).map(|c| detect_file_type(&c)).unwrap_or(FileType::Unknown);
                        let icon = match ft {
                            FileType::Declarative => "📜",
                            FileType::Imperative => "⚙️",
                            FileType::Unknown => "📄",
                        };
                        entries.push(FileEntry {
                            display_name: format!("  {} {}", icon, f.file_name().unwrap_or_default().to_string_lossy()),
                            path: Some(f.clone()), is_dir: false,
                        });
                    }
                }
                break;
            }
        }

        // tests/
        for dir in &[base_dir.join("tests"), base_dir.join("..").join("tests")] {
            if dir.is_dir() {
                let files = Self::find_omni_files(dir);
                if !files.is_empty() {
                    entries.push(FileEntry {
                        display_name: "📂 tests".to_string(), path: None, is_dir: true,
                    });
                    for f in &files {
                        entries.push(FileEntry {
                            display_name: format!("  📋 {}", f.file_name().unwrap_or_default().to_string_lossy()),
                            path: Some(f.clone()), is_dir: false,
                        });
                    }
                }
                break;
            }
        }

        if entries.is_empty() {
            entries.push(FileEntry {
                display_name: "📄 (no .omni files found)".to_string(),
                path: None, is_dir: false,
            });
        }
        entries
    }

    fn find_omni_files(dir: &Path) -> Vec<PathBuf> {
        let mut files: Vec<PathBuf> = fs::read_dir(dir)
            .into_iter().flatten()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|ext| ext == "omni").unwrap_or(false))
            .map(|e| e.path())
            .collect();
        files.sort();
        files
    }

    // ─── File Operations ─────────────────────────────────────────────
    fn open_selected_file(&mut self) {
        if let Some(entry) = self.file_entries.get(self.selected_file_idx) {
            if let Some(path) = &entry.path {
                match fs::read_to_string(path) {
                    Ok(content) => {
                        self.file_type = detect_file_type(&content);
                        self.input = content;
                        self.current_file = path.file_name()
                            .unwrap_or_default().to_string_lossy().to_string();
                        self.current_file_path = Some(path.clone());
                        self.scroll_offset = 0;

                        // Update tabs based on file type
                        self.tabs = match self.file_type {
                            FileType::Declarative => vec![
                                "[1] Runtime".to_string(),
                                "[2] Lint".to_string(),
                                "[3] Logs".to_string(),
                                "[4] Chat".to_string(),
                            ],
                            _ => vec![
                                "[1] Omni IR".to_string(),
                                "[2] Rust".to_string(),
                                "[3] Logs".to_string(),
                                "[4] Chat".to_string(),
                            ],
                        };
                        self.active_tab = 0;
                        self.compile();
                    }
                    Err(e) => {
                        self.status_msg = format!("ERROR: {}", e);
                    }
                }
            }
        }
    }

    // ─── Dual-Engine Compilation ─────────────────────────────────────
    fn compile(&mut self) {
        match self.file_type {
            FileType::Declarative => self.compile_declarative(),
            FileType::Imperative => self.compile_imperative(),
            FileType::Unknown => {
                // Try imperative first, fall back to declarative
                self.compile_imperative();
                if self.ir_output.first().map(|s| s.contains("Error")).unwrap_or(false) {
                    self.compile_declarative();
                }
            }
        }
    }

    fn compile_declarative(&mut self) {
        if let Some(ref path) = self.current_file_path {
            let (runtime, lint) = run_core_engine(path);
            self.runtime_output = runtime;
            self.lint_output = lint;
            self.ir_output = vec![format!("Policy file: {} (Declarative Mode)", self.current_file)];
            self.rust_output = "// Declarative files use Core Engine, not Rust backend".to_string();

            let has_error = self.runtime_output.iter().any(|l| l.contains("Error"));
            if has_error {
                self.status_msg = format!(
                    "⚠ POLICY ERROR {} | ↑↓: Nav | Enter: Open | q: Quit",
                    self.current_file
                );
            } else {
                self.status_msg = format!(
                    "✅ POLICY OK {} [CORE ENGINE] | ↑↓: Nav | Enter: Open | q: Quit",
                    self.current_file
                );
            }
        } else {
            self.runtime_output = vec!["No file path available for Core Engine".to_string()];
            self.lint_output = String::new();
        }
    }

    fn compile_imperative(&mut self) {
        let prev_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));

        let input_clone = self.input.clone();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            Compiler::compile(&input_clone)
        }));

        std::panic::set_hook(prev_hook);

        match result {
            Ok(compile_result) => {
                if compile_result.success {
                    self.ir_output = compile_result.ir_output;
                    self.rust_output = compile_result.rust_code;
                    self.runtime_output = vec!["Imperative file — use tab [1] for IR".to_string()];
                    self.lint_output = "No policy lint for imperative files.".to_string();
                    self.status_msg = format!(
                        "✅ COMPILED {} [OMC] | ↑↓: Nav | Enter: Open | q: Quit",
                        self.current_file
                    );
                } else {
                    self.ir_output = vec!["Compilation Failed".to_string()];
                    self.rust_output = compile_result.errors.join("\n");
                    self.status_msg = format!("❌ ERRORS in {} | Check Output", self.current_file);
                }
            }
            Err(panic_info) => {
                let panic_msg = if let Some(s) = panic_info.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = panic_info.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "Unknown parser error".to_string()
                };
                self.ir_output = vec![
                    "⚠ Parser Error".to_string(), "".to_string(),
                    format!("  {}", panic_msg), "".to_string(),
                    "Supported: fn, let, if/else, match, print, return".to_string(),
                ];
                self.rust_output = format!("// Parser error: {}", panic_msg);
                self.status_msg = format!("⚠ SYNTAX ERROR {} | q: Quit", self.current_file);
            }
        }
    }

    // ─── CUI Chat Engine ─────────────────────────────────────────────
    fn process_chat_command(&mut self, cmd: &str) {
        let cmd = cmd.trim();
        self.chat_history.push(format!("▶ {}", cmd));

        match cmd {
            "help" => {
                self.chat_history.push("📖 Perintah tersedia:".to_string());
                self.chat_history.push("  exec     - Jalankan file aktif".to_string());
                self.chat_history.push("  lint     - Periksa kualitas policy".to_string());
                self.chat_history.push("  compile  - Compile ke IR/Rust".to_string());
                self.chat_history.push("  list     - Daftar file .omni".to_string());
                self.chat_history.push("  info     - Info file aktif".to_string());
                self.chat_history.push("  clear    - Bersihkan chat".to_string());
            }
            "exec" => {
                self.compile();
                self.chat_history.push(format!("🔄 Mengeksekusi {}...", self.current_file));
                match self.file_type {
                    FileType::Declarative => {
                        for line in &self.runtime_output {
                            self.chat_history.push(format!("  {}", line));
                        }
                    }
                    FileType::Imperative => {
                        self.chat_history.push("  ✅ Compiled to Rust successfully".to_string());
                        self.chat_history.push(format!("  {} IR instructions generated", self.ir_output.len()));
                    }
                    _ => {
                        self.chat_history.push("  ⚠ Unknown file type".to_string());
                    }
                }
            }
            "lint" => {
                self.chat_history.push(format!("🔍 Linting {}...", self.current_file));
                if self.lint_output.is_empty() || self.lint_output.contains("No issues") || self.lint_output.contains("clean") {
                    self.chat_history.push("  ✅ No issues found. Policy is clean!".to_string());
                } else {
                    for line in self.lint_output.lines() {
                        self.chat_history.push(format!("  {}", line));
                    }
                }
            }
            "compile" => {
                self.compile();
                self.chat_history.push(format!("🔧 Compiling {}...", self.current_file));
                self.chat_history.push(format!("  Engine: {}", match self.file_type {
                    FileType::Declarative => "Core Engine (Validator Runtime)",
                    FileType::Imperative => "omc Compiler (Rust Backend)",
                    FileType::Unknown => "Auto-detect",
                }));
                self.chat_history.push("  ✅ Done".to_string());
            }
            "list" => {
                self.chat_history.push("📂 File .omni yang tersedia:".to_string());
                for entry in &self.file_entries {
                    if !entry.is_dir {
                        self.chat_history.push(format!("  {}", entry.display_name.trim()));
                    }
                }
            }
            "info" => {
                self.chat_history.push(format!("📄 File: {}", self.current_file));
                self.chat_history.push(format!("   Type: {:?}", self.file_type));
                self.chat_history.push(format!("   Lines: {}", self.input.lines().count()));
                self.chat_history.push(format!("   Engine: {}", match self.file_type {
                    FileType::Declarative => "Core Engine",
                    FileType::Imperative => "omc Compiler",
                    FileType::Unknown => "Unknown",
                }));
            }
            "clear" => {
                self.chat_history.clear();
                self.chat_history.push("🤖 Chat cleared.".to_string());
            }
            _ => {
                // Check if it's a file name
                let maybe_file = self.file_entries.iter().position(|e| {
                    e.display_name.contains(cmd) && !e.is_dir
                });
                if let Some(idx) = maybe_file {
                    self.selected_file_idx = idx;
                    self.open_selected_file();
                    self.chat_history.push(format!("📂 Opened: {}", self.current_file));
                } else {
                    self.chat_history.push(format!("❓ Perintah tidak dikenal: '{}'", cmd));
                    self.chat_history.push("   Ketik 'help' untuk daftar perintah.".to_string());
                }
            }
        }
    }

    // ─── Event Loop ──────────────────────────────────────────────────
    pub fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|f| self.ui(f))?;

            if let Event::Key(key) = event::read()? {
                if self.chat_mode {
                    // Chat input mode
                    match key.code {
                        KeyCode::Esc => {
                            self.chat_mode = false;
                            self.status_msg = "READY | ↑↓: Nav | Enter: Open | r: Compile | c: Chat | q: Quit".to_string();
                        }
                        KeyCode::Enter => {
                            let cmd = self.chat_input.clone();
                            self.chat_input.clear();
                            if !cmd.is_empty() {
                                self.process_chat_command(&cmd);
                            }
                        }
                        KeyCode::Char(ch) => {
                            self.chat_input.push(ch);
                        }
                        KeyCode::Backspace => {
                            self.chat_input.pop();
                        }
                        _ => {}
                    }
                } else {
                    // Normal mode
                    match key.code {
                        KeyCode::Char('q') => self.should_quit = true,
                        KeyCode::Char('r') => self.compile(),
                        KeyCode::Char('c') => {
                            self.chat_mode = true;
                            self.active_tab = 3; // Chat tab
                            self.status_msg = "💬 CHAT MODE | Type command + Enter | Esc: Exit chat".to_string();
                        }
                        KeyCode::Char('1') => self.active_tab = 0,
                        KeyCode::Char('2') => self.active_tab = 1,
                        KeyCode::Char('3') => self.active_tab = 2,
                        KeyCode::Char('4') => self.active_tab = 3,
                        KeyCode::Up => {
                            if self.selected_file_idx > 0 {
                                self.selected_file_idx -= 1;
                                while self.selected_file_idx > 0
                                    && self.file_entries[self.selected_file_idx].is_dir {
                                    self.selected_file_idx -= 1;
                                }
                                if self.file_entries[self.selected_file_idx].is_dir
                                    && self.selected_file_idx + 1 < self.file_entries.len() {
                                    self.selected_file_idx += 1;
                                }
                            }
                        }
                        KeyCode::Down => {
                            if self.selected_file_idx + 1 < self.file_entries.len() {
                                self.selected_file_idx += 1;
                                while self.selected_file_idx < self.file_entries.len() - 1
                                    && self.file_entries[self.selected_file_idx].is_dir {
                                    self.selected_file_idx += 1;
                                }
                            }
                        }
                        KeyCode::Enter => self.open_selected_file(),
                        _ => {}
                    }
                }
            }

            if self.should_quit { break; }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        Ok(())
    }

    // ─── UI Rendering ────────────────────────────────────────────────
    fn ui(&self, f: &mut ratatui::Frame) {
        let root_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(1)].as_ref())
            .split(f.size());

        let main_area = root_chunks[0];
        let status_area = root_chunks[1];

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(22), Constraint::Percentage(78)].as_ref())
            .split(main_area);

        let sidebar_area = main_chunks[0];
        let editor_panel_area = main_chunks[1];

        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
            .split(editor_panel_area);

        let editor_area = right_chunks[0];
        let panel_area = right_chunks[1];

        // ─── 1. Sidebar (Explorer) ───────────────────────────────
        let file_items: Vec<ListItem> = self.file_entries.iter().enumerate().map(|(i, entry)| {
            let style = if i == self.selected_file_idx {
                Style::default().fg(Color::Yellow).bg(Color::DarkGray).add_modifier(Modifier::BOLD)
            } else if entry.is_dir {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            ListItem::new(entry.display_name.as_str()).style(style)
        }).collect();

        let engine_label = match self.file_type {
            FileType::Declarative => " EXPLORER [CORE] ",
            FileType::Imperative => " EXPLORER [OMC] ",
            FileType::Unknown => " EXPLORER ",
        };

        let sidebar = List::new(file_items)
            .block(Block::default().borders(Borders::RIGHT).title(engine_label)
                .style(Style::default().fg(Color::Blue)));
        f.render_widget(sidebar, sidebar_area);

        // ─── 2. Editor ───────────────────────────────────────────
        let lines: Vec<Line> = self.input.lines().enumerate().map(|(i, line)| {
            Line::from(vec![
                Span::styled(format!("{:3} ", i + 1), Style::default().fg(Color::DarkGray)),
                Span::raw(line),
            ])
        }).collect();

        let type_badge = match self.file_type {
            FileType::Declarative => "📜 ",
            FileType::Imperative => "⚙️ ",
            FileType::Unknown => "📄 ",
        };
        let editor_block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" {}{} ", type_badge, self.current_file))
            .style(Style::default().fg(match self.file_type {
                FileType::Declarative => Color::Green,
                FileType::Imperative => Color::Blue,
                FileType::Unknown => Color::Gray,
            }));
        let editor = Paragraph::new(lines)
            .block(editor_block)
            .style(Style::default().fg(Color::White));
        f.render_widget(editor, editor_area);

        // ─── 3. Panel (Tabs + Content) ───────────────────────────
        let titles: Vec<Line> = self.tabs.iter().enumerate().map(|(i, t)| {
            let color = if i == self.active_tab { Color::Yellow } else { Color::Green };
            Line::from(Span::styled(t.as_str(), Style::default().fg(color)))
        }).collect();

        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::TOP).title(" TERMINAL "))
            .select(self.active_tab)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

        let panel_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(0)].as_ref())
            .split(panel_area);

        f.render_widget(tabs, panel_chunks[0]);

        let content = match self.active_tab {
            0 => {
                match self.file_type {
                    FileType::Declarative => self.runtime_output.join("\n"),
                    _ => self.ir_output.join("\n"),
                }
            }
            1 => {
                match self.file_type {
                    FileType::Declarative => self.lint_output.clone(),
                    _ => self.rust_output.clone(),
                }
            }
            2 => {
                let mut logs = vec![
                    format!("[log] File: {}", self.current_file),
                    format!("[log] Type: {:?}", self.file_type),
                    format!("[log] Lines: {}", self.input.lines().count()),
                    format!("[log] Engine: {}", match self.file_type {
                        FileType::Declarative => "Core Engine (Validator Runtime)",
                        FileType::Imperative => "omc Compiler (Rust Backend)",
                        FileType::Unknown => "Auto-detect",
                    }),
                ];
                if find_core_engine().is_some() {
                    logs.push("[log] Core Engine: ✅ Available".to_string());
                } else {
                    logs.push("[log] Core Engine: ❌ Not found (run cargo build in root)".to_string());
                }
                logs.join("\n")
            }
            3 => {
                // Chat tab
                let mut output = self.chat_history.join("\n");
                if self.chat_mode {
                    output.push_str(&format!("\n\n> {}_", self.chat_input));
                }
                output
            }
            _ => String::new(),
        };

        let output = Paragraph::new(content)
            .block(Block::default().borders(Borders::NONE))
            .style(Style::default().fg(Color::Gray));
        f.render_widget(output, panel_chunks[1]);

        // ─── 4. Status Bar ───────────────────────────────────────
        let line_count = self.input.lines().count();
        let engine_tag = match self.file_type {
            FileType::Declarative => "CORE",
            FileType::Imperative => "OMC",
            FileType::Unknown => "AUTO",
        };
        let bg_color = match self.file_type {
            FileType::Declarative => Color::Rgb(0, 100, 50),    // Green
            FileType::Imperative => Color::Blue,
            FileType::Unknown => Color::DarkGray,
        };
        let status = Paragraph::new(Text::styled(
            format!(" {} | {} lines | {} | OmniLang v2.0.0-multi ", self.status_msg, line_count, engine_tag),
            Style::default().bg(bg_color).fg(Color::White)
        ));
        f.render_widget(status, status_area);
    }
}
