use eframe::{egui, App, Frame};
use rfd::FileDialog;
use std::{fs, path::{PathBuf, Path}, time::{Duration, Instant, SystemTime}};
use crate::models::FileData;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileEntry {
    pub path: PathBuf,
    pub content: String,
    pub include: bool,
    pub last_modified: SystemTime,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectoryEntry {
    pub path: PathBuf,
    pub expanded: bool,
    pub include: bool,
    pub children: Vec<Node>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind")]
pub enum Node {
    File(FileEntry),
    Directory(DirectoryEntry),
}

#[derive(PartialEq)]
enum ViewMode { Content, All }

pub struct FileTeacherApp {
    nodes: Vec<Node>,
    selected: Option<PathBuf>,
    view: ViewMode,
    all_text: String,
    last_check: Instant,
    check_interval: Duration,
}

impl FileTeacherApp {
    fn state_file() -> &'static str { "file_teacher_state.json" }
    fn load_state() -> Vec<Node> {
        if let Ok(text) = fs::read_to_string(Self::state_file()) {
            if let Ok(nodes) = serde_json::from_str::<Vec<Node>>(&text) {
                return nodes;
            }
        }
        Vec::new()
    }
    fn save_state(&self) {
        if let Ok(text) = serde_json::to_string_pretty(&self.nodes) {
            let _ = fs::write(Self::state_file(), text);
        }
    }
    fn set_include(nodes: &mut [Node], value: bool) {
        for node in nodes.iter_mut() {
            match node {
                Node::File(f) => f.include = value,
                Node::Directory(d) => {
                    d.include = value;
                    Self::set_include(&mut d.children, value);
                }
            }
        }
    }
    fn add_node(collection: &mut Vec<Node>, path: PathBuf) {
        if path.is_dir() {
            let mut dir = DirectoryEntry { path: path.clone(), expanded: false, include: true, children: Vec::new() };
            if let Ok(entries) = fs::read_dir(&path) {
                for entry in entries.filter_map(Result::ok) {
                    Self::add_node(&mut dir.children, entry.path());
                }
            }
            collection.push(Node::Directory(dir));
        } else if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if ext.eq_ignore_ascii_case("exe") { return; }
            }
            if let Ok(fd) = FileData::from_path(path.to_str().unwrap()) {
                let md = fs::metadata(&path).and_then(|m| m.modified()).unwrap_or(SystemTime::now());
                collection.push(Node::File(FileEntry { path, content: fd.content, include: true, last_modified: md }));
            }
        }
    }
    fn prune_path(nodes: &mut Vec<Node>, target: &Path) {
        nodes.retain(|node| match node {
            Node::File(f) => &f.path != target,
            Node::Directory(d) => &d.path != target,
        });
        for node in nodes.iter_mut() {
            if let Node::Directory(d) = node {
                Self::prune_path(&mut d.children, target);
            }
        }
    }
    fn refresh_modified(&mut self) {
        fn refresh(nodes: &mut [Node]) {
            for node in nodes.iter_mut() {
                match node {
                    Node::File(f) => {
                        if let Ok(md) = fs::metadata(&f.path).and_then(|m| m.modified()) {
                            if md > f.last_modified {
                                if let Ok(fd) = FileData::from_path(f.path.to_str().unwrap()) {
                                    f.content = fd.content;
                                    f.last_modified = md;
                                }
                            }
                        }
                    }
                    Node::Directory(d) => refresh(&mut d.children),
                }
            }
        }
        if self.last_check.elapsed() >= self.check_interval {
            refresh(&mut self.nodes);
            self.last_check = Instant::now();
        }
    }
    fn generate_all_text(&mut self) {
        fn collect(nodes: &[Node], out: &mut String) {
            for node in nodes {
                match node {
                    Node::File(f) if f.include => out.push_str(&format!("{}: '{}'\n", f.path.display(), f.content)),
                    Node::Directory(d) if d.include => collect(&d.children, out),
                    _ => {}
                }
            }
        }
        self.all_text.clear();
        collect(&self.nodes, &mut self.all_text);
    }
}
impl Default for FileTeacherApp {
    fn default() -> Self {
        FileTeacherApp { nodes: Self::load_state(), selected: None, view: ViewMode::Content, all_text: String::new(), last_check: Instant::now(), check_interval: Duration::from_secs(1) }
    }
}
impl App for FileTeacherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        self.refresh_modified();
        let mut to_add = Vec::new();
        let mut to_remove = Vec::new();
        let mut changed = false;
        egui::SidePanel::left("side").default_width(200.0).show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("📂 Fichier").clicked() { if let Some(path) = FileDialog::new().pick_file() { to_add.push(path); }}
                if ui.button("📁 Dossier").clicked() { if let Some(path) = FileDialog::new().pick_folder() { to_add.push(path); }}
            }); ui.separator();
            fn draw_tree(ui: &mut egui::Ui, nodes: &mut Vec<Node>, selected: &mut Option<PathBuf>, to_remove: &mut Vec<PathBuf>, changed: &mut bool) {
                for node in nodes.iter_mut() {
                    match node {
                        Node::File(f) => {
                            ui.horizontal(|ui| {
                                if ui.checkbox(&mut f.include, "").changed() { *changed = true; }
                                if ui.selectable_label(selected.as_ref() == Some(&f.path), f.path.file_name().unwrap().to_string_lossy().as_ref()).clicked() { *selected = Some(f.path.clone()); }
                                if ui.small_button("🗑").clicked() { to_remove.push(f.path.clone()); }
                            });
                        }
                        Node::Directory(d) => {
                            ui.horizontal(|ui| {
                                let symbol = if d.expanded { "▼" } else { "▶" };
                                if ui.small_button(symbol).clicked() { d.expanded = !d.expanded; }
                                if ui.checkbox(&mut d.include, d.path.file_name().unwrap().to_string_lossy().as_ref()).changed() {
                                    *changed = true; FileTeacherApp::set_include(&mut d.children, d.include);
                                }
                                if ui.small_button("🗑").clicked() { to_remove.push(d.path.clone()); }
                            });
                            if d.expanded {
                                ui.indent("", |ui| draw_tree(ui, &mut d.children, selected, to_remove, changed));
                            }
                        }
                    }
                }
            }
            draw_tree(ui, &mut self.nodes, &mut self.selected, &mut to_remove, &mut changed);
        });
        if !to_add.is_empty() { for p in to_add { FileTeacherApp::add_node(&mut self.nodes, p); changed = true; }}
        if !to_remove.is_empty() { for path in to_remove { FileTeacherApp::prune_path(&mut self.nodes, &path); changed = true; }}
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.selectable_label(self.view == ViewMode::Content, "Contenu").clicked() { self.view = ViewMode::Content; }
                if ui.selectable_label(self.view == ViewMode::All, "Tous les fichiers").clicked() { self.generate_all_text(); self.view = ViewMode::All; }
                if ui.button("Copier tout").clicked() { ctx.output_mut(|o| o.copied_text = self.all_text.clone()); }
            }); ui.separator(); let size = ui.available_size();
            match self.view {
                ViewMode::Content => {
                    if let Some(path) = &self.selected {
                        fn find<'a>(nodes: &'a [Node], target: &Path) -> Option<&'a FileEntry> {
                            for node in nodes {
                                match node {
                                    Node::File(f) if f.path == target => return Some(f),
                                    Node::Directory(d) => if let Some(f) = find(&d.children, target) { return Some(f); },
                                    _ => {}
                                }
                            }
                            None
                        }
                        if let Some(f) = find(&self.nodes, path) {
                            egui::ScrollArea::vertical().show(ui, |ui| { ui.add_sized(size, egui::TextEdit::multiline(&mut f.content.clone())); });
                        }
                    }
                }
                ViewMode::All => {
                    egui::ScrollArea::vertical().show(ui, |ui| { ui.add_sized(size, egui::TextEdit::multiline(&mut self.all_text)); });
                }
            }
        });
        if changed { self.save_state(); }
    }
}