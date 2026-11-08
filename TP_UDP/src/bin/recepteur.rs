use std::net::UdpSocket;

fn main(){
    let socket = UdpSocket::bind("127.0.0.1:12345").unwrap();
    println!("Récepteur en attente de message");

    let mut buffer = [0u8;1024];
    loop{
        let (taille,adresse) = socket.recv_from(&mut buffer).unwrap();
        let message = String::from_utf8_lossy(&buffer[..taille]);
        println!("Message recu de {} : {}",adresse,message);
    }
}