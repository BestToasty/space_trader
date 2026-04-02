use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractResponse {
    pub data: Vec<ContractData>,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub total: i32,
    pub page: i32,
    pub limit: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContractData {
    pub id: String,
    pub faction_symbol: String,
    #[serde(rename = "type")]
    pub contract_type: String,
    pub terms: Terms,
    pub accepted: bool,
    pub fulfilled: bool,
    pub expiration: DateTime<Utc>,
    pub deadline_to_accept: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Terms {
    pub deadline: DateTime<Utc>,
    pub payment: Payment,
    pub deliver: Vec<Deliver>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub on_accepted: i32,
    pub on_fulfilled: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Deliver {
    pub trade_symbol: String,
    pub destination_symbol: String,
    pub units_required: i32,
    pub units_fulfilled: i32,
}
