use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AgentData {
    pub account_id: String,
    pub symbol: String,
    pub headquarters: String,
    pub credits: i32,
    pub starting_faction: String,
    pub ship_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AgentResponse {
    pub data: AgentData,
}
