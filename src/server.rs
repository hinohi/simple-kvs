use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;
use std::sync::Arc;
use std::thread;

use shared_channel::{shared_channel, SharedReceiver};

use cmd::Cmd;
use db::DB;

pub struct Server {
    addr: String,
    num: usize,
    db: Arc<DB>,
}

impl Server {
    pub fn new(addr: &str, num: usize) -> Server {
        Server {
            addr: String::from(addr),
            num,
            db: Arc::new(DB::new(16)),
        }
    }

    pub fn run_forever(&self) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        let (sender, receiver) = shared_channel();
        for _ in 0..self.num {
            let db = Arc::clone(&self.db);
            let rcv = receiver.clone();
            thread::spawn(move || worker(db, rcv));
        }
        loop {
            match listener.accept() {
                Ok((stream, addr)) => {
                    println!("new client: {}", addr);
                    sender.send(Some(stream)).unwrap();
                }
                Err(e) => println!("accept error: {}", e),
            }
        }
    }
}

fn worker(db: Arc<DB>, receiver: SharedReceiver<Option<TcpStream>>) {
    for i in receiver.iter() {
        if let Some(stream) = i {
            client_handler(&db, stream);
        } else {
            break;
        }
    }
}

fn client_handler(db: &DB, stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = BufWriter::new(stream);
    loop {
        let mut buf = String::new();
        if let Err(e) = reader.read_line(&mut buf) {
            println!("{}", e);
            break;
        }
        if buf.is_empty() {
            break;
        }
        let cmd = match Cmd::from_str(&buf) {
            Ok(c) => c,
            Err(e) => {
                let mut msg = e + "\n";
                writer.write(&mut msg.as_bytes()).unwrap();
                writer.flush().unwrap();
                break;
            }
        };
        let ret;
        if let Some(v) = db.execute(cmd) {
            ret = format!("{}\n", v);
        } else {
            ret = "\n".to_string();
        }
        writer.write(&mut ret.as_bytes()).unwrap();
        writer.flush().unwrap();
    }
}
