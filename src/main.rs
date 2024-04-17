use std::io::{self};

use rand::Rng;

fn main() {
    println!("*******Adivine el Numero********");
    let number_to_guess = rand::thread_rng().gen_range(0..10);

    let mut input_number = String::new();

    let mut parsed_number: i32;
    
    loop {
        println!("Ingrese numero");
        io::stdin().read_line(&mut input_number).expect("Fallo al leer dato");
    
        parsed_number = match input_number.trim().parse() {
            Ok(number) => number,
            Err(err) => {
                println!("Entrada no valida: {}", err);
                return;
            }
        };

        if parsed_number == number_to_guess {
            println!("Adivinaste! :D");
            break;
        } else {
            println!("Sigue intentando");

            if  number_to_guess > parsed_number {
                println!("El numero es mas grande");
            } else {
                println!("El numero es mas pequeño");
            }

            input_number.clear();
            
        }
    }
}
