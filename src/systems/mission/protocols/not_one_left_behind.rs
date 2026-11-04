use crate::systems::personnel::kernel::records::{Personnel, PersonnelId, PersonnelList};
// N.O.L.B PROTOCOL

//

pub enum Protocols {
    Accountability,
    NotOneLeftBehind,
}

//
pub enum AccountabilityParameter {

}

pub struct AccountabiltyProtocol {

}

//

pub fn sample_expected() -> PersonnelList {
    let mut expected = PersonnelList::new();
    for id in 1..=3 {
        expected.add(Personnel::new(PersonnelId::new(id)));
    }
    expected
}



