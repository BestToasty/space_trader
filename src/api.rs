use crate::cache::{save_shipyard, save_system};
use crate::logic::split_waypoint;
use crate::models::*;
use crate::{ACCOUNT_TOKEN, HOST_URL};
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

        crate::cache::save_agent(self.agent.as_ref().unwrap())?;
        crate::cache::save_ships(self.ships.as_ref().unwrap())?;
        crate::cache::save_contracts(self.contracts.as_ref().unwrap())?;
        Ok(())
    }

    // to be removed
    pub fn load_state(&self) -> LocalState {
        fs::read_to_string("state.json")
            .and_then(|data| Ok(serde_json::from_str(&data)?))
            .unwrap_or_default()
    }
    // to be removed
    pub fn save_state(&self, state: &LocalState) -> Result<()> {
        let path = "cache/state.json";
        let data = serde_json::to_string_pretty(state)?;
        fs::write(path, data)?;
        Ok(())
    }

    pub fn accept_contract(&self, contract_id: String) -> Result<ContractData> {
        let path = format!("my/contracts/{}/accept", contract_id);
        let url = format!("{}{}", HOST_URL, path);

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()?;
        if response.status().is_success() {
            Ok(response.json::<ContractData>()?)
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
            .send()?;

        let status = response.status();
        let body = response.json::<ContractResponse>()?;

        if let Some(data) = body.data {
            Ok(data)
        } else if let Some(err) = body.error {
            anyhow::bail!("API Fehler {}: {}", err.code, err.message)
        } else {
            anyhow::bail!("[!] Unbekanntes Antwortformat bei Status {}", status)
        }
    }

    pub fn fetch_waypoint_data(&self, waypoint_symbol: String) -> Result<WaypointData> {
        let waypoint_symbol = waypoint_symbol.trim();
        let parts: Vec<&str> = waypoint_symbol.split('-').collect();
        let system_symbol = format!("{}-{}", parts[0], parts[1]);
        let path = format!("systems/{}/waypoints/{}", system_symbol, waypoint_symbol);
        let url = format!("{}/{}", HOST_URL, path);

        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()?;

        let status = response.status();
        let body = response.json::<WaypointResponse>()?;

        if let Some(data) = body.data {
            Ok(data)
        } else if let Some(err) = body.error {
            anyhow::bail!("API Fehler {}: {}", err.code, err.message)
        } else {
            anyhow::bail!("[!] Unbekanntes Antwortformat bei Status {}", status)
        }
    }

    pub fn fetch_shipyard_data(&self, waypoint_symbol: String) -> Result<()> {
        let parts = split_waypoint(&waypoint_symbol)?;
        let system_symbol = format!("{}-{}", parts[0], parts[1]);
        let path = format!(
            "systems/{}/waypoints/{}/shipyard",
            system_symbol, waypoint_symbol
        );
        let url = format!("{}/{}", HOST_URL, path);

        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()?;

        let status = response.status();
        let body = response.json::<ShipyardResponse>()?;

        if let Some(data) = body.data {
            save_shipyard(system_symbol, data)?;
            Ok(())
        } else if let Some(err) = body.error {
            anyhow::bail!("API Fehler {}: {}", err.code, err.message)
        } else {
            anyhow::bail!("[!] Unbekanntes Antwortformat bei Status {}", status)
        }
    }

    pub fn fetch_system(&self, system_symbol: String) -> Result<()> {
        let path = format!("systems/{}", system_symbol);
        let url = format!("{}/{}", HOST_URL, path);

        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()?;

        let status = response.status();
        let body = response.json::<SystemResponse>()?;

        if let Some(data) = body.data {
            save_system(system_symbol, data)?;
            Ok(())
        } else if let Some(err) = body.error {
            anyhow::bail!("API Fehler {}: {}", err.code, err.message)
        } else {
            anyhow::bail!("[!] Unbekanntes Antwortformat bei Status {}", status)
        }
    }

    pub fn get_starting_waypoint(&mut self) -> Result<WaypointData> {
        let agent = self.get_agent()?;
        let parts: Vec<&str> = agent.headquarters.split('-').collect();
        let system_symbol = format!("{}-{}", parts[0], parts[1]);

        let path = format!("systems/{}/waypoints/{}", system_symbol, agent.headquarters);
        let url = format!("{}/{}", HOST_URL, path);

        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()?;

        let status = response.status();
        let body = response.json::<WaypointResponse>()?;

        if let Some(data) = body.data {
            Ok(data)
        } else if let Some(err) = body.error {
            anyhow::bail!("API Fehler {}: {}", err.code, err.message)
        } else {
            anyhow::bail!("[!] Unbekanntes Antwortformat bei Status {}", status)
        }
    }

    pub fn get_agent(&mut self) -> Result<&AgentData> {
        if self.agent.is_none() {
            eprintln!("Lade Agent von API..");
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
            .send()?;

        let status = response.status();
        let body = response.json::<AgentResponse>()?;

        if let Some(data) = body.data {
            Ok(data)
        } else if let Some(err) = body.error {
            anyhow::bail!("API Fehler {}: {}", err.code, err.message)
        } else {
            anyhow::bail!("[!] Unbekanntes Antwortformat bei Status {}", status)
        }
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
            .send()?;

        let status = response.status();
        let body = response.json::<ShipResponse>()?;

        if let Some(data) = body.data {
            Ok(data)
        } else if let Some(err) = body.error {
            anyhow::bail!("API Fehler {}: {}", err.code, err.message)
        } else {
            anyhow::bail!("[!] Unbekanntes Antwortformat bei Status {}", status)
        }
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

    pub fn fetch_shipyards_in_system(&self, system_symbol: String) -> Result<Vec<WaypointData>> {
        let path = format!("systems/{}/waypoints?traits=SHIPYARD", system_symbol);
        let url = format!("{}/{}", HOST_URL, path);

        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()?;

        let status = response.status();
        let body = response.json::<WaypointsResponse>()?;

        if let Some(data) = body.data {
            Ok(data)
        } else if let Some(err) = body.error {
            anyhow::bail!("API Fehler {}: {}", err.code, err.message)
        } else {
            anyhow::bail!("[!] Unbekanntes Antwortformat bei Status {}", status)
        }
    }
}
