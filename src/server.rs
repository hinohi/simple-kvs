use std::io::{BufRead, BufReader, BufWriter, Write};
use std::iter::Iterator;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use cmd::Cmd;
use db::DB;

struct SharedReceiver(Arc<Mutex<Receiver<Option<TcpStream>>>>);

impl Iterator for SharedReceiver {
    type Item = TcpStream;
    fn next(&mut self) -> Option<Self::Item> {
        let guard = self.0.lock().unwrap();
        match guard.recv() {
            Ok(Some(stream)) => Some(stream),
            _ => None,
        }
    }
}

impl Clone for SharedReceiver {
    fn clone(&self) -> Self {
        SharedReceiver(Arc::clone(&self.0))
    }
}

fn shared_channel() -> (Sender<Option<TcpStream>>, SharedReceiver) {
    let (sender, receiver) = channel();
    (sender, SharedReceiver(Arc::new(Mutex::new(receiver))))
}

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
            db: Arc::new(DB::new()),
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

fn worker(db: Arc<DB>, receiver: SharedReceiver) {
    for stream in receiver {
        client_handler(&db, stream);
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
        let cmd = match Cmd::from_string(buf) {
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
