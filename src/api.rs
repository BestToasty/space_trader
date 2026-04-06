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

    pub fn load_state(&self) -> LocalState {
        fs::read_to_string("state.json")
            .and_then(|data| Ok(serde_json::from_str(&data)?))
            .unwrap_or_default()
    }

    pub fn save_state(&self, state: &LocalState) -> Result<()> {
        let path = "cache/state.json";
        let data = serde_json::to_string_pretty(state)?;
        fs::write(path, data)?;
        Ok(())
    }

    pub fn load_all_cache(&mut self) -> anyhow::Result<()> {
        self.load_agent_from_cache()?;
        self.load_contracts_from_cache()?;
        self.load_ships_from_cache()?;
        Ok(())
    }

    pub fn load_contracts_from_cache(&mut self) -> anyhow::Result<()> {
        let path = format!("{}/{}", CACHE_DIR, CONTRACT_CACHE_FILE);

        let json = fs::read_to_string(path)?;
        self.contracts = Some(serde_json::from_str(&json)?);

        self.update_state_contract_id()?;
        Ok(())
    }

    pub fn load_ships_from_cache(&mut self) -> anyhow::Result<()> {
        let path = format!("{}/{}", CACHE_DIR, SHIPS_CACHE_FILE);

        let json = fs::read_to_string(path)?;
        self.ships = Some(serde_json::from_str(&json)?);

        self.update_last_ship_symbol()?;
        Ok(())
    }

    pub fn load_agent_from_cache(&mut self) -> anyhow::Result<()> {
        let path = format!("{}/{}", CACHE_DIR, AGENT_CACHE_FILE);

        let json = fs::read_to_string(path)?;
        self.agent = Some(serde_json::from_str(&json)?);
        Ok(())
    }

    pub fn cache_contracts(&mut self) -> anyhow::Result<()> {
        fs::create_dir_all("cache")?;

        if let Some(ref a) = self.contracts {
            let json = serde_json::to_string_pretty(a)?;
            let path = "cache/contract_cache.json";

            fs::write(path, json)?;
        }

        self.update_state_contract_id()?;
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

    pub fn cache_ships(&mut self) -> anyhow::Result<()> {
        fs::create_dir_all("cache")?;

        if let Some(ref a) = self.ships {
            let json = serde_json::to_string_pretty(a)?;
            let path = "cache/ships_cache.json";

            fs::write(path, json)?;
        }

        self.update_last_ship_symbol()?;
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

    pub fn accept_contract(&self, contract_id: String) -> Result<()> {
        let path = format!("my/contracts/{}/accept", contract_id);
        let url = format!("{}{}", HOST_URL, path);

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()?;
        if response.status().is_success() {
            Ok(())
        } else {
            anyhow::bail!(
                "[!] Fehler beim Akzeptieren des Vertrages. {}",
                response.status()
            )
        }
    }

    pub fn request_new_contract(&self, ship_symbol: &str) -> Result<()> {
        let path = format!("my/ships/{}/negotiate/contract", ship_symbol);
        let url = format!("{}{}", HOST_URL, path);

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()?;

        if response.status().is_success() {
            Ok(())
        } else {
            anyhow::bail!(
                "[!] Fehler beim Anfordern eines neuen Vetrages: {}",
                response.status()
            )
        }
    }

    pub fn fetch_contract_data(&mut self) -> Result<Vec<ContractData>> {
        let path = format!("my/contracts");
        let url = format!("{}/{}", HOST_URL, path);

        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()?
            .json::<ContractResponse>()?;

        let mut state: LocalState = self.load_state();

        if let Some(first) = response.data.first() {
            state.last_contract_id = Some(first.id.clone());
            self.save_state(&state)?;
            self.update_state(StateUpdate::LastContractId(first.id.clone()))?;
        }

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

        self.update_state(StateUpdate::LastWaypointSymbol(
            response.data.symbol.clone(),
        ))?;

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

        if let Some(first) = response.data.first() {
            self.update_state(StateUpdate::LastShipSymbol(first.symbol.clone()))?;
        }

        Ok(response.data)
    }

    pub fn update_last_ship_symbol(&mut self) -> Result<()> {
        if let Some(first) = self.ships.as_ref().and_then(|c| c.first()) {
            self.update_state(StateUpdate::LastShipSymbol(first.symbol.clone()))?;
        }
        Ok(())
    }

    pub fn update_state_contract_id(&mut self) -> Result<()> {
        if let Some(first) = self.contracts.as_ref().and_then(|c| c.first()) {
            self.update_state(StateUpdate::LastContractId(first.id.clone()))?;
        }
        Ok(())
    }

    pub fn update_state_waypoint_symbol(&mut self, symbol: String) -> Result<()> {
        self.update_state(StateUpdate::LastWaypointSymbol(symbol))?;
        Ok(())
    }

    pub fn update_state(&mut self, update: StateUpdate) -> Result<()> {
        let mut state: LocalState = self.load_state();

        match update {
            StateUpdate::LastContractId(id) => {
                state.last_contract_id = Some(id);
                self.save_state(&state)?;
            }
            StateUpdate::LastShipSymbol(symbol) => {
                state.last_ship_symbol = Some(symbol);
                self.save_state(&state)?;
            }
            StateUpdate::LastWaypointSymbol(symbol) => {
                state.last_waypoint_symbol = Some(symbol);
                self.save_state(&state)?;
            }
        }

        Ok(())
    }
}
