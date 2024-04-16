// use exercicios::mdc_01;

use exercicios::mdc_01;

mod aula01;
mod aula02;
mod aula03;
mod aula04;
mod aula05;
mod aula06;
mod exercicios {
    pub mod mdc_01;
}

mod utils {
    pub fn convert_to_int(data_input: &String) -> i32 {
        let x = data_input.trim().parse::<i32>().unwrap();
        x
    }

    pub fn convert_to_float(data_input: &String) -> f32 {
        let x = data_input.trim().parse::<f32>().unwrap();
        x
    }
}


fn main() {
    aula01::variaveis();
    aula02::controle_fluxo();
    aula03::user_input();
    aula04::soma_algarismos();
    aula05::fatorial();
    aula06::alunos_em_recuperacao();
    mdc_01::calculo_mdc();
}
