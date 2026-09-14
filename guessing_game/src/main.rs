use std::io;
use rand::Rng;

fn main(){

   
    println!("Mon premier jeu de devinette");
    println!("Entrer votre valeur");

    let nombre_secret = rand::thread_rng().gen_range(1..=100);

    let mut dev = String::new();

    println!("Le nombre secret est : {}",nombre_secret);


    io::stdin()
        .read_line(&mut dev)
        .expect("Erreur de lecture");

    println!("Votre nombre deviner est : {} ", dev);
}
