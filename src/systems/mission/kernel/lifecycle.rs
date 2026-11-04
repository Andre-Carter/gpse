pub struct MissionId {
    mission_name: String,
    id: String,
}

struct Mission<Objective, Order, ProtocolId> {
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