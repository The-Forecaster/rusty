use std::io::{stdout, BufWriter, Error};

fn main() -> Result<(), Error> {
    let message = String::from("Hello fellow Rustaceans!");

    ferris_says::say(message.as_bytes(), message.chars().count(), &mut BufWriter::new(stdout().lock()))?;

    Ok(())
}
