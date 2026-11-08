use std::env;
use std::net::UdpSocket;
use std::time::Instant;

fn main(){
    let args: Vec<String>= env::args().collect();
    if args.len() < 2 {
        println!("usage : cargo run --<ip du server>");
        return;
        }

        let add = format!("{}:7777",args[1]);
        let socket = UdpSocket::bind("0.0.0.0:0").expect("impossble de crée le scoekt");

        let mut buffer = [0;1024];
        let mut rtts: Vec<f64> = Vec::new();



    for i in 0..5{
          let message  = format!("ping {}", i);
        let debut = Instant::now();
        socket.send_to(message.as_bytes(), &add).expect("erreur a l'envoi");
        println!("Sent '{}' to {}", message, add);

        let(n,src) = socket.recv_from(&mut buffer).expect("erreur à la reception");
        let rtt = debut.elapsed().as_secs_f64() * 1000.0;

        let reponse = String::from_utf8_lossy(&buffer[0..n]);
        println!("Receveid '{}' from {}, RTT {:.3} ms", reponse,src,rtt);

          rtts.push(rtt);

}

    let mut min  = rtts[0];
    let mut max = rtts[0];
    let mut somme = 0.0;

    for i in 0..rtts.len(){
        if rtts[i]<min{
            min = rtts[i];
    }
        if rtts[i] > max {
            max = rtts[i];
        }

        somme = somme  + rtts[i];
     }
    let moyennes  = somme / rtts.len() as f64;

println!();
println!("Min RTT {:.3} ms",min);
println!("Avg RTT {:.3} ms",moyennes);
println!("Max RTT {:.3} ms",max);

}


// ici : let mut buffer = [0;1024]; j'utilise un tableau fixe de de talle 1024 octets. si le serveur renvoie un message qui depasse cette taille , le reste du message sera perdu
// l'utilisation excessive de expect(), peut aussi provoquer un panic rust au nieavu du code, a la maoindre instabilité du réseaux . 