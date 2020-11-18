use std::io;
use std::net::{TcpListener, TcpStream};
use std::thread;

use echo_protocol::EchoProtocolConnection;

fn handle_client(stream: TcpStream) -> io::Result<()> {
    println!("Incomming connection from: {:?}", stream);

    let address = stream.peer_addr().unwrap().clone();

    let mut connection =
        EchoProtocolConnection::new(stream).expect("Could not create the echo-protocol connection");

    loop {
        let message = connection.read_message()?;
        println!("[{:?}] : {}", &address, message);
        connection.send_message(&message)?;
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2804").expect("Could not bind.");
    println!("Listening to {:?}", listener);
    for stream in listener.incoming() {
        match stream {
            Ok(strm) => {
                thread::spawn(move || handle_client(strm).map_err(|e| eprintln!("Error {}", e)));
            }
            Err(_) => {}
        }
    }
}
