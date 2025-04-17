mod models;
mod controllers; // on garde pour CLI si besoin
mod views;       // idem
mod gui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "File Teacher GUI",
        native_options,
        Box::new(|_cc| Box::new(gui::FileTeacherApp::default())),
    );
}
