use crate::ai_functions::architech::{print_project_scope, print_site_urls};

use crate::base::command_line::PrintCommand;

use crate::base::general::{ai_task_request_decoded , check_status_code};

use crate::models::agent_basic::basic_agent::BasicAgent;
use crate::models::agent_basic::basic_trait::BasicTraits;

use crate::models::agents::agent_traits::{FactSheet,ProjectScope,SpecialFunction};
use crate::models::agent_basic::basic_agent::AgentState;

use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;

pub struct AgentSolutionArchitect {

attributes:BasicAgent
}
// #[derive(Debug)]
impl AgentSolutionArchitect {
pub fn new()->Self{
    let attributes:BasicAgent= BasicAgent{
        objective:"Gathers information and design solutions for website development".to_string(),
        position:"Solution Architect".to_string(),
        state:AgentState::Discovery,
        memory:vec![]
    };
    Self{attributes}
}

async fn call_project_scope(&mut self,factsheet:&mut FactSheet) -> ProjectScope{
    let msg_context:String = format!("{}",factsheet.project_description);

    let ai_response:ProjectScope = ai_task_request_decoded::<ProjectScope>(
        msg_context,
        &self.attributes.position,
        get_function_string!(print_project_scope),
        print_project_scope,
    
    ).await;

    
    factsheet.project_scope=Some(ai_response.clone());
    self.attributes.update_state(AgentState::Finished);
    return ai_response;
    
    }

}
