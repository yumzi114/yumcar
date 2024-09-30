// use eframe::{egui::{self, Pos2, Rounding, Sense, Vec2, ViewportBuilder}};
mod app_conf;
use app_conf::setup_custom_fonts;
use catppuccin_egui::{FRAPPE, LATTE, MACCHIATO, MOCHA};
use eframe::egui::{self, menu, Color32, RichText, ViewportBuilder};


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
        setup_custom_fonts(&cc.egui_ctx);
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    ctx.request_repaint();
    catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA);
    ctx.set_pixels_per_point(1.25);
       egui::CentralPanel::default().show(ctx, |ui| {
        menu::bar(ui, |ui| {
            // ui.label(format!("{}", egui_nerdfonts::regular::REMOTE));
            ui.menu_button(
                RichText::new(
                    format!("{} Remote", egui_nerdfonts::regular::REMOTE_1))
                    .strong()
                    .size(30.0)
                    .color(Color32::from_rgb(38, 150, 255)), |ui| {
                if ui.button("Open").clicked() {
                    
                    // …
                }
            });
            ui.menu_button(RichText::new(
                format!("{} View", egui_nerdfonts::regular::MONITOR))
                .strong()
                .size(30.0)
                .color(Color32::from_rgb(38, 150, 255)), |ui| {
                if ui.button("Open").clicked() {
                    
                    // …
                }
            });
            ui.menu_button(RichText::new(
                format!("{} Graph", egui_nerdfonts::regular::GRAPH_1))
                .strong()
                .size(30.0)
                .color(Color32::from_rgb(38, 150, 255)), |ui| {
                if ui.button("Open").clicked() {
                    // …
                }
            });
        });
           ui.add_space(25.);
           ui.heading("START CAR APP");
       });
   }
}