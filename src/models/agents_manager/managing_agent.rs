use crate::models::agent_basic;
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};

use crate::models::agents::agent_traits::{FactSheet, SpecialFunctions};

use crate::ai_functions::managing::print_user_input_to_goal;
use crate::base::general::ai_task_request;

use crate::models::agents::agent_architect::AgentSolutionArchitect;

use crate::models::agents::agent_backend::AgentBackendDeveloper;


use crate::models::main::llm::Message;

#[derive(Debug)]

pub struct ManagingAgent {
    attributes: BasicAgent,
    factsheet: FactSheet,

    agents: Vec<Box<dyn SpecialFunctions>>,
}

impl ManagingAgent {
    pub async fn new(usr_req: String) -> Result<Self, Box<dyn std::error::Error>> {
        let position: String = "Project Manager".to_string();
        let attributes: BasicAgent = BasicAgent {
            objective: "Managing agents who are building an execellant website for the user"
                .to_string(),
            position: position.clone(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        let project_description: String = ai_task_request(
            usr_req,
            &position,
            get_function_string!(print_user_input_to_goal),
            print_user_input_to_goal,
        )
        .await;

        let agents: Vec<Box<dyn SpecialFunctions>> = vec![];

        let factsheet: FactSheet = FactSheet {
            project_description,
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,
        };

        Ok(Self {
            attributes,
            factsheet,
            agents,
        })
    }

    fn add_agents(&mut self, agents: Box<dyn SpecialFunctions>) {
        self.agents.push(agents);
    }

    fn create_agents(&mut self) {
        self.add_agents(Box::new(AgentSolutionArchitect::new()));
       self.add_agents(Box::new(AgentBackendDeveloper::new()));
  
    }

    pub async fn execlude_project(&mut self) {
        self.create_agents();
        for agent in &mut self.agents {
            let agent_res: Result<(), Box<dyn std::error::Error>> =
                agent.execute(&mut self.factsheet).await;

            // let agent_info: &BasicAgent = agent.get_attributes_from_agent();

            // dbg!(agent_info);
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[tokio::test]

    async fn tests_managing_agent() {
        let usr_request:&str = "need a full stack app that fetches and tracks my fitness progress. Need to include timezone into from the web.";

        let mut managing_agent: ManagingAgent = ManagingAgent::new(usr_request.to_string())
            .await
            .expect("Error creating Managing");

        managing_agent.execlude_project().await;
        dbg!(managing_agent.factsheet);
    }
}
