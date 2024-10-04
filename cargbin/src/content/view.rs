use eframe::egui::{self, InnerResponse, Ui};




pub fn view_view(
    ui: &mut Ui,
    ctx: &egui::Context,
    
)->InnerResponse<()>{
    ui.horizontal(|ui|{
        ui.heading("view_view");
    })
}