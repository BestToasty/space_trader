use crate::models::common::Meta;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractResponse {
    pub data: Vec<ContractData>,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContractData {
    pub id: String,
    pub faction_symbol: String,
    #[serde(rename = "type")]
    pub contract_type: ContractType,
    pub terms: Terms,
    pub accepted: bool,
    pub fulfilled: bool,
    pub expiration: DateTime<Utc>,
    pub deadline_to_accept: Option<DateTime<Utc>>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ContractType {
    PROCUREMENT,
    TRANSPORT,
    SHUTTLE,
}

impl ContractData {
    pub fn accept_contract(&self, client: &crate::api::SpaceTradersClient) -> anyhow::Result<()> {
        let contract = client.accept_contract(self.id.clone())?;
        crate::cache::update_contract_in_cache(contract)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Terms {
    pub deadline: DateTime<Utc>,
    pub payment: Payment,
    pub deliver: Vec<Deliver>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub on_accepted: i32,
    pub on_fulfilled: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Deliver {
    pub trade_symbol: String,
    pub destination_symbol: String,
    pub units_required: i32,
    pub units_fulfilled: i32,
}
