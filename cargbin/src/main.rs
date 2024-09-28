// use eframe::{egui::{self, Pos2, Rounding, Sense, Vec2, ViewportBuilder}};
use catppuccin_egui::{FRAPPE, LATTE, MACCHIATO, MOCHA};
use eframe::egui::{self, ViewportBuilder};
fn main() {
    let windows = ViewportBuilder{
        title: Some(String::from("yum car app")),
        app_id: Some(String::from("yum car app")),
        fullsize_content_view: Some(true),
        titlebar_shown: Some(false),
        resizable: Some(false),
        fullscreen:Some(true),
        ..Default::default()
    };
    let options = eframe::NativeOptions {
        viewport:windows,
        // default_theme:Theme::Dark,
        ..Default::default()
    };
    // let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA);
        ctx.set_pixels_per_point(1.25);
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("START CAR APP");
       });
   }
}