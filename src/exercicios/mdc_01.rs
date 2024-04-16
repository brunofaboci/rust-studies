pub fn calculo_mdc() {
  let mut maior = 40;
  let mut menor = 15;
  let mut resto = maior % menor;

  while resto > 0 {
    maior = menor;
    menor = resto;
    resto = maior % menor
  }

  let mdc = menor;

  println!("O MDC entre 40 e 15 é igual a {}", mdc)
}
