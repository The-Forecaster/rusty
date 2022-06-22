use std::{net::{SocketAddrV4, Ipv4Addr}, thread, io::stdin};

fn main() {
    let scanner = stdin();

    let a = 0;
    let b = 0;
    let c = 0;
    let d = 0;

    stdin().read_line(&mut a.to_string()).unwrap();
    stdin().read_line(&mut b.to_string()).unwrap();
    stdin().read_line(&mut c.to_string()).unwrap();
    stdin().read_line(&mut d.to_string()).unwrap();

    let main = thread::spawn(move || {

        for i in 1..65500 {
            spam(a, b, c, d, i)
        }
    });
}

fn spam(a: u8, b: u8, c: u8, d: u8, port: u16) {
    let sock = SocketAddrV4::new(Ipv4Addr::new(a, b, c, d), port);
}