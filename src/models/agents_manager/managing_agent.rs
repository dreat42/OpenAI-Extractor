use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};

use crate::models::agents::agent_traits::{FactSheet, SpecialFunctions};

pub struct ManagingAgent {
    attributes: BasicAgent,
    factsheet: FactSheet,

    agents: Vec<Box<dyn SpecialFunctions>>,
}
