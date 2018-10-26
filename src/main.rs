mod client;
mod cmd;
mod db;
mod server;

use client::Client;
use server::Server;

fn run_server(addr: String) {
    let server = Server::new(&addr, 16);
    server.run_forever();
}

fn run_client(addr: String, cmd: Option<String>) {
    use std::io::{stdin, BufRead, BufReader};
    let client = Client::new(&addr);

    if let Some(cmd) = cmd {
        client.request(vec![cmd]);
        return;
    }
    let cin = BufReader::new(stdin());
    let cmd_list = cin.lines().map(|x| x.unwrap()).collect();
    client.request(cmd_list);
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
    let addr = args[2].to_string();
    if args[1] == "server" {
        run_server(addr);
    } else if args[1] == "client" {
        if let Some(cmd) = args.get(3) {
            run_client(addr, Some(cmd.to_string()));
        } else {
            run_client(addr, None);
        }
    } else {
        println!("{}", usage);
    }
}
