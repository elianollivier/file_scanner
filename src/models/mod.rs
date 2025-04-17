/// Chargement et représentation des données (ici : fichiers à "enseigner")
pub struct FileData {
    pub name: String,
    pub content: String,
}

impl FileData {
    pub fn from_path(path: &str) -> std::io::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let name = std::path::Path::new(path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        Ok(FileData { name, content })
    }
}
