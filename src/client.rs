use std::io::{BufReader, BufWriter, Read, Write};
use std::net::TcpStream;

pub struct Client {
    addr: String,
}

impl Client {
    pub fn new(addr: &str) -> Self {
        Client {
            addr: String::from(addr),
        }
    }

    pub fn request(&self, cmd: &str) {
        let stream = TcpStream::connect(&self.addr).unwrap();
        let mut writer = BufWriter::new(&stream);
        let mut bytes = Vec::from(cmd);
        bytes.push('\n' as u8);
        writer.write(&mut bytes).unwrap();
        writer.flush().unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).unwrap();
        print!("{}", String::from_utf8(buf).unwrap());
    }
}
