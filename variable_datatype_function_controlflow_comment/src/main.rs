use std::io;

//fonctions
fn devine_tableau(){

    let nom : [&str;6] = ["Honoré","Marc","Jeannot","Caster","Oriel","Rose"];
    println!("Les noms du tableau sont : {}, {}, {}, {}, {}, {}",nom[0],nom[1],nom[2],nom[3],nom[4],nom[5]);

}

fn comparaison(y : i32, x : i32) -> String {

    if x > y {
        format!(" {x} est plus grang que {x}")
    } else{
        format!("nous sommes dans un cas d'inferiorité ou d'égalité")
    } 
}

fn analyse_nombre(nombres: i32) -> i32 {

    let mut nombre: i32 = nombres;

    let categorie = if nombre > 0 {
        "POSITIF"
    } else if nombre == 0 {
        "ZERO"
    } else {
        "NEGATIF"
    };

    println!("{}",categorie);


    if nombre % 2 == 0{
        println!("NOMBRE PAIR");
    } else {
        println!("NOMBRE IMPAIRE");
    }

    for number in  1..=nombre {

        if number % 2 == 0 {
            continue;
        }
        println!("Les nombres sont : {number}");
    }

    while nombre >= 0 {
        println!("{}",nombre);
        nombre = nombre -1;
    }

    let mut recherche : i32 = 1;

    loop {
        if recherche % 3 == 0 && recherche % 5 == 0 {
            break recherche;
        }

        recherche += 1;
    }



}

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

    devine_tableau();
    println!("{}",comparaison(122, 78));

    let res = analyse_nombre(12);
    println!("{}",res);
}


