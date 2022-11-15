use std::{io::stdin, io::Error};

pub fn main() -> Result<(), Error> {
    let mut database = Database::new();

    print!("Enter your username: ");

    let mut user = String::new();
    let mut pass = String::new();

    match stdin().read_line(&mut user) {
        Ok(_) => (),
        Err(_) => panic!()
    }

    match stdin().read_line(&mut pass) {
        Ok(_) => (),
        Err(_) => panic!()
    }

    database.add_user(&user, &pass);

    database.print_data();

    Ok(())
}

struct Database {
    table: Vec<User>,
    counter: u128
}

impl Database {
    fn new() -> Self {
        Self { table: vec![], counter: 0 }
    }

    fn add_user(&mut self, username: &str, password: &str) {
        self.table.push(User::new(username, password, &self.counter));
        self.counter += 1;
    }

    fn print_data(&self) {
        for user in &self.table {
            user.print_data()
        }
    }
}

struct User {
    username: String,
    password: String,
    id: u128
}

impl User {
    fn new(username: &str, password: &str, id: &u128) -> Self {
        Self { username: username.to_string(), password: password.to_string(), id: *id}
    }

    fn print_data(&self) {
        println!("{} {} {}", self.username, self.password, self.id)
    }
}
