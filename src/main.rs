pub struct Customer {
    name: String,
    age: u16
}

impl Customer {
    fn new(name: String, age: u16) -> Customer {
        Customer { name, age }
    }
}

trait Serve {
    fn serve(&self);
}

impl Serve for Customer {
    fn serve(&self) {
        if self.age > 21 {
            println!("What would you like? ");
            return;
        }
        println!("We can't serve you, {}", self.name);
    }
}

fn main() {
    let steve = Customer::new("Steve".to_string(), 16);

    steve.serve()
}