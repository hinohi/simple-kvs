mod client;
mod cmd;
mod db;
mod server;

use client::Client;
use server::Server;

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
        let client = Client::new(&args[2]);
        client.request(&args[3]);
    } else {
        println!("{}", usage);
    }
}
