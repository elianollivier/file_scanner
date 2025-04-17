use crate::models::FileData;
use serde_json::json;

/// Affichage “plain” complet
pub fn display_plain(files: &[FileData]) {
    for f in files {
        println!("{} : '{}'\n", f.name, f.content);
    }
}

/// Affichage JSON complet
pub fn display_json(files: &[FileData]) {
    let arr = files.iter().map(|f| {
        json!({
            "name": f.name,
            "content": f.content,
        })
    }).collect::<Vec<_>>();
    println!("{}", serde_json::to_string_pretty(&arr).unwrap());
}

/// Affiche seulement les noms (texte brut)
pub fn display_names(files: &[FileData]) {
    for f in files {
        println!("{}", f.name);
    }
}

/// Affiche seulement les noms (JSON)
pub fn display_names_json(files: &[FileData]) {
    let names = files.iter().map(|f| json!(f.name)).collect::<Vec<_>>();
    println!("{}", serde_json::to_string_pretty(&names).unwrap());
}
