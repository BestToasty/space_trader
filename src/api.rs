use crate::models::agent::{AgentData, AgentResponse};
use crate::models::contract::{ContractData, ContractResponse};
use crate::models::ship::{ShipData, ShipReactor, ShipResponse};
use crate::models::waypoints::{WaypointData, WaypointResponse};
use anyhow::Result;
use reqwest::blocking::Client;

use crate::{ACCOUNT_TOKEN, HOST_URL};

pub struct SpaceTradersClient {
    client: Client,
    token: String,
    pub agent: Option<AgentData>,
    pub ships: Option<Vec<ShipData>>,
}

impl SpaceTradersClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            token: ACCOUNT_TOKEN.to_string(),
            agent: None,
            ships: None,
        }
    }

    pub fn print_raw(&self) -> anyhow::Result<()> {
        let path = format!("my/ships");
        let url = format!("{}/{}", HOST_URL, path);

        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()?;

        let json: serde_json::Value = response.json()?;

        println!("{:#?}", json);

        Ok(())
    }

    pub fn request_new_contract(&self, ship_symbol: &str) -> Result<()> {
        let path = format!("my/ships/{}/negotiate/contract", ship_symbol);
        let url = format!("{}/{}", HOST_URL, path);

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()?;

        if response.status().is_success() {
            Ok(())
        } else {
            anyhow::bail!(
                "Fehler beim Anfordern eines neuen Vetrages: {}",
                response.status()
            )
        }
    }

    pub fn accept_contract(&self, contract_id: &i32) -> Result<()> {
        let path = format!("my/contracts/{}/accept", contract_id);
        let url = format!("{}/{}", HOST_URL, path);

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()?;

        if response.status().is_success() {
            println!("Auftrag erfolgreich angenommen.");
            Ok(())
        } else {
            anyhow::bail!(
                "Fehler beim Akzeptieren des Vetrages: {}",
                response.status()
            )
        }
    }

    pub fn get_contract_data(&self) -> Result<Vec<ContractData>> {
        let path = format!("my/contracts");
        let url = format!("{}/{}", HOST_URL, path);

        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()?
            .json::<ContractResponse>()?;

        Ok(response.data)
    }

    pub fn get_starting_waypoint(&mut self) -> Result<WaypointData> {
        let agent = self.get_agent()?;
        let system_symbol = &agent.headquarters[0..7];

        let path = format!("systems/{}/waypoints/{}", system_symbol, agent.headquarters);
        let url = format!("{}/{}", HOST_URL, path);

        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()?
            .json::<WaypointResponse>()?;

        Ok(response.data)
    }

    pub fn get_agent(&mut self) -> Result<&AgentData> {
        if self.agent.is_none() {
            println!("Lade Agent von API..");
            let data = self.fetch_agent_from_api()?;
            self.agent = Some(data);
        }

        Ok(self.agent.as_ref().unwrap())
    }

    pub fn fetch_agent_from_api(&mut self) -> Result<AgentData> {
        let path: &str = "my/agent";
        let url = format!("{}/{}", HOST_URL, path);

        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()?
            .json::<AgentResponse>()?;

        Ok(response.data)
    }

    pub fn get_ships(&mut self) -> Result<&Vec<ShipData>> {
        if self.ships.is_none() {
            println!("Lade Ships von API..");
            let data = self.fetch_ship_data_from_api()?;
            self.ships = Some(data);
        }

        Ok(self.ships.as_ref().unwrap())
    }

    pub fn fetch_ship_data_from_api(&mut self) -> Result<Vec<ShipData>> {
        let path: &str = "my/ships";
        let url = format!("{}/{}", HOST_URL, path);

        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()?
            .json::<ShipResponse>()?;

        Ok(response.data)
    }
}
