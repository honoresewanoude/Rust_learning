use std::io;

fn main(){

   
    println!("Mon premier jeu de devinette");
    println!("Entrer votre valeur");

    let mut dev = String::new();


    io::stdin()
        .read_line(&mut dev)
        .expect("Erreur de lecture");

    println!("Votre nombre deviner est : {} ", dev);
}
