use std::ops::Add;
use chrono::{DateTime, Duration, Local, Months};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Calculadora {
    pub datas: CalculosDeData,
}


impl Calculadora{
    pub fn new() -> Self{
        Self{
            datas: CalculosDeData::new(DeltaData{dia: 0, mes: 0, ano: 0})
        }
    }
    pub fn calc_datas(&mut self, dd: DeltaData) {
        self.datas = CalculosDeData::new(dd);
    }
}
#[derive(Clone, Copy, Debug)]
pub struct CalculosDeData{
    inteiro:            DeltaData,
    inteiro_data:       DateTime<Local>,
    um_sexto:           DeltaData,
    um_sexto_data:      DateTime<Local>,
    um_quinto:          DeltaData,
    um_quinto_data:     DateTime<Local>,
    um_quarto:          DeltaData,
    um_quarto_data:     DateTime<Local>,
    um_terco:           DeltaData,
    um_terco_data:      DateTime<Local>,
    um_meio:            DeltaData,
    um_meio_data:       DateTime<Local>,
    dois_tercos:        DeltaData,
    dois_tercos_data:   DateTime<Local>,
}

impl CalculosDeData{
    pub fn new(dd: DeltaData) -> Self{
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
        Self{
            inteiro: dd,
            inteiro_data: Local::now() + dd - Duration::days(1),
            um_sexto,
            um_sexto_data: Local::now() + um_sexto - Duration::days(1),
            um_quinto,
            um_quinto_data: Local::now() + um_quinto - Duration::days(1),
            um_quarto,
            um_quarto_data: Local::now() + um_quarto - Duration::days(1),
            um_terco,
            um_terco_data: Local::now() + um_terco - Duration::days(1),
            um_meio,
            um_meio_data: Local::now() + um_meio - Duration::days(1),
            dois_tercos,
            dois_tercos_data: Local::now() + dois_tercos - Duration::days(1),
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
    dia: u32,
    mes: u32,
    ano: u32
}

impl DeltaData {
    pub fn new(dia: u32, mes: u32, ano: u32) -> Self {
        Self { dia, mes, ano }
    }
    pub fn to_string(&self) -> String {
        format!("{} a, {} m, {} d", self.ano, self.mes, self.dia)
    }
}


impl Add<DeltaData> for DateTime<Local> {
    type Output = DateTime<Local>;

    fn add(self, rhs: DeltaData) -> Self::Output {
        self + Duration::days(rhs.dia as i64) + Months::new(rhs.mes + rhs.ano * 12)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::PartialEq;
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_calculadora_new() {
        let dd = DeltaData { dia: 1, mes: 1, ano: 1 };
        let mut calc = Calculadora::new();
        calc.calc_datas(dd);

        // Check if the `inteiro_data` is not in the past
        assert!(calc.datas.inteiro_data >= Utc::now());
    }


    #[test]
    fn test_1() {
        let dd = DeltaData { dia: 1, mes: 1, ano: 1 };
        let mut calc = Calculadora::new();
        calc.calc_datas(dd);
        let um_sexto =      DeltaData { dia: 5, mes: 2, ano: 0 };
        let um_quinto =     DeltaData { dia: 18, mes: 2, ano: 0 };
        let um_quarto =     DeltaData { dia: 7, mes: 3, ano: 0 };
        let um_terco =      DeltaData { dia: 10, mes: 4, ano: 0 };
        let um_meio =       DeltaData { dia: 15, mes: 6, ano: 0 };
        let dois_tercos =   DeltaData { dia: 20, mes: 8, ano: 0 };

        let expected = Calculadora { datas: CalculosDeData {
            inteiro: dd,
            inteiro_data: Local::now() + dd,
            um_sexto,
            um_sexto_data: Local::now() + um_sexto,
            um_quinto,
            um_quinto_data: Local::now() + um_quinto,
            um_quarto,
            um_quarto_data: Local::now() + um_quarto,
            um_terco,
            um_terco_data: Local::now() + um_terco,
            um_meio,
            um_meio_data: Local::now() + um_meio,
            dois_tercos,
            dois_tercos_data: Local::now() + dois_tercos,
        }};
        println!("{:?}", calc);
        println!("{:?}", expected);
        assert_eq!(calc, expected);
    }

    #[test]
    fn test_2() {
        let dd = DeltaData { dia: 25, mes: 4, ano: 8 };
        let mut calc = Calculadora::new();
        calc.calc_datas(dd);
        let um_sexto =      DeltaData { dia: 24, mes: 4 , ano: 1 };
        let um_quinto =     DeltaData { dia: 5 , mes: 8 , ano: 1 };
        let um_quarto =     DeltaData { dia: 6 , mes: 1 , ano: 2 };
        let um_terco =      DeltaData { dia: 18, mes: 9 , ano: 2 };
        let um_meio =       DeltaData { dia: 12, mes: 2 , ano: 4 };
        let dois_tercos =   DeltaData { dia: 6 , mes: 7 , ano: 5 };

        let expected = Calculadora { datas: CalculosDeData {
            inteiro: dd,
            inteiro_data: Local::now() + dd,
            um_sexto,
            um_sexto_data: Local::now() + um_sexto,
            um_quinto,
            um_quinto_data: Local::now() + um_quinto,
            um_quarto,
            um_quarto_data: Local::now() + um_quarto,
            um_terco,
            um_terco_data: Local::now() + um_terco,
            um_meio,
            um_meio_data: Local::now() + um_meio,
            dois_tercos,
            dois_tercos_data: Local::now() + dois_tercos,
        }};
        println!("{:?}", calc);
        println!("{:?}", expected);
        assert_eq!(calc, expected);
    }
    #[test]
    fn test_3() {
        let dd = DeltaData { dia: 30, mes: 12, ano: 99 };
        let mut calc = Calculadora::new();
        calc.calc_datas(dd);
        let um_sexto =      DeltaData { dia: 5, mes: 8 , ano: 16};
        let um_quinto =     DeltaData { dia: 6 , mes: 0 , ano: 20};
        let um_quarto =     DeltaData { dia: 7 , mes: 0 , ano: 25};
        let um_terco =      DeltaData { dia: 10, mes: 4 , ano: 33};
        let um_meio =       DeltaData { dia: 15, mes: 0 , ano: 50};
        let dois_tercos =   DeltaData { dia: 20, mes: 8 , ano: 66};

        let expected = Calculadora { datas: CalculosDeData {
            inteiro: dd,
            inteiro_data: Local::now() + dd,
            um_sexto,
            um_sexto_data: Local::now() + um_sexto,
            um_quinto,
            um_quinto_data: Local::now() + um_quinto,
            um_quarto,
            um_quarto_data: Local::now() + um_quarto,
            um_terco,
            um_terco_data: Local::now() + um_terco,
            um_meio,
            um_meio_data: Local::now() + um_meio,
            dois_tercos,
            dois_tercos_data: Local::now() + dois_tercos,
        }};
        println!("{:?}", calc);
        println!("{:?}", expected);
        assert_eq!(calc, expected);
    }
    #[test]
    fn test_4() {
        let dd = DeltaData { dia: 22, mes: 6, ano: 11 };
        let mut calc = Calculadora::new();
        calc.calc_datas(dd);
        let um_sexto =      DeltaData { dia: 3, mes: 11, ano: 1};
        let um_quinto =     DeltaData { dia: 22, mes: 3 , ano: 2};
        let um_quarto =     DeltaData { dia: 20, mes: 10, ano: 2};
        let um_terco =      DeltaData { dia: 7, mes: 10, ano: 3};
        let um_meio =       DeltaData { dia: 11, mes: 9 , ano: 5};
        let dois_tercos =   DeltaData { dia: 14, mes: 8 , ano: 7};

        let expected = Calculadora { datas: CalculosDeData {
            inteiro: dd,
            inteiro_data: Local::now() + dd,
            um_sexto,
            um_sexto_data: Local::now() + um_sexto,
            um_quinto,
            um_quinto_data: Local::now() + um_quinto,
            um_quarto,
            um_quarto_data: Local::now() + um_quarto,
            um_terco,
            um_terco_data: Local::now() + um_terco,
            um_meio,
            um_meio_data: Local::now() + um_meio,
            dois_tercos,
            dois_tercos_data: Local::now() + dois_tercos,
        }};
        println!("{:?}", calc);
        println!("{:?}", expected);
        assert_eq!(calc, expected);
    }
    #[test]
    fn test_5() {
        let dd = DeltaData { dia: 0, mes: 8, ano: 0 };
        let mut calc = Calculadora::new();
        calc.calc_datas(dd);
        let um_sexto =      DeltaData { dia: 10, mes: 1, ano: 0};
        let um_quinto =     DeltaData { dia: 18, mes: 1 , ano: 0};
        let um_quarto =     DeltaData { dia: 0, mes: 2, ano: 0};
        let um_terco =      DeltaData { dia: 20, mes: 2, ano: 0};
        let um_meio =       DeltaData { dia: 0, mes: 4 , ano: 0};
        let dois_tercos =   DeltaData { dia: 10, mes: 5 , ano: 0};

        let expected = Calculadora { datas: CalculosDeData {
            inteiro: dd,
            inteiro_data: Local::now() + dd,
            um_sexto,
            um_sexto_data: Local::now() + um_sexto,
            um_quinto,
            um_quinto_data: Local::now() + um_quinto,
            um_quarto,
            um_quarto_data: Local::now() + um_quarto,
            um_terco,
            um_terco_data: Local::now() + um_terco,
            um_meio,
            um_meio_data: Local::now() + um_meio,
            dois_tercos,
            dois_tercos_data: Local::now() + dois_tercos,
        }};
        println!("{:?}", calc);
        println!("{:?}", expected);
        assert_eq!(calc, expected);
    }
}
