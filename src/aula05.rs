use crate::utils;
use std::io;

pub fn fatorial() {
  let mut user_input = String::new();
  io::stdin().read_line(&mut user_input).expect("deu ruim");
  let mut valor = utils::convert_to_int(&user_input);

  let mut fatorial = 1;

  while valor > 1 {
    fatorial = fatorial * valor;
    valor = valor - 1;
  }

  println!("o fatorial de {} é igual a {}", user_input, fatorial)
}
