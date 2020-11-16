use std::io::{self, Write};
use echo_protocol::EchoProtocolConnection;

fn main() -> io::Result<()> {
    let mut connection = EchoProtocolConnection::new_and_connect_to("127.0.0.1:2804")
        .expect("Could not create the echo-protocol connection");

    loop {
        // Read something from the standard input
        print!("[Client] : ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");

        // The protocol will add the new line character before sending
        // the message so we pop it from the input.
        input.pop(); 
                     

        // The echo protocol connection will manage the transmition. No need 
        // to perform anything else.
        connection.send_message(input.as_str())?;

        // Wait and collect the server response.
        let message = connection.read_message()?;
        println!("[Server] : {}", message);
    }
}
