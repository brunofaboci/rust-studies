use crate::utils;
use std::io;

pub fn soma_algarismos() {
    let mut soma = 0;
    let mut valor_entrada = String::new();
    io::stdin().read_line(&mut valor_entrada).expect("deu ruim");
    let mut valor_i32 = utils::convert_to_int(&valor_entrada);

    while valor_i32 != 0 {
        let resto = valor_i32 % 10;
        soma = soma + resto;
        valor_i32 = valor_i32 / 10;
    }

    println!("o fatorial de {} é igual a {}", valor_entrada, soma);
}
