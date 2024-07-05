use eframe::{App, egui, Frame};
use eframe::egui::Context;
use crate::calculadora::{Calculadora, DeltaData};
use crate::input_numerico::InputNumerico;
use chrono::{NaiveDate};
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
                self.anos.show(ui, &mut self.calc.dd.ano, &mut self.calc.updt);
                self.meses.show(ui, &mut self.calc.dd.mes, &mut self.calc.updt);
                self.dias.show(ui, &mut self.calc.dd.dia, &mut self.calc.updt);
            });
            self.calc.update();
            ui.add_space(50.0);
            ui.draw_datepicker(&mut self.calc.data_inicial, &mut self.calc.updt);
            ui.add_space(50.0);

            ui.horizontal(|ui|{
                ui.vertical(|ui|{
                    ui.triple_label("1/6".to_string(), self.calc.datas.um_sexto, self.calc.datas.um_sexto_data);
                    ui.triple_label("1/5".to_string(), self.calc.datas.um_quinto, self.calc.datas.um_quinto_data);
                    ui.triple_label("1/4".to_string(), self.calc.datas.um_quarto, self.calc.datas.um_quarto_data);
                });
                ui.vertical(|ui|{
                    ui.triple_label("1/3".to_string(), self.calc.datas.um_terco, self.calc.datas.um_terco_data);
                    ui.triple_label("1/2".to_string(), self.calc.datas.um_meio, self.calc.datas.um_meio_data);
                    ui.triple_label("2/3".to_string(), self.calc.datas.dois_tercos, self.calc.datas.dois_tercos_data);
                });
            });

        });
    }
}
impl Calcpena {
    pub(crate) fn default() -> Self {
        Self {
            anos:  InputNumerico::new("anos: ".to_string(), 999),
            meses: InputNumerico::new("meses: ".to_string(), 12),
            dias:  InputNumerico::new("dias: ".to_string(), 30),
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
            ui.label(esq);
            ui.vertical(|ui| {
                if dir_up.zero(){
                    ui.label("0".to_string());
                    ui.label("0".to_string());
                }
                else{
                    ui.label(dir_up.to_string());
                    ui.label(dir_down.format("%d/%m/%Y").to_string());

                }
            });
        });
    }

    fn draw_datepicker(&mut self, date: &mut NaiveDate, update: &mut bool) {
        self.horizontal(|ui| {
            ui.label("Data Inicial");
            if ui.add(DatePickerButton::new(date)).changed(){
                *update = true;
            };
        });
    }
}

