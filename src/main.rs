use std::io;
use rand::Rng;

fn main() {
    println!("Bem-vindo!");

    let mut random = rand::thread_rng();
    let secretnumber: i8 = random.gen_range(0..=19) + 1;
    let tries: i8 = 5;
    let mut plays: i8 = 0;
    let mut numstring = String::new();

    let mut randomic_points = rand::thread_rng();
    let random_reducer: i16 = randomic_points.gen_range(5..=15);
    let mut points: i16 = 100;

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
        points -= random_reducer;
    }

    if plays > tries {
        println!("Suas chances acabaram");
        println!("Sua pontuação foi: {}", points.to_string());
        break;
    } else {
        print!("Você jogou {} vezes, com limite de 5\n", plays)
    }
}
}
