use std::io;

fn main(){

   
    println!("Mon premier jeu de devinette");
    println!("Entrer votre valeur");

    let mut dev = string::new();


    io::stdin()
        .read_line(&mut dev);

    println!("Votre nombre deviner est : {} ", dev);
}
