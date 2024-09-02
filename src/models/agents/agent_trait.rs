use crate::model::models::agent_basic::basic_agent::BasicAgent;


use async_trait::async_trait;

use serde::{Deserialize,Serialize};
use std::fmt::Debug;


pub struct RouteObject{
    pub is_route_dynamic:String,
    pub method:String,
    pub request_body:serde_json::Value,
    pub response:serde_json::Value,
    pub route:String,
}



pub struct ProjectScope{
    pub is_crud_required:bool,
    pub is_user_login_and_logout:bool,
    pub is_external_urls_required:bool,
}


#[derive(Debug,Serialize,Deserialize,Clone,PartialEq)]
pub struct FactSheet{

    pub project_description:String,
    pub project_scope:Options<ProjectScope>,
    pub external_urls:Options<Vec<String>>,
    pub external_urls:Options<String>,
    pub api_endpoint_schema:Options<Vec<RouteObject>>

}