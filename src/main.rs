use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Jouons au nombre mystère !");

    let borne_min = 1;
    let borne_max = 10;

    let nombre_mystere = rand::thread_rng().gen_range(borne_min..=borne_max);

    loop {
        println!("Entrez un nombre entre {borne_min} et {borne_max} !");

        let mut nombre_envoye = String::new();

        io::stdin()
            .read_line(&mut nombre_envoye)
            .expect("Echec de la lecture de la ligne");

        let nombre_envoye: i32 = match nombre_envoye.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Nombre envoyé {}", nombre_envoye);

        match nombre_envoye.cmp(&nombre_mystere) {
            Ordering::Less => println!("{}", "Trop petit !".red()),
            Ordering::Greater => println!("{}", "Trop grand !".red()),
            Ordering::Equal => {
                println!("{}", "Parfait ! C'est ça :)".green());
                break;
            }
        }
    }
}
