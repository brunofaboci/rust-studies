// use std::io;
mod aula01;
mod aula02;
mod aula03;
mod aula04;
mod aula05;

mod utils {
    pub fn convert_to_int(data_input: &String) -> i32 {
        let x = data_input.trim().parse::<i32>().unwrap();
        x
    }
}


fn main() {
    aula01::variaveis();
    aula02::controle_fluxo();
    aula03::user_input();
    aula04::soma_algarismos();
    aula05::fatorial();
}
