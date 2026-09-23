use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

//#[derive(Default)]
pub struct Ledger {
    pub entries: Vec<String>,
}

impl Ledger {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, entry: String) {
        self.entries.push(entry);
    }
}

impl Default for Ledger {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

pub fn open_ledger() {
    let mut ledger = Ledger::new();

    ledger.add(String::from("GPSE milestone."));

    loop {
        let mut command = String::new();

        print!("Ledger> ");

        read(&mut command);

        let command = command.trim();

        //list, read <index>, search <term>, delete <index>, clear, save, load, help, exit

        match command {
            "entry" => {
                print!("Ledger Entry> ");

                let mut note: String = String::new();

                read(&mut note);

                if note.is_empty() {
                    println!("Ledger Entry Invalid.");
                    continue;
                }

                let note = note.trim().to_string();

                ledger.add(note.clone());
            }
            //EXIT LEDGER
            "exit" => {
                //println!("Ledger closed.");
                break;
            }
            //EMPTY COMMAND
            _ => {
                println!("Unknown ledger command.");
            } //END OF LEDGER COMMANDS
        }
        //END OF MATCH
    }
    //END OF LOOP
}
//END OF OPEN LEDGER
