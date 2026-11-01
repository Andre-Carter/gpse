use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Comm {
    pub to: String,
    pub from: String,
    pub message: String,
    pub sent_at: SystemTime,
}

#[derive(Debug, Default)]
pub struct Inbox {
    comms: Vec<Comm>,
}

impl Inbox {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, comm: Comm) {
        self.comms.push(comm);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Comm> {
        self.comms.iter()
    }
}
