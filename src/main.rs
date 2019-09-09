use std::fs::File;
use std::io::{Error, Write};

fn result_with() -> Result<(), Error> {
    let mut f = File::open("Foo.txt")?;

    f.write(b"a");

    Ok(())
}

fn main() {
    match result_with() {
        Ok(_) => println!("A"),
        Err(_) => println!("B"),
    }
}
