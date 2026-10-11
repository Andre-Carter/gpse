First experiment: Astroid mining
- objective
- orders
- protocols
- parameters

objectives contain orders, orders contain protocols, protocols have parameters.

The mission from beginning to end

Phase 1 — Mission proposal

Planning

MISSION receives a proposed asteroid target, resource objective, equipment package, budget, time window, and success criteria.

Example objective: obtain and characterize at least 5 kg of target material.

Phase 2 — Authorization

CONTROL

CONTROL evaluates whether the proposal meets the mission's requirements. Missing fuel estimates, inadequate power margins, unavailable equipment, or failed safety requirements could cause a denial or request for revision.

Approval means permission to proceed under specified conditions—not a guarantee of success.

Phase 3 — Transit and rendezvous

NAVIGATION

The spacecraft travels toward the target, estimates its state, and approaches at an approved relative speed. Telemetry reports propulsion, power, thermal, and navigation status.

Phase 4 — Survey and site selection

Reconnaissance

The probe maps candidate sites, estimates material composition, and checks surface conditions. MISSION evaluates the survey against the extraction objectives; CONTROL or the designated authorization process approves any action that requires a new permission.

Phase 5 — Extraction

Operations

The mining mechanism anchors, drills or collects loose material, and stores the sample. The system tracks energy use, collected mass, storage capacity, and equipment condition.

Phase 6 — Verification and disposition

Validation

GPSE verifies the measured quantity, sample integrity, and data receipt. MISSION determines whether the objectives are met, partially met, or unmet. The mission then follows its approved plan for returning, transferring, or retaining the sample.

Phase 7 — Closeout

Mission record

Record the outcome, resource totals, failures, deviations, remaining assets, and lessons learned. The final report should distinguish verified facts from estimates and unknowns.

// SECTION

These are different kinds of data. They should not be interchangeable.

An objective describes a desired outcome.

An order requests or directs an action.

A protocol defines mandatory rules and conditions governing actions.

An authorization establishes whether an action or mission is permitted.

A result records what actually happened.

// SECTION

What happens when the mission goes wrong?

A mission architecture isn't proven by the happy path alone. Let's test the design with three failures.

Scenario A — Power falls below the extraction threshold

TELEMETRY reports the condition. The applicable protocol prevents unsafe drilling. MISSION records that extraction is suspended, and CONTROL evaluates whether the revised operation remains authorized. COMMAND should not be able to bypass the safety requirement with an ordinary drill command.

Scenario B — Communication is lost

COMMUNICATION records the loss of contact; TELEMETRY and onboard checks continue where possible. The spacecraft follows a previously approved autonomous contingency plan. A missing acknowledgement is not proof that a command executed.

Scenario C — The drill jams after collecting 3 kg

The mission has not achieved its 5 kg target, but it has not necessarily failed completely. The system stops or isolates the faulty mechanism according to its procedures, records the 3 kg result, and evaluates safe recovery options. No system should falsely mark the objective complete.

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

The crucial rule: a mission being Authorized does not mean every possible order is automatically authorized. Some actions may need preconditions, resource checks, or additional approval.

- MISSION builds plans, CONTROL evalutes, COMMAND executes

MISSION: Build a plan to reach an asteroid, survey it, extract a sample, and return the required data.

CONTROL: Evaluate whether the plan satisfies its protocols, resources, constraints, and authorization requirements.

COMMAND: Execute approved orders by dispatching work to NAVIGATION, TELEMETRY, COMMUNICATION, and the other relevant systems.

One important safeguard: COMMAND should not be able to bypass CONTROL's required authorization. If an order fails a mandatory precondition during execution, the execution flow must stop, hold, or follow an approved contingency rather than blindly continuing.

And I'd keep the distinction between plan approval and execution success. CONTROL can approve a plan without guaranteeing the mission will succeed; COMMAND reports what actually happened.