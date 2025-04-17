use crate::models::FileData;
use crate::views;

/// Contrôleur principal : reçoit les chemins, charge les modèles et appelle la vue
pub fn run() {
    // Exemple statique ; on remplacera plus tard par une lecture d’arguments CLI
    let paths = vec![
        r"C:\Users\elian\OneDrive\Documents\Liste\CVT.py",
        r"C:\Users\elian\Downloads\TP8_GABORIT_LEO\tp8_js\views\pages\suggest\index.ejs",
    ];

    let mut files = Vec::new();
    for p in paths {
        match FileData::from_path(p) {
            Ok(fd) => files.push(fd),
            Err(e) => eprintln!("Erreur en lisant '{}': {}", p, e),
        }
    }

    views::display(files);
}
