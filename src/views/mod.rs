use crate::models::FileData;
use serde_json::json;

/// Affichage “plain”
pub fn display_plain(files: &[FileData]) {
    for f in files {
        println!("{} : '{}'\n", f.name, f.content);
    }
}

/// Affichage JSON
pub fn display_json(files: &[FileData]) {
    let arr = files.iter().map(|f| {
        json!({
            "name": f.name,
            "content": f.content,
        })
    }).collect::<Vec<_>>();
    // Sérialisation et impression
    println!("{}", serde_json::to_string_pretty(&arr).unwrap());
}
