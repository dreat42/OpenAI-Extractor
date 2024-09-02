use crate::models::agent::agent_basic::AgentState;
use crate::models::general::AgentState::Message;


pub trait BasicTraits{
    fn new(objective:String,position:String)->Self;
    fn update_state(&mut self, new_state:AgentState);
    fn new(&mut self)->&String;
    fn get_position(&mut self)->&String;
    fn get_state(&mut self)->&AgentState;
    fn get_memory(&mut self)->&Vec<Message>;

}