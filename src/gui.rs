use eframe::{egui, App, Frame};
use rfd::FileDialog;
use crate::models::FileData;

pub struct FileTeacherApp {
    files: Vec<FileData>,
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
        egui::SidePanel::left("left_panel")
            .default_width(250.0)
            .min_width(150.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Fichiers");
                if ui.button("📂 Charger un fichier").clicked() {
                    if let Some(path) = FileDialog::new().pick_file() {
                        if let Some(p) = path.to_str() {
                            if let Ok(fd) = FileData::from_path(p) {
                                self.files.push(fd);
                                self.selected = Some(self.files.len() - 1);
                                self.show_all = false;
                            }
                        }
                    }
                }
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (idx, f) in self.files.iter().enumerate() {
                        let is_sel = Some(idx) == self.selected;
                        if ui.selectable_label(is_sel, &f.name).clicked() {
                            self.selected = Some(idx);
                            self.show_all = false;
                        }
                    }
                });
                ui.separator();
                if !self.files.is_empty() {
                    if ui.button("Tous les fichiers").clicked() {
                        self.all_text = self.files.iter()
                            .map(|f| format!("{}: '{}'\n", f.name, f.content))
                            .collect();
                        self.show_all = true;
                    }
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.show_all {
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
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        let size = ui.available_size();
                        ui.add_sized(size, egui::TextEdit::multiline(&mut self.all_text));
                    });
            } else {
                ui.heading("Contenu");
                ui.separator();
                if let Some(i) = self.selected {
                    egui::ScrollArea::vertical()
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            let size = ui.available_size();
                            ui.add_sized(size, egui::TextEdit::multiline(&mut self.files[i].content));
                        });
                } else {
                    ui.label("Aucun fichier sélectionné");
                }
            }
        });
    }
}
