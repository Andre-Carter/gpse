

pub const _CIRCA: f64 = 100.0;

// LOGIC GATE

use std::io::{Write, stdin, stdout};

pub const TRUE: bool = true;
pub const FALSE: bool = false;

fn read(cli_input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(cli_input).expect("failed to read");
}

pub fn logic_cli() {
    loop {
        let mut cli_input = String::new();

        print!("Enter switch state (on/off): ");

        read(&mut cli_input);

        let logic_input = cli_input.trim();

        let state = match logic_input {
            "on" => Switch::ON,
            "off" => Switch::OFF,
            "switch" => Switch::ON.switch(),
            "exit" => break,
            _ => {
                println!("Invalid switch command.");
                return;
            }
        };

        println!("Switch state: {}", state.state());
    }
        
    //test_switch();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Switch {
    pub logic: bool,
}

impl Switch {
    pub const ON: Self = Self { logic: TRUE };
    pub const OFF: Self = Self { logic: FALSE };

    pub fn new(self) -> Self {
        Self {
            logic: self.logic,
        }
    }

    // SWITCH ON
    pub fn on(self) -> Self {
        Self { logic: TRUE }
    }

    // SWITCH OFF
    pub fn off(self) -> Self {
        Self { logic: FALSE }
    }

    pub fn switch(self) -> Self {
        Self {
            logic: !self.logic,
        }
    }

    pub fn state(self) -> &'static str {
        match self.logic {
            TRUE => "ON",
            FALSE => "OFF",
        }
    }  
}

pub fn test_switch() {
    let state = Switch::ON;
    let next_state = state.switch();

    assert_eq!(next_state, Switch::OFF);
    assert_eq!(next_state.switch(), Switch::ON);

    println!("{}", Switch::ON.state());
    println!("{}", Switch::OFF.state());
}