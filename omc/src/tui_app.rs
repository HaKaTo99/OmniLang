use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use std::io;

use crate::compiler::Compiler;

pub struct App {
    input: String,
    ir_output: Vec<String>,
    rust_output: String,
    status_msg: String,
    should_quit: bool,
}

impl App {
    pub fn new(input: String) -> Self {
        let mut app = Self {
            input,
            ir_output: vec![],
            rust_output: String::new(),
            status_msg: "Press 'r' to Compile | 'q' to Quit".to_string(),
            should_quit: false,
        };
        app.compile();
        app
    }

    fn compile(&mut self) {
        let result = Compiler::compile(&self.input);
        if result.success {
            self.ir_output = result.ir_output;
            self.rust_output = result.rust_code;
            self.status_msg = "Compilation Successful! | 'r' to Recompile | 'q' to Quit".to_string();
        } else {
            self.ir_output = vec!["Compilation Failed".to_string()];
            self.rust_output = result.errors.join("\n");
            self.status_msg = "Errors Found! Check monitors.".to_string();
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|f| self.ui(f))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => self.should_quit = true,
                    KeyCode::Char('r') => self.compile(),
                    _ => {}
                }
            }

            if self.should_quit {
                break;
            }
        }

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen
        )?;
        terminal.show_cursor()?;
        Ok(())
    }

    fn ui(&self, f: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3), // Title
                    Constraint::Min(10),   // Main Content
                    Constraint::Length(3), // Status Bar
                ]
                .as_ref(),
            )
            .split(f.size());

        // 1. Title Block
        let title = Paragraph::new(Text::styled(
            " OMNILANG WORKSTATION v1.1 :: MILITARY GRADE IDE ",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(ratatui::widgets::BorderType::Thick),
        );
        f.render_widget(title, chunks[0]);

        // 2. Main Content (Split 3 ways)
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ]
                .as_ref(),
            )
            .split(chunks[1]);

        // Panel 1: Source Code
        let source_block = Block::default().borders(Borders::ALL).title(" 📄 Source Code ");
        let source_text = Paragraph::new(self.input.clone())
            .block(source_block)
            .style(Style::default().fg(Color::Green));
        f.render_widget(source_text, main_chunks[0]);

        // Panel 2: IR Monitor
        let ir_block = Block::default().borders(Borders::ALL).title(" ⚙️ Omni VM Monitor ");
        let ir_content = self.ir_output.join("\n");
        let ir_text = Paragraph::new(ir_content)
            .block(ir_block)
            .style(Style::default().fg(Color::Magenta))
            .wrap(Wrap { trim: true });
        f.render_widget(ir_text, main_chunks[1]);

        // Panel 3: Rust Output
        let rust_block = Block::default().borders(Borders::ALL).title(" 🚀 Rust Output ");
        let rust_text = Paragraph::new(self.rust_output.clone())
            .block(rust_block)
            .style(Style::default().fg(Color::Yellow))
            .wrap(Wrap { trim: true });
        f.render_widget(rust_text, main_chunks[2]);

        // 3. Status Bar
        let status_block = Block::default().borders(Borders::ALL).title(" Status ");
        let status_text = Paragraph::new(self.status_msg.clone())
            .block(status_block)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Cyan));
        f.render_widget(status_text, chunks[2]);
    }
}
