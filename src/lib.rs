pub mod icons;

pub fn add_to_fonts(fonts: &mut egui::FontDefinitions) {
    fonts.font_data.insert("remixicon".into(), font_data());

    if let Some(font_keys) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
        font_keys.push("remixicon".into());
    }
}

fn font_data() -> egui::FontData {
    let font_data = egui::FontData::from_static(include_bytes!("../res/remixicon.ttf"));
    font_data
}
