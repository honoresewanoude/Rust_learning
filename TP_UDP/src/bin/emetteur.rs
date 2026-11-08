use std::io::{self, BufRead};
use std::net::UdpSocket;

fn main(){
    let socket = UdpSocket::bind("0.0.0.0").unwrap();
    println!("Ecris un message : ");

    for ligne in io::stdin().lock().lines(){
        let message = ligne.unwrap();
        socket.send_to(message.as_bytes(), "127.0.0.1:12345").unwrap();
    }
}