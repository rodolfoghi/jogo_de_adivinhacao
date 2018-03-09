use std::io;

fn main() {
    println!("Adivinhe o número!");

    println!("Digite o seu palpite.");

    let mut palpite = String::new();

    io::stdin().read_line(&mut palpite)
        .expect("Falha ao ler entrada");

    println!("O seu palpite é {}", palpite);
}
