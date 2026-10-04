fn main() {
   
   struct etudiant {
    name : String,
    prenom : String,
    age : i32,
    est_major : bool,
    note_maths : f64,
    note_info : f64,
   }

   struct Serveur {
    Nom : String,
    Ip : String,
    Ram : i32,
    Allume : bool,
   }

   impl Serveur {
    fn afficher(&self){
        println!("Nom : {}",self.Nom);
        println!("Ip : {}",self.Ip);
        println!("Ram : {}",self.Ram);
        if self.Allume == true {
            println!("Allumer")
        }else{
            println!("Eteint")
        }
    }

    fn demarrer(&mut self){
        if self.Allume  == false{
            self.Allume = true;
        }
    }

    fn est_suffisant(&self, other : i32) -> bool {
         self.Ram >= other 
    }

    fn nouveau (nom : String, ip : String, ram : i32) -> Serveur {
        Serveur {
            Nom : nom,
            Ip : ip,
            Ram : ram,
            Allume: false,
        }
    }

   }

   

   let mut etudiant1 = etudiant{
    name : String::from("Honoré"),
    prenom : String::from("Max"),
    age : 73,
    est_major : true,
    note_maths : 15.4,
    note_info : 18.34,
   };

   println!("Je m'appelle {} {}, agé de {}. J'ai {} en maths et {} en info, {}"
            , etudiant1.name, etudiant1.prenom, etudiant1.age, etudiant1.note_maths, etudiant1.note_info, 
            if etudiant1.est_major  {
                "et je suis major de ma promo"
                    } else {
                "mais je ne suis pas major de ma promo"
            });

   etudiant1.name = String::from("Marc-Aurel");
   etudiant1.prenom = String::from("KPK");
   etudiant1.est_major = false;

   println!("Je m'appelle {} {}, agé de {}. J'ai {} en maths et {} en info, {}"
            , etudiant1.name, etudiant1.prenom, etudiant1.age, etudiant1.note_maths, etudiant1.note_info, 
            if etudiant1.est_major  {
                "et je suis major de ma promo"
                    } else {
                "mais je ne suis pas major de ma promo"
            });

    
    let mut Server = Serveur::nouveau(
        String::from("serveur-web"),
        String::from("192.168.1.10"),
        16,
    );

    Server.afficher();

    Server.demarrer();

    Server.afficher();

    println!("RAM suffisante : {}", Server.est_suffisant(8));

}
