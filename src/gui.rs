use eframe::{egui, App, Frame};
use rfd::FileDialog;
use crate::models::FileData;

struct FileEntry {
    data: FileData,
    include: bool,
}

pub struct FileTeacherApp {
    files: Vec<FileEntry>,
    selected: Option<usize>,
    all_text: String,
    show_all: bool,
}

impl Default for FileTeacherApp {
    fn default() -> Self {
        Self { files: Vec::new(), selected: None, all_text: String::new(), show_all: false }
    }
}

impl App for FileTeacherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        let mut to_remove: Option<usize> = None;
        egui::SidePanel::left("left_panel").default_width(250.0).min_width(150.0).resizable(true).show(ctx, |ui| {
            ui.heading("Fichiers");
            if ui.button("📂 Charger un fichier").clicked() {
                if let Some(path) = FileDialog::new().pick_file() {
                    if let Some(p) = path.to_str() {
                        if let Ok(fd) = FileData::from_path(p) {
                            self.files.push(FileEntry { data: fd, include: true });
                            self.selected = Some(self.files.len() - 1);
                            self.show_all = false;
                        }
                    }
                }
            }
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (idx, entry) in self.files.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut entry.include, "");
                        let selected = Some(idx) == self.selected;
                        if ui.selectable_label(selected, &entry.data.name).clicked() {
                            self.selected = Some(idx);
                            self.show_all = false;
                        }
                        if ui.small_button("🗑").clicked() {
                            to_remove = Some(idx);
                        }
                    });
                }
            });
            ui.separator();
            if !self.files.is_empty() {
                if ui.button("Tous les fichiers").clicked() {
                    self.show_all = true;
                }
            }
        });

        if let Some(idx) = to_remove {
            self.files.remove(idx);
            if let Some(sel) = self.selected {
                self.selected = if sel == idx { None } else if sel > idx { Some(sel - 1) } else { Some(sel) };
            }
            self.show_all = false;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let size = ui.available_size();
            if self.show_all {
                let s: String = self.files.iter().filter(|e| e.include).map(|e| format!("{}: '{}'\n", e.data.name, e.data.content)).collect();
                self.all_text = s;
                ui.horizontal(|ui| {
                    if ui.button("Copier tout").clicked() {
                        let text = self.all_text.clone();
                        ctx.output_mut(|out| out.copied_text = text);
                    }
                    if ui.button("Retour").clicked() {
                        self.show_all = false;
                    }
                });
                ui.separator();
                egui::ScrollArea::vertical().max_width(size.x).max_height(size.y).show(ui, |ui| {
                    ui.add_sized(size, egui::TextEdit::multiline(&mut self.all_text));
                });
            } else {
                ui.heading("Contenu");
                ui.separator();
                if let Some(i) = self.selected {
                    let entry = &mut self.files[i];
                    egui::ScrollArea::vertical().max_width(size.x).max_height(size.y).show(ui, |ui| {
                        ui.add_sized(size, egui::TextEdit::multiline(&mut entry.data.content));
                    });
                    ui.separator();
                    if ui.button("Générer tous").clicked() {
                        self.show_all = true;
                    }
                } else {
                    ui.label("Aucun fichier sélectionné");
                }
            }
        });
    }
}