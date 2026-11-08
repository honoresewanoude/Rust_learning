use std::net::UdpSocket;

fn main(){
    let socket = UdpSocket::bind("127.0.0.1:7777").unwrap();
    let mut buffer = [0u8;1024];

    loop{
        let (taille,adresse)=socket.recv_from(&mut buffer).unwrap();
        socket.send_to(&buffer[..taille],adresse).unwrap();
    }

}