#![windows_subsystem = "windows"]

use eframe::egui;
use std::fs::File;
use std::io::BufReader;
use rfd::FileDialog;

mod crypt_tools;
use crypt_tools::sha1::hash_sha1;
use crypt_tools::sha256::hash_sha256;
use crypt_tools::sha512::hash_sha512;





#[derive(Default)]
struct MyApp {
    input_text: String,
    file_path: Option<String>,
    sha1_hash: String,
    sha256_hash: String,
    sha512_hash: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("SHA Hash Calculator");

            ui.horizontal(|ui| {
                ui.label("Input Text:");
                ui.text_edit_singleline(&mut self.input_text);
            });

            if ui.button("Text").clicked() {
                self.sha1_hash = hash_sha1(self.input_text.as_bytes());
                self.sha256_hash = hash_sha256(self.input_text.as_bytes());
                self.sha512_hash = hash_sha512(self.input_text.as_bytes());
            }

            ui.separator();

            if ui.button("Select File").clicked() {
                if let Some(path) = FileDialog::new().pick_file() {
                    self.file_path = Some(path.display().to_string());
                    if let Ok(file) = File::open(&path) {
                        self.sha1_hash = crypt_tools::sha1::hash_sha1(BufReader::new(file.try_clone().expect("Failed to clone file handle")));
                        self.sha256_hash = hash_sha256(BufReader::new(file.try_clone().expect("Failed to clone file handle")));
                        self.sha512_hash = hash_sha512(BufReader::new(file));
                    }
                }
            }

            if let Some(ref path) = self.file_path {
                ui.label(format!("Selected File: {}", path));
            }

            ui.separator();

            ui.colored_label(egui::Color32::GREEN, format!("SHA-1:   {}", self.sha1_hash));
            ui.colored_label(egui::Color32::YELLOW, format!("SHA-256: {}", self.sha256_hash));
            ui.colored_label(egui::Color32::BLUE, format!("SHA-512: {}", self.sha512_hash));
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "SHA Hash Calculator",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
