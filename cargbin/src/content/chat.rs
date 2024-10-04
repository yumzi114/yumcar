use std::sync::{Arc, Mutex};

use eframe::egui::{self, Color32, InnerResponse, RichText, Ui};




pub fn chat_view(
    ui: &mut Ui,
    ctx: &egui::Context,
    msg_mem:Arc<Mutex<Vec<String>>>,
)->InnerResponse<()>{
    
    ui.vertical(|ui|{
        ui.label("text");
    })
}