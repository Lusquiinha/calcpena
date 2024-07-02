use std::fmt::format;
use eframe::egui;
use eframe::egui::{Response, Ui, Widget};
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

            ui.label(self.label.clone());
            let mut text = format!("{}", self.valor);
            if ui.add_sized([50.0, 30.0], egui::TextEdit::singleline(&mut text)).changed(){
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
                if ui.button("1").clicked() {
                    self.concatenar(1);
                    *calc = self.valor;
                    *update = true;
                }
                if ui.button("2").clicked() {
                    self.concatenar(2);
                    *calc = self.valor;
                    *update = true;
                }
                if ui.button("3").clicked() {
                    self.concatenar(3);
                    *calc = self.valor;
                    *update = true;
                }
            });
            ui.horizontal(|ui| {
                if ui.button("4").clicked() {
                    self.concatenar(4);
                    *calc = self.valor;
                    *update = true;
                }
                if ui.button("5").clicked() {
                    self.concatenar(5);
                    *calc = self.valor;
                    *update = true;
                }
                if ui.button("6").clicked() {
                    self.concatenar(6);
                    *calc = self.valor;
                    *update = true;
                }
            });
            ui.horizontal(|ui| {
                if ui.button("7").clicked() {
                    self.concatenar(7);
                    *calc = self.valor;
                    *update = true;
                }
                if ui.button("8").clicked() {
                    self.concatenar(8);
                    *calc = self.valor;
                    *update = true;
                }
                if ui.button("9").clicked() {
                    self.concatenar(9);
                    *calc = self.valor;
                    *update = true;
                }
            });
            ui.horizontal(|ui| {
                if ui.button("0").clicked() {
                    self.concatenar(0);
                    *calc = self.valor;
                    *update = true;
                }
                if ui.button("C").clicked() {
                    self.set_valor(0);
                    *calc = self.valor;
                    *update = true;
                }
            });
        });
    }
}

