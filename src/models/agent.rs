use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AgentData {
    pub account_id: String,
    pub symbol: String,
    pub headquarters: String,
    pub credits: i32,
    pub starting_faction: String,
    pub ship_count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentResponse {
    pub data: AgentData,
}
