use clap::Parser;
use crate::models::FileData;
use crate::views;

/// Définit les arguments CLI
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
}

/// Point d’entrée du contrôleur principal
pub fn run() {
    // Parse les arguments
    let args = Args::parse();

    let mut files = Vec::new();
    for path in &args.files {
        match FileData::from_path(path) {
            Ok(fd) => files.push(fd),
            Err(e) => eprintln!("Erreur en lisant '{}': {}", path, e),
        }
    }

    // Passe la liste à la vue
    views::display(files);
}
