#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PersonnelId { pub id: u64 }

impl PersonnelId {
    pub fn new(value: u64) -> Self {
            Self { id: value }
    }
}

// minimal representation of a person
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Personnel {
    id: PersonnelId,
}

impl Personnel {
    pub fn new(id: PersonnelId) -> Self {
        Self { id }
    }
}

// a list of personnel
#[derive(Debug, Default)]
pub struct PersonnelList {
    personnel: Vec<Personnel>,
}

impl PersonnelList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, personnel: Personnel) -> bool {
        if self.personnel.contains(&personnel) {
            return false;
        }
        self.personnel.push(personnel);
        true
    }

    pub fn contains(&self, id: PersonnelId) -> bool {
        self.personnel.iter().any(|person| person.id == id)
    }

    pub fn len(&self) -> usize {
        self.personnel.len()
    }

    pub fn is_empty(&self) -> bool {
        self.personnel.is_empty()
    }
}

// personnel record

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PersonnelName { 
    pub name: String,
}

impl PersonnelName {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into()
        }
    }
}

#[derive(Debug, Default)]
pub struct PersonnelRecord {
    id: PersonnelId,
    name: PersonnelName,
}

impl PersonnelRecord {
    pub fn new(id: PersonnelId, name: PersonnelName) -> Self {
        Self { id, name }
    }
}

#[derive(Debug, Default)]
pub struct PersonnelRecords {
    personnel_records: Vec<PersonnelRecord>,
}

impl PersonnelRecords {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, record: PersonnelRecord) -> bool {
        if self.personnel_records.iter().any(|r| r.id == record.id) {
            return false;
        }
        self.personnel_records.push(record);
        true
    }

    pub fn get(&self, id: PersonnelId) -> Option<&PersonnelRecord> {
        self.personnel_records.iter().find(|r| r.id == id)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_rejects_duplicates() {
        let mut list = PersonnelList::new();
        assert!(list.add(Personnel::new(PersonnelId::new(1))));
        assert!(!list.add(Personnel::new(PersonnelId::new(1))));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn records_lookup_by_id() {
        let mut records = PersonnelRecords::new();
        records.add(PersonnelRecord::new(
            PersonnelId::new(556),
            PersonnelName::new("Doe"),
        ));
        assert!(records.get(PersonnelId::new(556)).is_some());
        assert!(records.get(PersonnelId::new(223)).is_none());
    }
}
