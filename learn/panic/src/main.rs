use std::fs::File;
use std::net::IpAddr;
use std::io::ErrorKind;

fn main() {
    let file = File::open("hello.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("../hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    let ip: IpAddr = "127.0.0.1".parse().unwrap();
    println!("ip: {:?}", ip);
}
