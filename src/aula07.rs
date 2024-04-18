pub fn functions() {
  let num = 8;
  let a = 15;
  let b = 3;
  println!("o dobro de {} é {}", num, dobro(num));
  println!("o maior valor entre {} e {} é {}", a, b, maior(a, b));
  println!("o produto entre {} e {} é igual a {}", a, b, casting(a, b));
}

fn dobro(num: i32) -> i32 {
  return num * 2;
}

fn maior(a: i32, b: i32) -> i32 {
  if a > b {
    return a; // sintax de retorno explícita
  } else {
    b // sintax de retorno implicita
  }
}

fn casting(a: i32, b: i32) -> f32 {
  let product = a * b;
  product as f32
}
