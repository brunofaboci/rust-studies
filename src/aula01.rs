pub fn variaveis() {
  let mut name = "Bruno";
    println!("Hello, {}!", name);
    name = "Joao";
    println!("Hello, {}!", name);

    // tipos de dados

    // integer
    // i8, i16, i32, i64, i128.....
    let age = 25;
    println!("{}", age);

    let new_age: i128 = 25;
    println!("{}", new_age);

    // para criar variaveis numericas que nao permitam valores negativos, usamos o "unsigned"
    let year: u64 = 2024;
    println!("{}", year);

    // float
    let height: f32 = 1.79;
    println!("{}", height);

    // bool
    let is_true: bool = true;
    println!("{}", is_true);
}
