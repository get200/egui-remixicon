pub mod icons;

#[cfg(feature = "egui")]
pub fn add_to_fonts(fonts: &mut egui::FontDefinitions) {
    fonts.font_data.insert("remixicon".into(), font_data());

    if let Some(font_keys) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
        font_keys.push("remixicon".into());
    }
}

#[cfg(feature = "egui")]
fn font_data() -> egui::FontData {
    let font_data = egui::FontData::from_static(FONT);
    font_data
}

pub const FONT: &[u8] = include_bytes!("../assets/MaterialIcons-Regular.ttf");
