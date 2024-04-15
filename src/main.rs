use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    // // variaveis
    // let mut name = "Bruno";
    // println!("Hello, {}!", name);
    // name = "Joao";
    // println!("Hello, {}!", name);

    // // tipos de dados

    // // integer
    // // i8, i16, i32, i64, i128.....
    // let age = 25;
    // println!("{}", age);

    // let new_age: i128 = 25;
    // println!("{}", new_age);

    // // para criar variaveis numericas que nao permitam valores negativos, usamos o "unsigned"
    // let year: u64 = 2024;
    // println!("{}", year);

    // // float
    // let height: f32 = 1.79;
    // println!("{}", height);

    // // bool
    // let is_true: bool = true;
    // println!("{}", is_true);

    // // controle de fluxo
    // let n1 = 20;
    // let n2 = 40;

    // if n1 > n2 {
    //     println!("{} > {}", n1, n2)
    // }
    // else {
    //     println!("{} <= {}", n1, n2)
    // }

    // user input
    let mut number1 = String::new();
    io::stdin().read_line(&mut number1).expect("Deu erro");

    let mut number2 = String::new();
    io::stdin().read_line(&mut number2).expect("Deu ruim");

    if convert_to_int(&number1) > convert_to_int(&number2) {
        println!("{} é maior que {}", number1, number2);
    } else {
        println!("{} é menor ou igual a {}", number1, number2);
    }
}
