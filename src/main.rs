mod cmd;
mod db;

use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::{TcpListener, TcpStream};

use cmd::Cmd;
use db::DB;

struct Server {
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
                writer.write(&mut e.as_bytes()).unwrap();
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

fn client(addr: &str, cmd: &str) {
    let stream = TcpStream::connect(addr).unwrap();
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

fn main() {
    use std::env;

    let args: Vec<String> = env::args().collect();
    let usage = r#"Usage:
    ./cmd server addr
    ./cmd client addr "cmd string""#;

    if args.len() < 3 {
        println!("{}", usage);
        return;
    }
    if args[1] == "server" {
        let server = Server::new(&args[2]);
        server.run_forever();
    } else if args[1] == "client" && args.len() >= 4 {
        client(&args[2], &args[3]);
    } else {
        println!("{}", usage);
    }
}
