use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..101);

    loop {
        println!("Introduce un numero entre el 1 y el 100");

        let mut reader = String::new();
        io::stdin()
            .read_line(&mut reader)
            .expect("Fallo al leer stdin");

        let input_text: i32 = reader
            .trim()
            .parse()
            .expect("Debe introducir un numero valido");

        if input_text > random_number {
            println!("El numero es mayor, introduce otro:");
        } else if input_text < random_number {
            println!("El numero es menor, introduce otro:");
        } else {
            println!("El numero es CORRECTO!");
            break;
        }
    }
}
