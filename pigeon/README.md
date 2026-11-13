Pigeon — first prototype
Single file
 Local filesystem


Our initial lifecycle:
1. Read — capture user input.
2. Package — turn input into a message.
3. Send — pass the message to the local inbox.
4. Receive — retrieve the message.
5. Save — persist it.
6. Read — display saved messages.
7. Delete — remove saved messages.
Complete src/main.rs

use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write, stdin, stdout};

// PIGEON STORAGE
const INBOX: &str = "pigeon_data/inbox.txt";
const ARCHIVE: &str = "pigeon_data/messages.txt";

// MESSAGE
struct Message {
    content: String,
}

// INPUT
fn input() -> io::Result<String> {
    let mut value = String::new();

    stdout().flush()?;
    stdin().read_line(&mut value)?;

    Ok(value.trim().to_string())
}

// PACKAGE
fn package(content: String) -> Message {
    Message { content }
}

// SEND
fn send(message: Message) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(INBOX)?;

    writeln!(file, "{}", message.content)?;

    println!("message sent");

    Ok(())
}

// RECEIVE
fn receive() -> io::Result<()> {
    let content = fs::read_to_string(INBOX)?;

    if content.is_empty() {
        println!("inbox empty");
        return Ok(());
    }

    let mut archive = OpenOptions::new()
        .create(true)
        .append(true)
        .open(ARCHIVE)?;

    archive.write_all(content.as_bytes())?;

    fs::write(INBOX, "")?;

    println!("message received and saved");

    Ok(())
}

// READ
fn read() -> io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(ARCHIVE)?;

    let mut content = String::new();

    file.read_to_string(&mut content)?;

    if content.is_empty() {
        println!("no saved messages");
    } else {
        println!("\n{content}");
    }

    Ok(())
}

// DELETE
fn delete() -> io::Result<()> {
    fs::write(ARCHIVE, "")?;

    println!("messages deleted");

    Ok(())
}

// MAIN
fn main() -> io::Result<()> {
    fs::create_dir_all("pigeon_data")?;

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(INBOX)?;

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(ARCHIVE)?;

    println!("WELCOME TO PIGEON");

    loop {
        print!("\npigeon> ");

        match input()?.as_str() {
            "send" => {
                print!("message> ");
                let content = input()?;

                let message = package(content);
                send(message)?;
            }

            "receive" => receive()?,

            "read" => read()?,

            "delete" => delete()?,

            "<" => {
                println!("exiting pigeon");
                break;
            }

            _ => println!("unknown command"),
        }
    }

    Ok(())
}



How the data flows
USER INPUT

Raw text


PACKAGE

Message struct


SEND

Local inbox


RECEIVE → SAVE

Local archive


READ

Display archive


DELETE

Clear archive




Resulting filesystem
pigeon/
├── Cargo.toml
├── src/
│   └── main.rs
└── pigeon_data/
    ├── inbox.txt
    └── messages.txt


Test it
cargo run


Try:
WELCOME TO PIGEON

pigeon> send
message> Hello from Pigeon!
message sent

pigeon> receive
message received and saved

pigeon> read

Hello from Pigeon!

pigeon> delete
messages deleted

pigeon> <
exiting pigeon


A few deliberate limitations:
- Sending and receiving currently happen locally, not over a network.
- Messages are plain text, one per line.
- Delete clears the archive but leaves the inbox intact.
- There is no message identification or encryption yet.
One architectural decision
I'd keep these seven operations as our initial Pigeon vocabulary:

input → package → send → receive → save → read → delete

Notice that save is currently part of receive. We can separate it later if the architecture calls for it.