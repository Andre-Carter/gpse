
use std::fs::File;
use std::io::{self, Read, Write, stdin, stdout};

fn read(input: &mut String) -> io::Result<()> {
    stdout().flush()?;
    stdin().read_line(input)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let mut input = String::new();

    print!("pigeon> ");
    read(&mut input)?;

    let command = input.trim();

    match command {
        "write" => {
            fn write_file() -> io::Result<()> {
    let mut file = File::create("pig.txt")?;
    file.write_all(b"I love Rust!")?;
    Ok(())
}
        }

        "read" => {
            fn read_file() -> io::Result<()> {
    let mut file = File::open("pig.txt")?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    println!("{content}");

    Ok(())
}
        }

        "<" => {
            println!("exiting pigeon");
        }

        _ => println!("unknown command"),
    }

    Ok(())
}




