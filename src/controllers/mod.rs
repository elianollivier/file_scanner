use clap::{Parser, ValueEnum};
use crate::models::FileData;
use crate::views;

/// Formats de sortie possibles
#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    Plain,
    Json,
}

/// Arguments CLI
#[derive(Parser, Debug)]
#[command(
    name = "file_teacher",
    version,
    about = "Charge et affiche le contenu de fichiers donnés en argument",
)]
struct Args {
    /// Chemins vers les fichiers à "enseigner"
    #[arg(value_name = "FILE", required = true)]
    files: Vec<String>,

    /// Format de sortie (plain ou json)
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Plain)]
    format: OutputFormat,

    /// Extensions à inclure (sans le point). Peut être répété.
    #[arg(short = 'e', long = "extension", value_name = "EXT")]
    extensions: Vec<String>,

    /// N'affiche que les noms de fichiers (ignore le contenu)
    #[arg(short = 'n', long = "names-only")]
    names_only: bool,
}

pub fn run() {
    let args = Args::parse();

    // Charge et filtre par extension si demandé
    let mut files = Vec::new();
    for path in &args.files {
        // si on a un filtre et que l'extension ne matche pas, on skip
        if !args.extensions.is_empty() {
            if let Some(ext) = std::path::Path::new(path)
                .extension()
                .and_then(|s| s.to_str())
            {
                if !args.extensions.iter().any(|e| e.eq_ignore_ascii_case(ext)) {
                    continue;
                }
            } else {
                continue;
            }
        }

        match FileData::from_path(path) {
            Ok(fd) => files.push(fd),
            Err(e) => eprintln!("Erreur en lisant '{}': {}", path, e),
        }
    }

    // Dispatch selon names_only et format
    match (args.names_only, args.format) {
        (true, OutputFormat::Plain) => views::display_names(&files),
        (true, OutputFormat::Json)  => views::display_names_json(&files),
        (false, OutputFormat::Plain)=> views::display_plain(&files),
        (false, OutputFormat::Json) => views::display_json(&files),
    }
}
