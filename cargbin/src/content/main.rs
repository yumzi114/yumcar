use eframe::egui::{self, InnerResponse, Ui};




pub fn main_view(ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
    ui.horizontal(|ui|{
        ui.heading("main_view");
    })
}