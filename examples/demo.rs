use eframe::egui;


fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "T-Rex Demo",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(TrexApp::default()))),
    )
}

#[derive(Default)]
struct TrexApp {}

impl eframe::App for TrexApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            //Nothing added yet.
        });
    }
}