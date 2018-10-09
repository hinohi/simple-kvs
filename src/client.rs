use std::io::{BufRead, BufReader, BufWriter, Write};
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

    pub fn request(&self, cmd_list: Vec<String>) {
        let stream = TcpStream::connect(&self.addr).unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut writer = BufWriter::new(stream);
        for cmd in cmd_list {
            let mut bytes = Vec::from(cmd.clone());
            bytes.push('\n' as u8);
            writer.write(&mut bytes).unwrap();
            writer.flush().unwrap();
            let mut buf = String::new();
            reader.read_line(&mut buf).unwrap();
            if buf.is_empty() {
                println!("connection closed by peer");
                break;
            }
            print!(r#""{}" = {}"#, cmd, buf);
        }
    }
}
