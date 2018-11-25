extern crate simple_kvs;

use simple_kvs::Client;

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
    ./cmd addr "cmd string""#;

    if args.len() <= 2 {
        println!("{}", usage);
        return;
    }
    let addr = args[1].to_string();
    if let Some(cmd) = args.get(2) {
        run_client(addr, Some(cmd.to_string()));
    } else {
        run_client(addr, None);
    }
}
