use std::io;

fn main(){
    println!("olá, qual seu nome?");
    let mut nome = String::new();
    io::stdin()
        .read_line(&mut nome)
        .expect("Falha ao ler a linha");
    println!("agora me diga a sua idade: ");
    let mut inputing = String::new();
    let mut numero: i32 = 45;
    io::stdin()
        .read_line(&mut inputing)
        .expect("Falha ao ler a linha");
    numero = inputing.trim().parse()
        .expect("Digita um numero valido");

    println!("ola, {} com {}", nome.trim(), numero);
}