use eframe::NativeOptions;

fn main() {
    let native_options = NativeOptions {
        centered: true,
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 300.0]),
        ..Default::default()
    };
    eframe::run_native(
        "egui-remixicon demo",
        native_options,
        Box::new(|cc| Box::new(Demo::new(cc))),
    )
    .unwrap();
}

struct Demo {}

impl Demo {
    fn new(cc: &eframe::CreationContext) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        egui_remixicon::add_to_fonts(&mut fonts);

        cc.egui_ctx.set_fonts(fonts);

        Self {}
    }
}

impl eframe::App for Demo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let _ = ui.label(
                    egui::RichText::new(format!("{} Home", egui_remixicon::icons::HOME_2_FILL))
                        .size(20.0),
                );

                ui.add_space(5.0);

                let _ = ui.label(
                    egui::RichText::new(format!("{} Heart", egui_remixicon::icons::HEARTS_FILL))
                        .size(20.0),
                );

                ui.add_space(5.0);
                let _ = ui.label(
                    egui::RichText::new(format!(
                        "{} Settings",
                        egui_remixicon::icons::SETTINGS_3_FILL
                    ))
                    .size(20.0),
                );
            });
        });
    }
}
