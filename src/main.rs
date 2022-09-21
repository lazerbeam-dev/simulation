use eframe::{epi, NativeOptions};
use eframe::egui::{CentralPanel};

struct Sim{
    name: String
}

impl Sim {
    pub fn new() -> Sim{
        Sim{name: "name name".to_string()}
    }
}
impl eframe::epi::App for Sim{
    fn setup(
        &mut self,
        _ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        //self.configure_fonts(ctx);
    }
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        //self.render_top_panel(ctx);
        CentralPanel::default().show(ctx, |ui| {
        });
    }

    fn name(&self) -> &str {
        &self.name
    }
}


fn main() {
    let app = Sim::new();
    let mut win_option = NativeOptions::default();
    eframe::run_native(Box::new(app), win_option);
}