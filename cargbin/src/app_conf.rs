use eframe::egui;

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!(
            // "../files/Pilseung_Gothic.ttf"
            "../../files/RusticShine.ttf"
        )),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());
    egui_nerdfonts::add_to_fonts(&mut fonts, egui_nerdfonts::Variant::Regular);
    ctx.set_fonts(fonts);
}