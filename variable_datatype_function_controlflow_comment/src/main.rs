use std::io;

fn main(){
    
    //utilisation et déclaration des variables

    let age: i32 = 23;

    let taille: f64 = 1.8;

    let majeur: bool = true;

    let initiale: char = 'S';

    let nom: &str = "Honoré";

    let annee_naissance:  i32 = 2026 - age;

    println!("Je m'appelle {}, j'ai {}, je mésure {} m. Mon initial est {}. Mon année de naissance est: {}",nom,age,taille,initiale,annee_naissance);


    println!("Entrez votre age réel :");

    let mut age_reel = String::new();

    io::stdin()
        .read_line(&mut age_reel)
        .expect("Erreur de nombre");

    let age_reel : u32 = age_reel.trim().parse().expect("Eurreur de convertion");

    let y = age_reel;

    let y = age_reel + 5;

    println!("Dans 5 ans vous aurez : {}",y);
}
