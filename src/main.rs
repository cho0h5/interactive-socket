use std::net::TcpStream;
use std::io::prelude::*;
use std::env;
use std::str;

fn main() {
    let mode = env::args().nth(1).unwrap();

    match mode.as_str() {
        "server" => server(),
        "client" => {
            let addr = env::args().nth(2).unwrap();
            client(&addr)
        },
        _ => panic!("invalid mode"),
    }
}

fn client(addr: &str) {
    println!("connect to {}", addr);


    let mut stream = TcpStream::connect(addr).unwrap();
    stream.write("hello server".as_bytes()).unwrap();
    let mut buffer = vec![];
    stream.read_to_end(&mut buffer).unwrap();
    println!("read: {:?}", buffer);
}

fn server() {
    println!("start server");
}
