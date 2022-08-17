use eframe::{
    egui::{self, CentralPanel, Color32, CtxRef, FontDefinitions, FontFamily, Label, Separator},
    epi::App,
    run_native, NativeOptions,
};
use std::borrow::Cow;
pub struct Headlinex {
    pub video_start: String,
    pub video_end: String,
}
impl App for Headlinex {
    fn setup(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.configure_fonts(ctx)
    }
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
      
    }
    fn name(&self) -> &str {
        "视频转换"
    }
}

fn main() {
    let app = Headlinex::new();
    let win_option = NativeOptions::default();
    run_native(Box::new(app), win_option);
  
}
