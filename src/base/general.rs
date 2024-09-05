use crate::base::command_line::PrintCommand;
use crate::{models::main::llm::Message, requests::call_request::call_gpt};
use serde::de::DeserializeOwned;
use reqwest::Client;
use std::fs;

const CODE_TEMPLATE_PATH:&str = "D:/code/rust_autogpt/web_template/src/code_template.rs";
const EXEC_MAIN_PATH:&str ="D:/code/rust_autogpt/web_template/src/main.rs";
const API_SCHEMA_PATH:&str ="D:/code/rust_autogpt/schemas/api_schema.json";


pub fn external_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str = ai_func(func_input);

    let msg: String = format!(
        "Function:{} 
    INSTRUCTION: You are a function printer. You only print the result function.
    Nothing else. NO commentary. Here is the input to the fucntion:{}.
    Print out what the function will return",
        ai_function_str, func_input
    );

    // dbg!(msg);

    Message {
        role: "system".to_string(),
        content: msg,
    }
}

pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    let extended_msg: Message = external_ai_function(function_pass, &msg_context);

    PrintCommand::AICall.print_agend_message(agent_position, agent_operation);

    let llm_response_res: Result<String, Box<dyn std::error::Error + Send>> =
        call_gpt(vec![extended_msg.clone()]).await;

    match llm_response_res {
        Ok(llm_resp) => llm_resp,
        Err(_) => call_gpt(vec![extended_msg.clone()])
            .await
            .expect("Failed to call request twice"),
    };

    String::from("something")
}

pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response: String =
        ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;

    let decoded_response: T = serde_json::from_str(llm_response.as_str())
        .expect("Failed to decode ai reponse from serde_json");

    decoded_response
}


pub async fn check_status_code(client:&Client,url:&str)->Result<u16,reqwest::Error>{
    let reponse: reqwest::Response = client.get(url).send().await?;
    Ok(reponse.status().as_u16())
}

pub fn read_code_template_contents()->String{
    let path:String  = String::from(CODE_TEMPLATE_PATH);

    fs::read_to_string(path).expect("Falied to read code template")
}

pub fn save_backend_code(contents:&String){
    let path:String  = String::from(EXEC_MAIN_PATH);

    fs::write(path,contents).expect("Falied to write main.rs template");
}

pub fn save_api_endpoints(api_endpoints:&String){
    let path:String  = String::from(EXEC_MAIN_PATH);

    fs::write(path,api_endpoints).expect("Falied to write API Endpoinnts template");
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::managing::print_user_input_to_goal;

    #[test]
    fn tests_extending_ai_fun() {
        let extended_msg: Message =
            external_ai_function(print_user_input_to_goal, "dummmy variable");

        dbg!(&extended_msg);
        assert_eq!(extended_msg.role, "system".to_string());
    }

    #[tokio::test]
    async fn tests_ai_task_requests() {
        let ai_fun_param: String =
            "Build me a web server for making stock price api requests".to_string();

        let res: String = ai_task_request(
            ai_fun_param,
            "Managing Agent",
            "Defining user requirements",
            print_user_input_to_goal,
        )
        .await;

        //  dbg!(res);

        assert!(res.len() > 20);
    }
}
