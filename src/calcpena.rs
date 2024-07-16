use eframe::{App, egui, Frame};
use eframe::egui::Context;
use crate::calculadora::{Calculadora, DeltaData};
use crate::input_numerico::InputNumerico;
use chrono::{NaiveDate};
use eframe::egui::Key::F10;
use egui_extras::DatePickerButton;


pub struct Calcpena {
    anos:  InputNumerico,
    meses: InputNumerico,
    dias:  InputNumerico,
    calc: Calculadora
}

impl App for Calcpena {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{

                ui.horizontal(|ui| {
                    ui.add_space(60.0);
                    self.anos.show(ui, &mut self.calc.dd.ano, &mut self.calc.updt);
                    ui.add_space(10.0);
                    self.meses.show(ui, &mut self.calc.dd.mes, &mut self.calc.updt);
                    ui.add_space(10.0);
                    self.dias.show(ui, &mut self.calc.dd.dia, &mut self.calc.updt);
                });

            ui.add_space(30.0);
            ui.draw_datepicker(&mut self.calc.data_inicial, &mut self.calc.updt);
            ui.add_space(30.0);
            ui.label(format!("Data Final: {}", self.calc.datas.inteiro_data.format("%d/%m/%Y").to_string()));
            ui.add_space(30.0);

            self.calc.update();
            ui.horizontal(|ui|{
                ui.vertical(|ui|{
                    ui.triple_label("1/8".to_string(), self.calc.datas.um_oitavo, self.calc.datas.um_oitavo_data);
                    ui.triple_label("16%".to_string(), self.calc.datas.dezesseis_pct, self.calc.datas.dezesseis_pct_data);
                    ui.triple_label("1/6".to_string(), self.calc.datas.um_sexto, self.calc.datas.um_sexto_data);
                    ui.triple_label("1/5".to_string(), self.calc.datas.um_quinto, self.calc.datas.um_quinto_data);
                    ui.triple_label("1/4".to_string(), self.calc.datas.um_quarto, self.calc.datas.um_quarto_data);
                    ui.triple_label("30%".to_string(), self.calc.datas.trinta_pct, self.calc.datas.trinta_pct_data);

                });
                ui.add_space(3.0);
                ui.vertical(|ui|{
                    ui.triple_label("1/3".to_string(), self.calc.datas.um_terco, self.calc.datas.um_terco_data);
                    ui.triple_label("40% | 2/5".to_string(), self.calc.datas.dois_quintos, self.calc.datas.dois_quintos_data);
                    ui.triple_label("1/2".to_string(), self.calc.datas.um_meio, self.calc.datas.um_meio_data);
                    ui.triple_label("60% | 3/5".to_string(), self.calc.datas.tres_quintos, self.calc.datas.tres_quintos_data);
                    ui.triple_label("2/3".to_string(), self.calc.datas.dois_tercos, self.calc.datas.dois_tercos_data);
                    ui.triple_label("70%".to_string(), self.calc.datas.setenta_pct, self.calc.datas.setenta_pct_data);

                });

            });
        });
    }
}
impl Calcpena {
    pub(crate) fn default() -> Self {
        Self {
            anos:  InputNumerico::new("ANOS: ".to_string(), 999),
            meses: InputNumerico::new("MESES: ".to_string(), 12),
            dias:  InputNumerico::new("DIAS: ".to_string(), 30),
            calc: Calculadora::new()
        }
    }

}

//trait that defines the behavior of the ui elements
trait CustomUi {
    fn triple_label(&mut self, esq: String, dir_up: DeltaData, dir_down: NaiveDate);
    fn draw_datepicker(&mut self, date: &mut NaiveDate, update: &mut bool);
}

impl CustomUi for egui::Ui {
    fn triple_label(&mut self, esq: String, dir_up: DeltaData, dir_down: NaiveDate) {
        self.horizontal(|ui| {
            egui::Frame::canvas(ui.style()).stroke(egui::Stroke::new(0.5, egui::Color32::BLACK)).rounding(0.0).show(ui, |ui| {
                ui.add_sized([60.0,46.0],egui::Label::new(esq));
            });
            ui.vertical(|ui| {

                egui::Frame::canvas(ui.style()).stroke(egui::Stroke::new(0.5, egui::Color32::BLACK)).rounding(0.0).show(ui, |ui| {
                    ui.add_sized([90.0,20.0],egui::Label::new(dir_up.to_string()));

                });
                egui::Frame::canvas(ui.style()).stroke(egui::Stroke::new(0.5, egui::Color32::BLACK)).rounding(0.0).show(ui, |ui| {
                    if dir_up.zero() {
                        ui.add_sized([90.0,20.0],egui::Label::new("".to_string()));
                    } else {
                        ui.add_sized([90.0, 20.0], egui::Label::new(dir_down.format("%d/%m/%Y").to_string()));
                    }
                });

            });
        });
    }

    fn draw_datepicker(&mut self, date: &mut NaiveDate, update: &mut bool) {
        self.horizontal(|ui| {
            ui.label("Data Inicial");
            if ui.add(DatePickerButton::new(date).calendar(false)).changed(){
                *update = true;
            };
        });
    }
}

