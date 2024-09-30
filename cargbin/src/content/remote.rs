use eframe::egui::{self, InnerResponse, Ui};




pub fn remote_view(ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
    ui.horizontal(|ui|{
        ui.heading("remote_view");
    })
}