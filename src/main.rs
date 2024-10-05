// SCAN FOR MUSIC FILES
use walkdir::WalkDir;

fn scan_music_files(directory: &str) -> Vec<String> {
    let mut music_files = Vec::new();
    for entry in WalkDir::new(directory) {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "mp3" || extension == "wav" || extension == "ogg" {
                    music_files.push(path.display().to_string());
                }
            }
        }
    }
    music_files
}

// PLAY AUDIO FILE
use rodio::{Decoder, OutputStream};
use rodio::Source;
use std::fs::File;
use std::io::BufReader;

fn play_music(file_path: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = File::open(file_path).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();
    
    stream_handle.play_raw(source.convert_samples()).unwrap();
}

// TUI INTERFACE
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use std::io;

fn draw_ui<B: tui::backend::Backend>(terminal: &mut Terminal<B>, music_files: &[String], selected: usize) -> Result<(), io::Error> {
    let items: Vec<ListItem> = music_files.iter().map(|f| ListItem::new(f.as_str())).collect();
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Music Library"))
        .highlight_style(tui::style::Style::default().fg(tui::style::Color::Yellow))
        .highlight_symbol("> ");
    
    terminal.draw(|f| {
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());
        f.render_stateful_widget(list, chunks[0], &mut tui::widgets::ListState::default());
    })?;
    Ok(())
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode().unwrap();
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let music_files = scan_music_files("~/Music");
    let selected = 0;

    draw_ui(&mut terminal, &music_files, selected)?;
    
    disable_raw_mode().unwrap();
    Ok(())
}
