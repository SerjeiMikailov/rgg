use std::io;

fn main() {
    println!("Bem-vindo!");

    let secretnumber: i8 = 14;
    let tries: i8 = 5;
    let mut plays: i8 = 0;
    let mut numstring = String::new();

    

    loop {
    println!("Digite um número entre 1 e 20:\n");
    numstring.clear();
    io::stdin().read_line(&mut numstring).expect("Erro ao ler entrada");

    let guess: i8 = match numstring.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Entrada inválida!");
            return;
        }
    };

    if guess == secretnumber {
        println!("Você acertou!");
        break;
    } else if guess <= 0 || guess > 20 {
        println!("O número deve estar entre 1 e 20!");
    } else {
        plays += 1;
    }

    if plays > tries {
        println!("Suas chances acabaram");
        break;
    } else {
        print!("Você jogou {} vezes, com limite de 5\n", plays)
    }
}
}
