struct Mission {
    id: MissionId,
    objectives: Vec<Objective>,
    orders: Vec<Order>,
    protocols: Vec<ProtocolId>,
    status: MissionStatus,
}

enum MissionStatus {
    Proposed,
    UnderReview,
    Authorized,
    Active,
    Suspended,
    Completed,
    Aborted,
}

pub struct MissionLog {

}