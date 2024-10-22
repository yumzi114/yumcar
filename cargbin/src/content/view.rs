use crossbeam_channel::Receiver;
use eframe::egui::{self, Color32, InnerResponse, RichText, Ui};




pub fn ht_view(
    ui: &mut Ui,
    ctx: &egui::Context,
    msg_list:&mut Vec<String>,
    
)->InnerResponse<()>{
    // egui::ScrollArea::both().show(ui, |ui| {
        
    // })
    ui.vertical_centered(|ui|{
        ui.heading(RichText::new(
            "humidity/temperature VIEW"
        )
            .strong()
            .size(22.0)
            .color(Color32::from_rgb(235, 64, 52)));

        ui.add_space(10.0);
        if msg_list.len()>30{
            msg_list.clear();
        }
        for i in msg_list{
            ui.label(RichText::new(
                i.as_str()
            )
                .strong()
                .size(15.0)
                .color(Color32::from_rgb(235, 64, 52)));
        }
    })
}