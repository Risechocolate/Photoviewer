use egui::{CentralPanel, epaint::tessellator::path};
use rfd::FileDialog;
use url::Url;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
    )
    .unwrap();
}

#[derive(Default)]
struct MyApp {
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                if ui.button("Open file...").clicked() {
                    if let Some(path) = FileDialog::new()
                        .add_filter("Picture", &["png", "jpg", "svg", "webp"])
                        .pick_file()
                    {
                        self.picked_path = Url::from_file_path(path).ok().map(|p| p.into());
                    }
                }

                if let Some(path) = &self.picked_path {
                    ui.label(path);
                }
            });

            if !self.dropped_files.is_empty() {
                ui.group(|ui| {
                    ui.label("Dropped files:");
                });
            }

            if let Some(path) = &self.picked_path {
                ui.horizontal_centered(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.image(path);
                    });
                });
            }
        });
    }
}

// ©2025 Rise_chocolate
