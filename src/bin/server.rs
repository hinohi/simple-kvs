extern crate simple_kvs;

use simple_kvs::Server;

fn run_server(addr: String) {
    let server = Server::new(&addr, 16);
    server.run_forever();
}

fn main() {
    use std::env;

    let args: Vec<String> = env::args().collect();
    let usage = r#"Usage:
    ./cmd addr"#;

    if args.len() <= 1 {
        println!("{}", usage);
        return;
    }
    let addr = args[1].to_string();
    run_server(addr);
}
