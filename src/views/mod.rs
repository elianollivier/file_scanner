use crate::models::FileData;

/// Affichage "View" dans la console
pub fn display(files: Vec<FileData>) {
    for f in files {
        println!("{} : '{}'\n", f.name, f.content);
    }
}
