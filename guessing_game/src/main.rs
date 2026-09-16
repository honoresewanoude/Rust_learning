use std::{cmp::Ordering, io};
use rand::Rng;


fn main(){

   
    println!("Mon premier jeu de devinette");

    let nombre_secret = rand::thread_rng().gen_range(1..=100);

    

    //let dev: u32 = dev.trim().parse().expect("S'il vous plait saisissez un nombre");

    loop {
    
    println!("Entrer votre valeur");
    let mut dev = String::new();

    io::stdin()
        .read_line(&mut dev)
        .expect("Erreur de lecture");

    let dev: u32 = match dev.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };


    println!("Votre nombre deviner est : {} ", dev);

    match dev.cmp(&nombre_secret){
        
        Ordering::Equal => {println!("Vous avez trouvé la bonne reponse");break},
        Ordering::Greater => println!("Le nombre que vous avez chois est trop grand"),
        Ordering::Less => println!("Le nombre que vous avez choisi est trop petit"),

    }

}
}
