use crate::ai_functions::backend::{
    self, print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
    print_rest_api_endpoints,
};
use crate::base::command_line::{confirm_safe_code, PrintCommand};
use crate::base::general::{
    ai_task_request, ai_task_request_decoded, check_status_code, read_code_template_contents,
    read_exec_main_contents, save_api_endpoints, save_backend_code, WEB_SERVER_PROJECT_PATH,
};
use crate::models::agent_basic::basic_agent::AgentState;
use crate::models::agent_basic::basic_agent::BasicAgent;
use crate::models::agent_basic::basic_trait::BasicTraits;
use crate::models::agents::agent_traits::{FactSheet, ProjectScope, SpecialFunctions};

use async_trait::async_trait;
use crossterm::execute;
use crossterm::style::Attributes;
use reqwest::Client;
use std::fmt::format;
use std::fs;
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time;

use super::agent_traits::RouteObject;

#[derive(Debug)]

pub struct AgentBackendDeveloper {
    attributes: BasicAgent,
    bug_errors: Option<String>,
    bug_count: u8,
}

impl AgentBackendDeveloper {
    pub fn new() -> Self {
        let attributes: BasicAgent = BasicAgent {
            objective: "Develops backend code for webserver and json database".to_string(),
            position: "Backend Developer".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        Self {
            attributes,
            bug_errors: None,
            bug_count: 0,
        }
    }

    async fn call_initial_backend_code(&mut self, factsheet: &mut FactSheet) {
        let code_template_str: String = read_code_template_contents();

        let msg_context: String = format!(
            "CODE TEMPLATE: {:?} \n PROJECT_DESCRIPTION {:?} \n",
            code_template_str, factsheet.project_description
        );

        let ai_response: String = ai_task_request_decoded(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_backend_webserver_code),
            print_backend_webserver_code,
        )
        .await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    async fn call_improved_backend_code(&mut self, factsheet: &mut FactSheet) {
        let code_template_str: String = read_code_template_contents();

        let msg_context: String = format!(
            "CODE TEMPLATE: {:?} \n PROJECT_DESCRIPTION {:?} \n",
            code_template_str, factsheet.project_description
        );

        let ai_response: String = ai_task_request_decoded(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_improve_webserver_code),
            print_improved_webserver_code,
        )
        .await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    async fn call_fix_code_bugs(&mut self, factsheet: &mut FactSheet) {
        let msg_context: String = format!(
            "BACKEND_CODE: {:?} \n ERROR_BUGS {:?} \n
        THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE.",
            factsheet.backend_code, self.bug_errors
        );

        let ai_response: String = ai_task_request_decoded(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_fixed_code),
            print_fixed_code,
        )
        .await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    async fn call_extract_external_api_endpoints(&self) -> String {
        let backend_code: String = read_exec_main_contents();
        let msg_context: String = format!("CODE_INPUT: {:?}", backend_code);

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_rest_api_endpoints),
            print_rest_api_endpoints,
        )
        .await;

        ai_response
    }
}

#[async_trait]
impl SpecialFunctions for AgentBackendDeveloper {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        &self.attributes
    }

    async fn execute(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while self.attributes.state != AgentState::Finished {
            match &self.attributes.state {
                AgentState::Discovery => {
                    self.call_initial_backend_code(factsheet).await;
                    self.attributes.state = AgentState::Working;
                }
                AgentState::Working => {
                    if self.bug_count == 0 {
                        self.call_improved_backend_code(factsheet).await;
                        self.attributes.state = AgentState::UnitTesting;
                        continue;
                    } else {
                        self.call_fix_code_bugs(factsheet).await;
                        self.attributes.state = AgentState::UnitTesting;
                        continue;
                    }
                }
                AgentState::UnitTesting => {
                    PrintCommand::UnitTest.print_agend_message(
                        &self.attributes.position.as_str(),
                        "Backend Code Unit Testing Requesting user input",
                    );

                    let is_safe_code = confirm_safe_code();

                    if !is_safe_code {
                        self.attributes.state = AgentState::Finished;
                    }

                    PrintCommand::UnitTest.print_agend_message(
                        &self.attributes.position.as_str(),
                        "Buildind Code Unit Testing : building project...",
                    );

                    let build_backend_server: std::process::Output = Command::new("cargo")
                        .arg("build")
                        .current_dir(WEB_SERVER_PROJECT_PATH)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .output()
                        .expect("Failed to run backend application");

                    if build_backend_server.status.success() {
                        self.bug_count = 0;
                        PrintCommand::UnitTest.print_agend_message(
                            self.attributes.position.as_str(),
                            "Backend Code Unit Testing: Test server build successful...",
                        );
                    } else {
                        let error_err: Vec<u8> = build_backend_server.stderr;
                        let error_str: String = String::from_utf8(error_err).unwrap();

                        self.bug_count += 1;
                        self.bug_errors = Some(error_str);

                        if self.bug_count > 2 {
                            PrintCommand::Issue.print_agend_message(
                                self.attributes.position.as_str(),
                                "Backend Code Unit Testing: Too many bugs find in code...",
                            );

                            panic!("Error: Too many bugs")
                        }

                        self.attributes.state = AgentState::Working;
                        continue;
                    }

                    let api_endpoints_str: String =
                        self.call_extract_external_api_endpoints().await;

                    let api_endpoints: Vec<RouteObject> =
                        serde_json::from_str(api_endpoints_str.as_str())
                            .expect("Faled to decode API Endpoints");

                    let check_endpoints: Vec<RouteObject> = api_endpoints
                        .iter()
                        .filter(|&route_object| {
                            route_object.method == "get" && route_object.is_route_dynamic == "false"
                        })
                        .cloned()
                        .collect();

                    factsheet.api_endpoint_schema = Some(check_endpoints.clone());

                    PrintCommand::UnitTest.print_agend_message(
                        self.attributes.position.as_str(),
                        "Build Code Unit Testing: Starting Web Server....",
                    );

                    let run_backend_server: std::process::Output = Command::new("cargo")
                        .arg("run")
                        .current_dir(WEB_SERVER_PROJECT_PATH)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .output()
                        .expect("Failed to run backend application");

                    PrintCommand::UnitTest.print_agend_message(
                        self.attributes.position.as_str(),
                        "Build Code Unit Testing: Launching tests on web server in 5 seconds....",
                    );

                    let seconds_sleep: Duration = Duration::from_secs(5);
                    time::sleep(seconds_sleep).await;

                    self.attributes.state = AgentState::Finished;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_writing_backend_code() {
        let mut agent: AgentBackendDeveloper = AgentBackendDeveloper::new();

        let factsheet_str: &str = r#"{
    "project_description": "build a website that fetches and tracks fitness with timezone information",
    "project_scope": {
        "is_crud_required": true,
        "is_user_login_and_logout": true,
        "is_external_urls_required": true
    },
    "external_urls": [
        "http://worldtimeapi.org/api/timezone"
    ],
    "backend_code": null,
    "api_endpoint_schema": null
}
"#;

        let mut factsheet: FactSheet = serde_json::from_str(factsheet_str).unwrap();

        agent
            .execute(&mut factsheet)
            .await
            .expect("Failed to execute Backend Developer agent");
    }
}
