use crate::models::main::llm::{APIResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Response};
use std::env;

pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    let api_key: String = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found in env variable");
    let api_org: String = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found in env variable");

    let url: &str = "https://api.openai.com/v1/chat/completions";

    let mut headers: HeaderMap = HeaderMap::new();

    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    let client: Client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    let chat_completion = ChatCompletion {
        model: "gpt-3.5-turbo".to_string(),
        messages,
        temperature: 0.1,
    };

    // let res_raw = client
    //     .post(url)
    //     .json(&chat_completion)
    //     .send()
    //     .await
    //     .unwrap();

    // dbg!(res_raw.text().await.unwrap());

    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_call_to_openai() {
        let message = Message {
            role: "user".to_string(),
            content: "Hi there, this is a test. Give me a short response.".to_string(),
        };

        let messages: Vec<Message> = vec![message];

        let res = call_gpt(messages).await;

        //    dbg!(res);
        match res {
            Ok(res_str) => {
                dbg!(res_str);
                assert!(true);
            }
            Err(_) => {
                assert!(false);
                dbg!("err================>");
            }
        }
    }
}
