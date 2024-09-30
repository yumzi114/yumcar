use eframe::egui::{self, InnerResponse, Ui};
use egui_plot::{Line, Plot, PlotPoints};



pub fn graph_view(ui: &mut Ui,ctx: &egui::Context)->InnerResponse<()>{
    ui.vertical_centered(|ui|{
        // ui.heading("graph_view");
        let sin: PlotPoints = (0..1000).map(|i| {
            let x = i as f64 * 0.01;
            [x, x.sin()]
        }).collect();
        let line = Line::new(sin);
        Plot::new("live")
        .width(310.)
        .height(400.)
        // .view_aspect(2.0)
        .show(ui, |plot_ui| plot_ui.line(line));
    })
}