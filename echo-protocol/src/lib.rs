use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;

pub struct EchoProtocolConnection {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl EchoProtocolConnection {
    /// Constructs a new EchoProtocolConnection instance, handling the
    /// connection to the given address:port before returning the new
    /// structure.
    ///
    /// This function is intended to be used when writting the client.
    /// 
    /// # Return
    /// * Returns a new instance of EchoProtocolConnection within a io::Result enum. 
    /// * It will return Err() if the connection is not established correctly or
    /// if the TcpStream could not be created to initialize the reader buffer.
    /// 
    /// # Examples
    /// ```
    /// let mut connection = EchoProtocolConnection::new_and_connect_to("127.0.0.1:1234")
    ///     .expect("Could not create the echo-protocol connection");
    /// ```
    pub fn connect(address: &str) -> io::Result<Self> {
        let stream = match TcpStream::connect(address) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        Self::new(stream)
    }

    /// Constructs a new EchoProtocolConnection instance, initializing
    /// the internal reader and writer with the given stream.
    ///
    /// This function is intended to be used when writting the server.
    ///
    /// # Examples
    /// ```
    /// let mut connection = EchoProtocolConnection::new_with_stream(stream)
    ///     .expect("Could not create the echo-protocol connection");
    /// ```
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        let reader = BufReader::new(match stream.try_clone() {
            Ok(s) => s,
            Err(e) => return Err(e),
        });

        let writer = BufWriter::new(stream);
        
        Ok(Self { reader, writer })
    }

    /// Senda a message 
    /// 
    pub fn send_message(&mut self, message: &str) -> io::Result<()> {
        self.writer.write(&message.as_bytes())?;
        self.writer.write(&['\n' as u8])?;
        self.writer.flush()?;
        Ok(())
    }

    pub fn read_message(&mut self) -> io::Result<String> {
        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        line.pop();
        Ok(line)
    }
}
