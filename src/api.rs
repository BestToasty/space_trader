use crate::models::*;
use crate::{
    ACCOUNT_TOKEN, AGENT_CACHE_FILE, CACHE_DIR, CONTRACT_CACHE_FILE, HOST_URL, SHIPS_CACHE_FILE,
};
use anyhow::Result;
use reqwest::blocking::Client;
use std::fs;

pub struct SpaceTradersClient {
    client: Client,
    token: String,
    pub agent: Option<AgentData>,
    pub ships: Option<Vec<ShipData>>,
    pub contracts: Option<Vec<ContractData>>,
}

impl SpaceTradersClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            token: ACCOUNT_TOKEN.to_string(),
            agent: None,
            ships: None,
            contracts: None,
        }
    }

    pub fn init(&mut self) -> anyhow::Result<()> {
        let agent_data = self.fetch_agent_from_api()?;
        let ships_data = self.fetch_ship_data_from_api()?;
        let contract_data = self.fetch_contract_data()?;

        self.agent = Some(agent_data);
        self.ships = Some(ships_data);
        self.contracts = Some(contract_data);

        self.cache_agent()?;
        self.cache_ships()?;
        self.cache_contracts()?;

        Ok(())
    }

    pub fn load_contracts_from_cache(&mut self) -> anyhow::Result<()> {
        let path = format!("{}/{}", CACHE_DIR, CONTRACT_CACHE_FILE);

        let json = fs::read_to_string(path)?;
        self.contracts = Some(serde_json::from_str(&json)?);
        Ok(())
    }

    pub fn load_ships_from_cache(&mut self) -> anyhow::Result<()> {
        let path = format!("{}/{}", CACHE_DIR, SHIPS_CACHE_FILE);

        let json = fs::read_to_string(path)?;
        self.ships = Some(serde_json::from_str(&json)?);
        Ok(())
    }

    pub fn load_agent_from_cache(&mut self) -> anyhow::Result<()> {
        let path = format!("{}/{}", CACHE_DIR, AGENT_CACHE_FILE);

        let json = fs::read_to_string(path)?;
        self.agent = Some(serde_json::from_str(&json)?);
        Ok(())
    }

    pub fn cache_contracts(&self) -> anyhow::Result<()> {
        fs::create_dir_all("cache")?;

        if let Some(ref a) = self.contracts {
            let json = serde_json::to_string_pretty(a)?;
            let path = "cache/contract_cache.json";

            fs::write(path, json)?;
        }
        Ok(())
    }

    pub fn cache_agent(&self) -> anyhow::Result<()> {
        fs::create_dir_all("cache")?;

        if let Some(ref a) = self.agent {
            let json = serde_json::to_string_pretty(a)?;
            let path = "cache/agent_cache.json";

            fs::write(path, json)?;
        }
        Ok(())
    }

    pub fn cache_ships(&self) -> anyhow::Result<()> {
        fs::create_dir_all("cache")?;

        if let Some(ref a) = self.ships {
            let json = serde_json::to_string_pretty(a)?;
            let path = "cache/ships_cache.json";

            fs::write(path, json)?;
        }
        Ok(())
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

    pub fn fetch_contract_data(&self) -> Result<Vec<ContractData>> {
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
