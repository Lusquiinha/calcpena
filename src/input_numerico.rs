use std::fmt::format;
use eframe::egui;
use eframe::egui::{Response, Rounding, Ui, Widget};
use eframe::egui::TextStyle::Heading;
use crate::calculadora::Calculadora;

pub struct InputNumerico {
    valor: u32,
    max_val: u32,
    label: String,
}

impl InputNumerico {
    pub fn new(label: String ,max: u32) -> InputNumerico {
        InputNumerico { valor: 0, max_val: max, label}
    }

    pub fn set_valor(&mut self, valor: u32) {
        self.valor = valor;
    }

    pub fn get_valor(&self) -> u32 {
        self.valor
    }

    pub fn concatenar(&mut self, valor: u32) {
        self.valor = self.valor * 10 + valor;
        if self.valor > self.max_val {
            self.valor = self.max_val;
        }
    }

    pub(crate) fn show(&mut self, ui: &mut egui::Ui, calc: &mut u32, update: &mut bool) {
        ui.vertical(|ui| {


            let mut text = format!("{}", self.valor);
            if Self::bordered_text_edit(ui, &mut text, self.label.clone()).changed(){
                if text.is_empty() {
                    self.valor = 0;
                }
                if let Ok(result) = text.parse() {
                    self.valor = result;
                    if self.valor > self.max_val {
                        self.valor = self.max_val;
                    }
                    *calc = self.valor;
                    *update = true;
                }
            };

            ui.horizontal(|ui| {
                ui.style_mut().spacing.item_spacing = [3.0, 0.0].into();

                if self.add_button(ui, "1").clicked() {
                    self.concatenar(1);
                    *calc = self.valor;
                    *update = true;
                }
                if self.add_button(ui, "2").clicked() {
                    self.concatenar(2);
                    *calc = self.valor;
                    *update = true;
                }
                if self.add_button(ui, "3").clicked() {
                    self.concatenar(3);
                    *calc = self.valor;
                    *update = true;
                }
            });
            ui.horizontal(|ui| {
                ui.style_mut().spacing.item_spacing = [3.0, 0.0].into();
                if self.add_button(ui, "4").clicked() {
                    self.concatenar(4);
                    *calc = self.valor;
                    *update = true;
                }
                if self.add_button(ui, "5").clicked() {
                    self.concatenar(5);
                    *calc = self.valor;
                    *update = true;
                }
                if self.add_button(ui, "6").clicked() {
                    self.concatenar(6);
                    *calc = self.valor;
                    *update = true;
                }
            });
            ui.horizontal(|ui| {
                ui.style_mut().spacing.item_spacing = [3.0, 0.0].into();
                if self.add_button(ui, "7").clicked() {
                    self.concatenar(7);
                    *calc = self.valor;
                    *update = true;
                }
                if self.add_button(ui, "8").clicked() {
                    self.concatenar(8);
                    *calc = self.valor;
                    *update = true;
                }
                if self.add_button(ui, "9").clicked() {
                    self.concatenar(9);
                    *calc = self.valor;
                    *update = true;
                }
            });
            ui.horizontal(|ui| {
                ui.style_mut().spacing.item_spacing = [3.0, 0.0].into();
                if self.add_button(ui, "0").clicked() {
                    self.concatenar(0);
                    *calc = self.valor;
                    *update = true;
                }
                if self.add_button(ui, "C").clicked() {
                    self.set_valor(0);
                    *calc = self.valor;
                    *update = true;
                }
            });
        });
    }
    pub fn add_button(&self, ui: &mut Ui, label: &str) -> Response {
        ui.add(egui::Button::new(egui::RichText::new(label).size(16.0)).stroke(egui::Stroke::new(0.5, egui::Color32::BLACK)).rounding(0.0))
    }
    pub fn bordered_text_edit(ui: &mut egui::Ui, text: &mut String, label: String) -> egui::Response {
        let frame = egui::Frame::dark_canvas(ui.style()).rounding(Rounding::from(0.0));
        frame.show(ui, |ui| {
            ui.label(egui::RichText::new(label).size(10.0).color(egui::Color32::GREEN));
            ui.style_mut().visuals.extreme_bg_color = egui::Color32::BLACK;
            ui.style_mut().visuals.widgets.hovered.bg_stroke.color = egui::Color32::BLACK;
            ui.add_sized([53.0, 22.0], egui::TextEdit::singleline(text).font(Heading).text_color(egui::Color32::GREEN));
        }).response
    }
}

