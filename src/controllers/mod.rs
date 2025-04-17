use clap::{Parser, ValueEnum};
use crate::models::FileData;
use crate::views;

/// Formats de sortie possibles
#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    /// Texte brut : `<nom> : '<contenu>'`
    Plain,
    /// JSON { "name": "...", "content": "..." }[]
    Json,
}

/// Arguments CLI
#[derive(Parser, Debug)]
#[command(
    name = "file_teacher",
    version,
    about = "Charge et affiche le contenu de fichiers donnés en argument",
    long_about = None
)]
struct Args {
    /// Chemins vers les fichiers à "enseigner"
    #[arg(value_name = "FILE", required = true)]
    files: Vec<String>,

    /// Format de sortie
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Plain)]
    format: OutputFormat,
}

/// Point d’entrée du contrôleur principal
pub fn run() {
    let args = Args::parse();

    // Charge tous les fichiers
    let mut files = Vec::with_capacity(args.files.len());
    for path in &args.files {
        match FileData::from_path(path) {
            Ok(fd) => files.push(fd),
            Err(e) => eprintln!("Erreur en lisant '{}': {}", path, e),
        }
    }

    // Appel de la vue selon le format
    match args.format {
        OutputFormat::Plain => views::display_plain(&files),
        OutputFormat::Json  => views::display_json(&files),
    }
}
