EXO1 :
fn afficher(texte: &String) {
    println!("{texte}");
}

fn main() {
    let message = String::from("Bonjour");

    afficher(&message);

    println!("{message}");
}

EXO2 : 
fn afficher(texte: String) {
    println!("{texte}");
}

fn main() {
    let message = String::from("Bonjour");

    afficher(message);

    println!("{message}");
}

EXO4 :
fn longueur(s: &String) -> usize {
    s.len()
}

fn main() {
    let texte = String::from("Rust");

    let taille = longueur(&texte);

    println!("{texte}");
    println!("{taille}");
}

EXO 5:
fn main() {
    let texte = String::from("Rust");

    let r1 = &texte;
    let r2 = &texte;
    let r3 = &texte;

    println!("{r1}");
    println!("{r2}");
    println!("{r3}");
}

EXO6 : 
fn modifier(s: &mut String) {
    s.push_str(" est puissant");
}

fn main() {
    let mut langage = String::from("Rust");

    modifier(&mut langage);

    println!("{langage}");
}

EXO7 :
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{r1}");
    println!("{r2}");
}

EXO8:
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    let r3 = &mut s;

    println!("{r1}");
    println!("{r2}");
    println!("{r3}");
}

Exo 1 : 
    Oui ça compile
    message possède le string
    Non afficher ne devient pas propriétaire, car il a emprunter la valeur. Il a une refence de la valeur pour aller la lire
    Message est toujours dans ca scope , propriétaire de ca valeur, rust n'a pas faire le drop(), parceque afficher l'avait emprunter et non transferer la propriétaire

EXO 2: 
    Non ca ne compile pas 
    En passant le string à afficher le transfert de propiété à été fait vers texte. Donc message perd la propiété et texte devient le propritaire. En essayant afficher message en bas dans le main, ca ne marchera pas. parceque rust a dejà drop la valeur dans message 

EXO 3:
    Appel	        Paramètre	Ownership transféré ?
afficher(message)	String	      oui
afficher(&message)	&String	      non

EXO 4:
    Pourquoi tu dis "qui" ? C'est nomalement que possède texte, puisque c'est elle la variable. Bon je ne sais pas si je me trompe.
    texte possède le string.
    s ne possède rien 
    Quand longeur se termine , s retourne une valeure qui est la longeur de s. Je rappelle que c'est la reference de la valeur qui est dans texte qui à ete passer en paramettre à longeur.

EXO 5:
    Ca compile. rust permet à plus autre variable d'utiliser en meme temps la reference de la valeur dune variable principale. La variable princile demeur toujours le propriétaire, vu que les autre on juste une reference

EXO 6:
    La valeur de langage doit etre mutable parceque dans ce programme modifier souhaite avoir la reference de la valeur de langage pour pouvoir la modifier. Sans mut, modfier ne peux pas faire son operation.
    C'est pour permettre à modifier de pourvoir modifier langage.
    Non modifier ne devient pas propritaire
    Après l'appel langage contient rust est puissant

EXO 7:
    Les deux ne peuvent pas changer la valeur de s en eme temps. si apres le priemier let r1 = &mut s, on fait le  println!("{r1}"), puis apres le second let r2 = &mut s, on fait le println!("{r2}"), ça allait marché.

EXO 8:
    Non ca ne compile pas, parceque au moment où les deux premmier let voulant utiliser la valeur, le troisième let le modifie. donc il y a une sorte de conflit