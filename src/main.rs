use std::{net::{SocketAddrV4, Ipv4Addr, UdpSocket}, io::{stdin, Error}};

fn main() -> Result<(), Error> {
    let scanner = stdin();

    let a = 0;
    let b = 0;
    let c = 0;
    let d = 0;

    println!("Enter in the first octet of the target here");
    scanner.read_line(&mut a.to_string()).unwrap();
    println!("Second octet");
    scanner.read_line(&mut b.to_string()).unwrap();
    println!("Third...");
    scanner.read_line(&mut c.to_string()).unwrap();
    println!("Final octet");
    scanner.read_line(&mut d.to_string()).unwrap();

    let connection = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::new(a, b, c, d), 0)).ok().unwrap();

    println!("Checking ports on the target: ");
    for i in 1..65500 {
        if !connection.local_addr().is_err() {
            connection.local_addr().unwrap().set_port(i);

            connection.send(&[0, 0, 0, 0]).and_then(|_| { Ok(println!("port {} is open", i)) }).expect("");
        }
    }

    Ok(())
}