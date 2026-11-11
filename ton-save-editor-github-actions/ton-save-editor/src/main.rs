mod storage;

use eframe::egui;
use std::path::PathBuf;

#[derive(Default)]
struct TonEditor {
    save_code: String,
    status: String,
    current_file: Option<PathBuf>,
    dirty: bool,
}

impl TonEditor {
    fn import_file(&mut self) {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("Save files", &["txt", "ton", "dat"])
            .pick_file()
        else { return };

        match std::fs::read_to_string(&path) {
            Ok(contents) => {
                if !self.save_code.trim().is_empty() {
                    match storage::create_backup(&self.save_code) {
                        Ok(backup) => self.status = format!("Previous editor contents backed up to {}", backup.display()),
                        Err(err) => self.status = format!("Could not create backup: {err}"),
                    }
                }
                self.save_code = contents;
                self.current_file = Some(path.clone());
                self.dirty = false;
                self.status = format!("Imported {}", path.display());
            }
            Err(err) => self.status = format!("Import failed: {err}"),
        }
    }

    fn export_file(&mut self) {
        let Some(path) = rfd::FileDialog::new()
            .set_file_name("ton-save.txt")
            .add_filter("Text files", &["txt"])
            .save_file()
        else { return };

        if path.exists() {
            if let Err(err) = storage::backup_file(&path) {
                self.status = format!("Export cancelled: could not back up existing file: {err}");
                return;
            }
        }

        match std::fs::write(&path, self.save_code.as_bytes()) {
            Ok(()) => {
                self.current_file = Some(path.clone());
                self.dirty = false;
                self.status = format!("Exported {}", path.display());
            }
            Err(err) => self.status = format!("Export failed: {err}"),
        }
    }
}

impl eframe::App for TonEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("ToN Save Editor");
                ui.label(egui::RichText::new("native Rust build • early development").weak());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Export save…").clicked() { self.export_file(); }
                    if ui.button("Import save…").clicked() { self.import_file(); }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Save code");
            ui.label("Paste a Terrors of Nowhere save code here, or import it from a text file.");
            let response = ui.add(
                egui::TextEdit::multiline(&mut self.save_code)
                    .desired_rows(8)
                    .desired_width(f32::INFINITY)
                    .hint_text("Paste save code…")
                    .font(egui::TextStyle::Monospace)
            );
            if response.changed() { self.dirty = true; }

            ui.horizontal(|ui| {
                if ui.button("Clear editor").clicked() {
                    if !self.save_code.trim().is_empty() {
                        match storage::create_backup(&self.save_code) {
                            Ok(path) => self.status = format!("Cleared editor; backup created at {}", path.display()),
                            Err(err) => self.status = format!("Clear cancelled: backup failed: {err}"),
                        }
                        if self.status.starts_with("Cleared editor") { self.save_code.clear(); self.dirty = false; }
                    }
                }
                if self.dirty { ui.label(egui::RichText::new("Unsaved changes").color(egui::Color32::YELLOW)); }
                if let Some(path) = &self.current_file { ui.label(path.display().to_string()); }
            });

            ui.separator();
            ui.heading("Editor modules");
            ui.label("The original editor exposes these data groups. Their controls will be enabled after the save-code format is implemented and verified.");
            egui::Grid::new("modules").num_columns(2).spacing([24.0, 8.0]).show(ui, |ui| {
                for (name, description) in [
                    ("Save stats", "Player name, item hue, play time, Enkephalin, deaths, SP and other counters"),
                    ("Terrors & alternates", "Collection progress and variants"),
                    ("Moons & specials", "Moon and special round collections"),
                    ("Unbound rounds", "Unbound collection progress"),
                    ("Beyond's Masks", "8 Pages, Cracked, Midnight and Variant"),
                    ("Locations", "Unlocked map locations"),
                    ("Achievements", "Achievement flags and progress"),
                    ("Collab codes", "The six collaboration-code fields"),
                ] {
                    ui.strong(name);
                    ui.label(description);
                    ui.end_row();
                }
            });

            ui.separator();
            ui.label(egui::RichText::new(
                "Safety note: this starter currently preserves save codes as raw text. It does not yet parse or rewrite the game's encoded data, so it cannot accidentally generate a malformed edited code."
            ).weak());
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.label(if self.status.is_empty() { "Ready".to_owned() } else { self.status.clone() });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("ToN Save Editor")
            .with_inner_size([1040.0, 760.0])
            .with_min_inner_size([760.0, 560.0]),
        ..Default::default()
    };

    eframe::run_native(
        "ToN Save Editor",
        options,
        Box::new(|_cc| Ok(Box::<TonEditor>::default())),
    )
}
