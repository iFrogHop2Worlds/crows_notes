use egui::Frame;
use eframe::egui;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use pulldown_cmark::{html, Parser};

fn main() {
    let app = MarkdownEditorApp::default();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Crows Note Md",
        options,
        Box::new(|_cc| Ok(Box::new(MarkdownEditorApp::default()))), // closure creates the app
    );
}

struct MarkdownEditorApp {
    files: Vec<String>,
    selected_file: Option<String>,
    file_content: String,
    is_markdown_mode: bool,
}

impl Default for MarkdownEditorApp {
    fn default() -> Self {
        let notes_path = "src/notes";
        if !Path::new(notes_path).exists() {
            fs::create_dir(notes_path).unwrap();
        }

        let files = WalkDir::new(notes_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .map(|e| e.path().display().to_string())
            .collect();

        Self {
            files,
            selected_file: None,
            file_content: String::new(),
            is_markdown_mode: false,
        }
    }
}

impl eframe::App for MarkdownEditorApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::SidePanel::left("file_list").resizable(false).show(ctx, |ui| {
            for file in &self.files {
                let display_name = file.trim_start_matches("src/notes\\");
                if ui.button(display_name).clicked() {
                    self.selected_file = Some(file.clone());
                    self.file_content = fs::read_to_string(file).unwrap_or_default();
                }
            }
        });

        egui::CentralPanel::default()
            .frame(Frame::default().outer_margin(10.0))
            .show(ctx, |ui| {
                if let Some(selected_file) = &self.selected_file {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(format!(
                                "Editing: {}",
                                selected_file.trim_start_matches("src/notes\\")
                            ))
                                .size(16.0)
                                .strong(),
                        );
                        if ui
                            .button(if self.is_markdown_mode {
                                "Switch to Edit View"
                            } else {
                                "Switch to Markdown View"
                            })
                            .clicked()
                        {
                            self.is_markdown_mode = !self.is_markdown_mode;
                        }
                    });
                    let available_size = ui.available_size();
                    if self.is_markdown_mode {
                        let rendered_markdown = render_markdown(&self.file_content);
                        ui.add_sized(
                            available_size,
                            egui::Label::new(egui::RichText::new(rendered_markdown).text_style(egui::TextStyle::Body)),
                        );
                    } else {
                        // raw markdown content
                        ui.add_sized(
                            available_size,
                            egui::TextEdit::multiline(&mut self.file_content)
                                .desired_width(f32::INFINITY),
                        );
                    }
                } else {
                    ui.label("Select a file to edit");
                }
            });
    }
}

fn render_markdown(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}