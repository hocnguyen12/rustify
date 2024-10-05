# rustify

Développer une application TUI en Rust qui permet de naviguer dans un dossier contenant des fichiers musicaux, de les lire, de gérer des playlists, et de mémoriser le dossier source des musiques peut se faire en plusieurs étapes. Je vais te guider pour construire ce projet petit à petit.
Plan de Développement

    Lecture des fichiers musicaux depuis un dossier et ses sous-dossiers.
    Lecture des fichiers audio.
    Interface TUI pour parcourir les fichiers et créer des playlists.
    Gestion des playlists avec la possibilité d’ajouter/supprimer des morceaux.
    Mémorisation du dossier source (par exemple, via un fichier de configuration).

1. Initialisation du Projet

Si tu as déjà un projet Cargo vide, tu devrais avoir un fichier Cargo.toml et un dossier src/ avec un fichier main.rs. Ajoutons les dépendances dont tu auras besoin dans Cargo.toml :
Ajouter des dépendances

toml

[dependencies]
tui = "0.19"               # Pour créer l'interface TUI
crossterm = "0.26"          # Pour gérer les entrées clavier et le terminal
rodio = "0.16"              # Pour lire des fichiers audio
walkdir = "2.3"             # Pour scanner les dossiers et sous-dossiers
serde = { version = "1.0", features = ["derive"] }  # Pour la sérialisation des playlists et des données du dossier
serde_json = "1.0"          # Pour sauvegarder et lire des fichiers JSON (mémoriser le dossier et les playlists)

2. Explorer le dossier source

Utilisons walkdir pour scanner tous les fichiers musicaux (formats comme .mp3, .wav, .ogg, etc.) dans un dossier et ses sous-dossiers.
Fonction pour scanner les fichiers musicaux

rust

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

Cette fonction parcourt récursivement le dossier et ajoute tous les fichiers de musique trouvés dans un vecteur.
3. Lire les fichiers audio

Pour lire les fichiers audio, nous allons utiliser la bibliothèque rodio. Voici une fonction simple qui joue une musique :
Lecture d’un fichier audio

rust

use rodio::{Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

fn play_music(file_path: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = File::open(file_path).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();
    
    stream_handle.play_raw(source.convert_samples()).unwrap();
}

Cette fonction ouvre le fichier audio et le lit en utilisant rodio.
4. Interface TUI pour gérer les musiques

Pour créer l’interface utilisateur dans le terminal, nous allons utiliser tui-rs avec crossterm pour la gestion des entrées clavier. Voici une première ébauche qui affiche la liste des fichiers musicaux trouvés :
Interface TUI de base

rust

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

    let music_files = scan_music_files("/path/to/music");
    let selected = 0;

    draw_ui(&mut terminal, &music_files, selected)?;
    
    disable_raw_mode().unwrap();
    Ok(())
}

Ce code affiche la liste des fichiers musicaux avec une interface utilisateur simple dans le terminal. Tu peux ajouter la gestion des entrées clavier avec crossterm pour permettre la navigation dans la liste.
5. Gestion des Playlists

Pour créer des playlists, tu peux stocker les fichiers sélectionnés dans un vecteur et les sauvegarder dans un fichier JSON avec serde pour te souvenir des playlists créées.
Créer et sauvegarder une playlist

rust

use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct Playlist {
    name: String,
    files: Vec<String>,
}

fn save_playlist(playlist: &Playlist) {
    let json = serde_json::to_string(playlist).unwrap();
    let mut file = File::create(format!("{}.json", playlist.name)).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

Avec cette fonction, tu peux sauvegarder une playlist dans un fichier JSON. De la même manière, tu peux charger une playlist depuis un fichier JSON.
6. Mémoriser le dossier source

Pour te souvenir du dossier qui contient toutes les musiques, tu peux également le sauvegarder dans un fichier de configuration JSON. Voici un exemple simple de gestion de ce dossier :
Sauvegarder et charger le dossier source

rust

use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    music_directory: String,
}

fn save_config(config: &Config) {
    let json = serde_json::to_string(config).unwrap();
    fs::write("config.json", json).unwrap();
}

fn load_config() -> Config {
    let data = fs::read_to_string("config.json").unwrap();
    serde_json::from_str(&data).unwrap()
}

Tu peux utiliser ces fonctions pour charger le dossier source lors du démarrage de l'application et sauvegarder toute modification.
7. Prochaines étapes

    Navigation clavier : Ajouter la gestion des flèches pour naviguer dans la liste des fichiers musicaux.
    Contrôle de la lecture : Ajouter des commandes pour lecture, pause, et stop.
    Gestion des playlists : Permettre de créer et de gérer plusieurs playlists.
    Configuration persistante : Mémoriser le dossier source et les playlists de manière persistante.

Avec ces fondations, tu devrais être en mesure de construire une application TUI capable de lire des musiques locales, de gérer des playlists, et de se souvenir du dossier source.