fn main() {
   
   struct etudiant {
    name : String,
    prenom : String,
    age : i32,
    est_major : bool,
    note_maths : f64,
    note_info : f64,
   }

   let mut etudiant1 = etudiant{
    name : String::from("Honoré"),
    prenom : String::from("Max"),
    age : 73,
    est_major : true,
    note_maths : 15.4,
    note_info : 18.34,
   };

   let major = if etudiant1.est_major  {
    "et je suis major de ma promo"
   } else {
    "mais je ne suis pas major de ma promo"
   };

   println!("Je m'appelle {} {}, agé de {}. J'ai {} en maths et {} en info, {}"
            , etudiant1.name, etudiant1.prenom, etudiant1.age, etudiant1.note_maths, etudiant1.note_info, major);

   etudiant1.name = String::from("Marc-Aurel");
   etudiant1.prenom = String::from("KPK");
   etudiant1.est_major = false;

   println!("Je m'appelle {} {}, agé de {}. J'ai {} en maths et {} en info, {}"
            , etudiant1.name, etudiant1.prenom, etudiant1.age, etudiant1.note_maths, etudiant1.note_info, major);

   


}
