use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};

use cmd::Cmd;
use db::DB;

pub struct Server {
    addr: String,
    db: DB,
}

impl Server {
    pub fn new(addr: &str) -> Server {
        Server {
            addr: String::from(addr),
            db: DB::new(),
        }
    }

    pub fn run_forever(&self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((stream, addr)) => {
                    println!("new client: {}", addr);
                    self.client_handler(stream);
                }
                Err(e) => println!("accept error: {}", e),
            }
        }
    }

    fn client_handler(&self, stream: TcpStream) {
        let mut reader = BufReader::new(&stream);
        let mut buf = String::new();
        if let Err(e) = reader.read_line(&mut buf) {
            println!("{}", e);
            return;
        }
        let mut writer = BufWriter::new(stream.try_clone().unwrap());
        let cmd = match Cmd::from_string(buf) {
            Ok(c) => c,
            Err(e) => {
                let mut msg = e + "\n";
                writer.write(&mut msg.as_bytes()).unwrap();
                writer.flush().unwrap();
                return;
            }
        };
        let ret;
        if let Some(v) = self.db.execute(cmd) {
            ret = format!("{}\n", v);
        } else {
            ret = "\n".to_string();
        }
        writer.write(&mut ret.as_bytes()).unwrap();
        writer.flush().unwrap();
    }
}
