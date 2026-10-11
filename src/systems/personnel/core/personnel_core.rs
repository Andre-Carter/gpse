#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PersonnelId(pub u64);

// minimal representation of a person
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Personnel {
    id: PersonnelId,
}

impl Personnel {
    pub fn new() -> Self {
        Self {
            Personnel::new();
        }
    }
}

// a list of personnel
pub struct PersonnelList {
    personnel: Vec<Personnel>,
}

impl PersonnelList {
    pub fn new() -> Self {
        Self {
            personnel: Vec::new();
        }
    }

    pub fn add(&mut self, personnel: Personnel) {
        if !self.personnel.contains(&personnel) {
            self.personnel.push(personnel);
        }
    }

    pub fn contains(&self, id: PersonnelId) -> bool {
        self.personnel.iter().any(|person| person.id == id)
    }
}

impl Default for PersonnelList {
    fn default() -> Self {
        Self::new()
    }
}

pub fn personnel_list(personnel: Vec<Personnel>) {
    
}

// personnel record

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PersonnelName(pub String);

pub struct PersonnelRecord {
    id: PersonnelId,
    name: PersonnelName,
}

impl PersonnelRecord {
    pub fn new() Self -> {
        Self {

        }
    }
}

pub struct PersonnelRecords {
    personnel_records: Vec<PersonnelRecord>,
}

impl PersonnelRecords {
    pub fn new() Self -> {
        Self {
            personnel_records: Vec::new();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountabilityStatus {
    AccountedFor,
    Unknown,
    Discrepancy,
    Missing,
    ConfirmedDeceased,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvailabilityStatus {
    Available,
    OnMission,
    OnOrders,
    Tasked,
    Resting, 
    Libo,
}

pub struct AccountabilityRecord {
    pub personnel_id: PersonnelId,
    pub status: AccountabilityStatus,
    pub observed_at: Option<String>,
    pub evidence_reference: Option<String>,
}

pub struct PersonnelReport {
    personnel_id: PersonnelId,
    accountability_status: AccountabilityStatus,
    avalability_status: AvailabilityStatus,
    personnel_record: PersonnelRecord,
}

// test 
const person_1: Personnel = 556;
const person_2: Personnel = 223;
const mission_1_personnel: PersonnelLog = !vec[person_1, person_2];

// mock implentation of N.O.L.B
pub struct Protocol {
    protocol_id: ProtocolId,
    protocol_name: ProtocolName,
    protocol_description: ProtocolDescription,
}
pub struct NotOneLeftBehindProtocol {

}

//

