use crate::utils;
use std::io;

pub fn user_input() {
    let mut number1 = String::new();
    io::stdin().read_line(&mut number1).expect("Deu erro");

    let mut number2 = String::new();
    io::stdin().read_line(&mut number2).expect("Deu ruim");

    if utils::convert_to_int(&number1) > utils::convert_to_int(&number2) {
        println!("{} é maior que {}", number1, number2);
    } else {
        println!("{} é menor ou igual a {}", number1, number2);
    }
}
