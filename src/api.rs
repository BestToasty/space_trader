use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};
use crate::models::agent::{AgentData, AgentResponse};
use crate::models::waypoints::{WaypointData, WaypointResponse};
use crate::models::contract::{ContractData, ContractResponse};
use anyhow::Result;

use crate::{HOST_URL, ACCOUNT_TOKEN};

pub fn get_contract_data(client: &Client, agent_data: &AgentData) -> Result<Vec<ContractData>> {

    let path = format!("my/contracts");
    let url = format!("{}/{}", HOST_URL, path);

    let response = client.get(url)
        .header("Authorization", format!("Bearer {}", ACCOUNT_TOKEN))
        .send()?
        .json::<ContractResponse>()?;

    Ok(response.data)
}

pub fn get_starting_waypoint(client: &Client, agent_data: &AgentData) ->Result<WaypointData> {

    let system_symbol = &agent_data.headquarters[0..7];

    let path = format!("systems/{}/waypoints/{}", system_symbol, agent_data.headquarters);
    let url = format!("{}/{}", HOST_URL, path);

    let response = client.get(url)
        .header("Authorization", format!("Bearer {}", ACCOUNT_TOKEN))
        .send()?
        .json::<WaypointResponse>()?;

    Ok(response.data)
}

pub fn get_my_agent(client: &Client) -> Result<AgentData> {

    let path: &str = "my/agent";
    let url  =  format!("{}/{}", HOST_URL, path);

    let response = client.get(url)
        .header("Authorization", format!("Bearer {}", ACCOUNT_TOKEN))
        .send()?
        .json::<AgentResponse>()?;

    Ok(response.data)
}
