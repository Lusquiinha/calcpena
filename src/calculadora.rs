use std::ops::Add;
use chrono::{Duration, Local, Months, NaiveDate};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Calculadora {
    pub datas: CalculosDeData,
    pub dd: DeltaData,
    pub data_inicial: NaiveDate,
    pub updt: bool,
    pub teste: u32,
}


impl Calculadora{
    pub fn new() -> Self{
        Self{
            datas: CalculosDeData::new(DeltaData{dia: 0, mes: 0, ano: 0}, Local::now().date_naive()),
            dd: DeltaData::new(0, 0, 0),
            data_inicial: Local::now().date_naive(),
            updt: false,
            teste: 0,
        }
    }
    pub fn calc_datas(&mut self, dd: DeltaData, data_inicial: NaiveDate) {
        self.datas = CalculosDeData::new(dd, data_inicial);
    }

    pub fn update(&mut self) {
        if self.updt{
            self.calc_datas(self.dd, self.data_inicial);
            self.updt = false;
            println!("{}", self.teste);
            self.teste+=1;
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct CalculosDeData{
    pub inteiro:            DeltaData,
    pub inteiro_data:       NaiveDate,
    pub um_oitavo:          DeltaData,
    pub um_oitavo_data:     NaiveDate,
    pub dezesseis_pct:      DeltaData,
    pub dezesseis_pct_data: NaiveDate,
    pub um_sexto:           DeltaData,
    pub um_sexto_data:      NaiveDate,
    pub um_quinto:          DeltaData,
    pub um_quinto_data:     NaiveDate,
    pub um_quarto:          DeltaData,
    pub um_quarto_data:     NaiveDate,
    pub trinta_pct:         DeltaData,
    pub trinta_pct_data:    NaiveDate,
    pub um_terco:           DeltaData,
    pub um_terco_data:      NaiveDate,
    pub dois_quintos:       DeltaData,
    pub dois_quintos_data:  NaiveDate,
    pub um_meio:            DeltaData,
    pub um_meio_data:       NaiveDate,
    pub tres_quintos:       DeltaData,
    pub tres_quintos_data:  NaiveDate,
    pub dois_tercos:        DeltaData,
    pub dois_tercos_data:   NaiveDate,
    pub setenta_pct:        DeltaData,
    pub setenta_pct_data:   NaiveDate,
}

impl CalculosDeData{
    pub fn new(dd: DeltaData, data_inicial: NaiveDate) -> Self{
        let meses = dd.ano * 12 + dd.mes;
        let mes_sexto = meses / 6;
        let um_sexto = DeltaData{dia: meses % 6 * 30 / 6 + dd.dia / 6, mes: mes_sexto % 12, ano: mes_sexto / 12};
        let mes_quinto = meses / 5;
        let um_quinto = DeltaData{dia: meses % 5 * 30 / 5 + dd.dia / 5, mes: mes_quinto % 12, ano: mes_quinto / 12};
        let mes_quarto = meses / 4;
        let um_quarto = DeltaData{dia: meses % 4 * 30 / 4 + dd.dia / 4, mes: mes_quarto % 12, ano: mes_quarto / 12};
        let mes_terco = meses / 3;
        let um_terco = DeltaData{dia: meses % 3 * 30 / 3 + dd.dia / 3, mes: mes_terco % 12, ano: mes_terco / 12};
        let mes_meio = meses / 2;
        let um_meio = DeltaData{dia: meses % 2 * 30 / 2 + dd.dia / 2, mes: mes_meio % 12, ano: mes_meio / 12};
        let mes_dois_tercos = meses * 2 / 3;
        let mut dois_tercos = DeltaData{dia: meses * 2 % 3 * 30 / 3 + dd.dia * 2 / 3, mes: mes_dois_tercos % 12, ano: mes_dois_tercos / 12};
        if dois_tercos.dia > 30 {
            dois_tercos.dia -= 30;
            dois_tercos.mes += 1;
        }
        let mes_oitavo = meses / 8;
        let um_oitavo = DeltaData{dia: meses % 8 * 30 / 8 + dd.dia / 8, mes: mes_oitavo % 12, ano: mes_oitavo / 12};
        let mes_dezesseis_pct: u32 = (meses as f32 * 0.16) as u32;
        let dezesseis_pct = DeltaData{dia: ((meses as f32 * 0.16 - mes_dezesseis_pct as f32) * 30.0 + dd.dia as f32 * 0.16) as u32, mes: mes_dezesseis_pct % 12, ano: mes_dezesseis_pct / 12};
        let mes_trinta_pct: u32 = (meses as f32 * 0.3) as u32;
        let trinta_pct = DeltaData{dia: ((meses as f32 * 0.3 - mes_trinta_pct as f32) * 30.0 + dd.dia as f32 * 0.3) as u32, mes: mes_trinta_pct % 12, ano: mes_trinta_pct / 12};
        let mes_dois_quintos: u32 = (meses as f32 * 0.4) as u32;
        let dois_quintos = DeltaData{dia: ((meses as f32 * 0.4 - mes_dois_quintos as f32) * 30.0 + dd.dia as f32 * 0.4) as u32, mes: mes_dois_quintos % 12, ano: mes_dois_quintos / 12};
        let mes_tres_quintos: u32 = (meses as f32 * 0.6) as u32;
        let mut tres_quintos = DeltaData{dia: ((meses as f32 * 0.6 - mes_tres_quintos as f32) * 30.0 + dd.dia as f32 * 0.6) as u32, mes: mes_tres_quintos % 12, ano: mes_tres_quintos / 12};
        let mes_setenta_pct: u32 = (meses as f32 * 0.7) as u32;
        let mut setenta_pct = DeltaData{dia: ((meses as f32 * 0.7 - mes_setenta_pct as f32) * 30.0 + dd.dia as f32 * 0.7) as u32, mes: mes_setenta_pct % 12, ano: mes_setenta_pct / 12};
        if tres_quintos.dia > 30 {
            tres_quintos.dia -= 30;
            tres_quintos.mes += 1;
        }
        if setenta_pct.dia > 30 {
            setenta_pct.dia -= 30;
            setenta_pct.mes += 1;
        }

        Self{
            inteiro: dd,
            inteiro_data: data_inicial + dd - Duration::days(1),
            um_oitavo,
            um_oitavo_data: data_inicial + um_oitavo - Duration::days(1),
            dezesseis_pct,
            dezesseis_pct_data: data_inicial + dezesseis_pct - Duration::days(1),
            um_sexto,
            um_sexto_data: data_inicial + um_sexto - Duration::days(1),
            um_quinto,
            um_quinto_data: data_inicial + um_quinto - Duration::days(1),
            um_quarto,
            um_quarto_data: data_inicial + um_quarto - Duration::days(1),
            trinta_pct,
            trinta_pct_data: data_inicial + trinta_pct - Duration::days(1),
            um_terco,
            um_terco_data: data_inicial + um_terco - Duration::days(1),
            dois_quintos,
            dois_quintos_data: data_inicial + dois_quintos - Duration::days(1),
            um_meio,
            um_meio_data: data_inicial + um_meio - Duration::days(1),
            tres_quintos,
            tres_quintos_data: data_inicial + tres_quintos - Duration::days(1),
            dois_tercos,
            dois_tercos_data: data_inicial + dois_tercos - Duration::days(1),
            setenta_pct,
            setenta_pct_data: data_inicial + setenta_pct - Duration::days(1),
        }
    }
}

impl PartialEq for CalculosDeData {
    fn eq(&self, other: &Self) -> bool {
        let mut res: bool = self.inteiro == other.inteiro;
        res = res && self.um_sexto == other.um_sexto;
        res = res && self.um_quinto == other.um_quinto;
        res = res && self.um_quarto == other.um_quarto;
        res = res && self.um_terco == other.um_terco;
        res = res && self.um_meio == other.um_meio;
        res = res && self.dois_tercos == other.dois_tercos;
        res
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeltaData {
    pub dia: u32,
    pub mes: u32,
    pub ano: u32
}

impl DeltaData {
    pub fn new(dia: u32, mes: u32, ano: u32) -> Self {
        Self { dia, mes, ano }
    }
    pub fn to_string(&self) -> String {
        let mut string= String::new();
        if self.ano != 0 {
            string += format!("{} a ", self.ano).as_str();
        }
        if self.mes != 0 {
            string += format!("{} m ", self.mes).as_str();
        }
        if self.dia != 0 {
            string += format!("{} d", self.dia).as_str();
        }
        string
    }
    pub fn zero(&self) -> bool {
        self.dia == 0 && self.mes == 0 && self.ano == 0
    }
}


impl Add<DeltaData> for NaiveDate {
    type Output = NaiveDate;

    fn add(self, rhs: DeltaData) -> Self::Output {
        self + Duration::days(rhs.dia as i64) + Months::new(rhs.mes + rhs.ano * 12)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_calculadora_new() {
        let dd = DeltaData { dia: 1, mes: 1, ano: 1 };
        let mut calc = Calculadora::new();
        calc.calc_datas(dd, Local::now().date_naive());

        // Check if the `inteiro_data` is not in the past
        assert!(calc.datas.inteiro_data >= Utc::now().date_naive());
    }


    #[test]
    fn test_1() {
        let dd = DeltaData { dia: 1, mes: 1, ano: 1 };
        let mut calc = Calculadora::new();
        calc.calc_datas(dd, Local::now().date_naive());
        let um_sexto =      DeltaData { dia: 5, mes: 2, ano: 0 };
        let um_quinto =     DeltaData { dia: 18, mes: 2, ano: 0 };
        let um_quarto =     DeltaData { dia: 7, mes: 3, ano: 0 };
        let um_terco =      DeltaData { dia: 10, mes: 4, ano: 0 };
        let um_meio =       DeltaData { dia: 15, mes: 6, ano: 0 };
        let dois_tercos =   DeltaData { dia: 20, mes: 8, ano: 0 };

        let expected = Calculadora { datas: CalculosDeData {
            inteiro: dd,
            inteiro_data: Local::now().date_naive() + dd,
            um_sexto,
            um_sexto_data: Local::now().date_naive() + um_sexto,
            um_quinto,
            um_quinto_data: Local::now().date_naive() + um_quinto,
            um_quarto,
            um_quarto_data: Local::now().date_naive() + um_quarto,
            um_terco,
            um_terco_data: Local::now().date_naive() + um_terco,
            um_meio,
            um_meio_data: Local::now().date_naive() + um_meio,
            dois_tercos,
            dois_tercos_data: Local::now().date_naive() + dois_tercos,
        },
        ..calc};
        println!("{:?}", calc);
        println!("{:?}", expected);
        assert_eq!(calc, expected);
    }

    #[test]
    fn test_2() {
        let dd = DeltaData { dia: 25, mes: 4, ano: 8 };
        let mut calc = Calculadora::new();
        calc.calc_datas(dd, Local::now().date_naive());
        let um_sexto =      DeltaData { dia: 24, mes: 4 , ano: 1 };
        let um_quinto =     DeltaData { dia: 5 , mes: 8 , ano: 1 };
        let um_quarto =     DeltaData { dia: 6 , mes: 1 , ano: 2 };
        let um_terco =      DeltaData { dia: 18, mes: 9 , ano: 2 };
        let um_meio =       DeltaData { dia: 12, mes: 2 , ano: 4 };
        let dois_tercos =   DeltaData { dia: 6 , mes: 7 , ano: 5 };

        let expected = Calculadora { datas: CalculosDeData {
            inteiro: dd,
            inteiro_data: Local::now().date_naive() + dd,
            um_sexto,
            um_sexto_data: Local::now().date_naive() + um_sexto,
            um_quinto,
            um_quinto_data: Local::now().date_naive() + um_quinto,
            um_quarto,
            um_quarto_data: Local::now().date_naive() + um_quarto,
            um_terco,
            um_terco_data: Local::now().date_naive() + um_terco,
            um_meio,
            um_meio_data: Local::now().date_naive() + um_meio,
            dois_tercos,
            dois_tercos_data: Local::now().date_naive() + dois_tercos,
        },
            ..calc};
        println!("{:?}", calc);
        println!("{:?}", expected);
        assert_eq!(calc, expected);
    }
    #[test]
    fn test_3() {
        let dd = DeltaData { dia: 30, mes: 12, ano: 99 };
        let mut calc = Calculadora::new();
        calc.calc_datas(dd, Local::now().date_naive());
        let um_sexto =      DeltaData { dia: 5, mes: 8 , ano: 16};
        let um_quinto =     DeltaData { dia: 6 , mes: 0 , ano: 20};
        let um_quarto =     DeltaData { dia: 7 , mes: 0 , ano: 25};
        let um_terco =      DeltaData { dia: 10, mes: 4 , ano: 33};
        let um_meio =       DeltaData { dia: 15, mes: 0 , ano: 50};
        let dois_tercos =   DeltaData { dia: 20, mes: 8 , ano: 66};

        let expected = Calculadora { datas: CalculosDeData {
            inteiro: dd,
            inteiro_data: Local::now().date_naive() + dd,
            um_sexto,
            um_sexto_data: Local::now().date_naive() + um_sexto,
            um_quinto,
            um_quinto_data: Local::now().date_naive() + um_quinto,
            um_quarto,
            um_quarto_data: Local::now().date_naive() + um_quarto,
            um_terco,
            um_terco_data: Local::now().date_naive() + um_terco,
            um_meio,
            um_meio_data: Local::now().date_naive() + um_meio,
            dois_tercos,
            dois_tercos_data: Local::now().date_naive() + dois_tercos,
        },
            ..calc};
        println!("{:?}", calc);
        println!("{:?}", expected);
        assert_eq!(calc, expected);
    }
    #[test]
    fn test_4() {
        let dd = DeltaData { dia: 22, mes: 6, ano: 11 };
        let mut calc = Calculadora::new();
        calc.calc_datas(dd, Local::now().date_naive());
        let um_sexto =      DeltaData { dia: 3, mes: 11, ano: 1};
        let um_quinto =     DeltaData { dia: 22, mes: 3 , ano: 2};
        let um_quarto =     DeltaData { dia: 20, mes: 10, ano: 2};
        let um_terco =      DeltaData { dia: 7, mes: 10, ano: 3};
        let um_meio =       DeltaData { dia: 11, mes: 9 , ano: 5};
        let dois_tercos =   DeltaData { dia: 14, mes: 8 , ano: 7};

        let expected = Calculadora { datas: CalculosDeData {
            inteiro: dd,
            inteiro_data: Local::now().date_naive() + dd,
            um_sexto,
            um_sexto_data: Local::now().date_naive() + um_sexto,
            um_quinto,
            um_quinto_data: Local::now().date_naive() + um_quinto,
            um_quarto,
            um_quarto_data: Local::now().date_naive() + um_quarto,
            um_terco,
            um_terco_data: Local::now().date_naive() + um_terco,
            um_meio,
            um_meio_data: Local::now().date_naive() + um_meio,
            dois_tercos,
            dois_tercos_data: Local::now().date_naive() + dois_tercos,
        },
            ..calc};
        println!("{:?}", calc);
        println!("{:?}", expected);
        assert_eq!(calc, expected);
    }
    #[test]
    fn test_5() {
        let dd = DeltaData { dia: 0, mes: 8, ano: 0 };
        let mut calc = Calculadora::new();
        calc.calc_datas(dd, Local::now().date_naive());
        let um_sexto =      DeltaData { dia: 10, mes: 1, ano: 0};
        let um_quinto =     DeltaData { dia: 18, mes: 1 , ano: 0};
        let um_quarto =     DeltaData { dia: 0, mes: 2, ano: 0};
        let um_terco =      DeltaData { dia: 20, mes: 2, ano: 0};
        let um_meio =       DeltaData { dia: 0, mes: 4 , ano: 0};
        let dois_tercos =   DeltaData { dia: 10, mes: 5 , ano: 0};

        let expected = Calculadora { datas: CalculosDeData {
            inteiro: dd,
            inteiro_data: Local::now().date_naive() + dd,
            um_sexto,
            um_sexto_data: Local::now().date_naive() + um_sexto,
            um_quinto,
            um_quinto_data: Local::now().date_naive() + um_quinto,
            um_quarto,
            um_quarto_data: Local::now().date_naive() + um_quarto,
            um_terco,
            um_terco_data: Local::now().date_naive() + um_terco,
            um_meio,
            um_meio_data: Local::now().date_naive() + um_meio,
            dois_tercos,
            dois_tercos_data: Local::now().date_naive() + dois_tercos,
        },
            ..calc};
        println!("{:?}", calc);
        println!("{:?}", expected);
        assert_eq!(calc, expected);
    }
}
