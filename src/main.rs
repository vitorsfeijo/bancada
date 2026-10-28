use std::io;

fn main() {
    println!("Digite ls");

    let mut entrada = String::new();

    io::stdin().read_line(&mut entrada).expect("Falhar a ler");

    entrada = entrada.trim().to_string();

    if entrada == "ls" {
        println!("ls feito com sucesso");
    } else {
        println!("você digitou {}", entrada);
    }
}