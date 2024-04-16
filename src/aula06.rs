use crate::utils;
use std::io;

pub fn alunos_em_recuperacao() {
    let mut medias: [f32; 3] = [0.0; 3];
    let mut index = 0;
    let mut qtd_medias = 3;
    let mut recuperacao = 0;

    while qtd_medias > 0 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("deu ruim");

        medias[index] = utils::convert_to_float(&input);
        index += 1;
        qtd_medias -= 1;
    }

    for media in medias {
      if media >= 4.0 && media < 6.0 {
        recuperacao += 1;
      }
    }

    if recuperacao == 0 {
      println!("Ninguem ficou de recuperacao")
    } else {
      println!("{} aluno(s) ficaram em recuperacao", recuperacao)
    }
}
