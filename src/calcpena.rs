use eframe::{App, egui, Frame};
use eframe::egui::Context;

pub struct Calcpena {

}

impl App for Calcpena {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.label("Calculadora");
        });
    }
}
impl Calcpena {
    pub(crate) fn default() -> Self {
        Self {}
    }
}