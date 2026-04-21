use crate::ACCOUNT_TOKEN;
use crate::HOST_URL;
use crate::cache::save_system;
use crate::{logic::split_waypoint, models::*};
use anyhow::Result;
use reqwest::blocking::Client;

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

    pub fn fetch_shipyard_data(&self, waypoint_symbol: String) -> Result<Shipyard> {
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

        // let status = response.status();
        // let text = response.text()?; // Lies den Body als rohen Text
        // println!("DEBUG API Response: {}", text); // Schau dir an, was wirklich ankommt

        // let body: ShipyardResponse = serde_json::from_str(&text)?;

        let status = response.status();
        let body = response.json::<ShipyardResponse>()?;

        if let Some(data) = body.data {
            Ok(data)
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

    pub fn navigate_ship(
        &self,
        destination_symbol: String,
        ship_symbol: String,
    ) -> Result<ShipNavigation> {
        let path = format!("my/ships/{}/navigate", ship_symbol);
        let url = format!("{}/{}", HOST_URL, path);
        let body = PurchaseRequest {
            waypoint_symbol: destination_symbol,
        };

        let response = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&body)
            .send()?;

        let status = response.status();
        let body = response.json::<SpaceTradersResponse<NavData>>()?;

        if let Some(data) = body.data {
            Ok(data.nav)
        } else if let Some(err) = body.error {
            anyhow::bail!("API Fehler {}: {}", err.code, err.message)
        } else {
            anyhow::bail!("[!] Unbekanntes Antwortformat bei Status {}", status)
        }
    }

    pub fn orbit_ship(&self, ship_symbol: String) -> Result<ShipNavigation> {
        let path = format!("my/ships/{}/orbit", ship_symbol);
        let url = format!("{}/{}", HOST_URL, path);

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()?;

        let status = response.status();
        let body = response.json::<SpaceTradersResponse<NavData>>()?;

        if let Some(data) = body.data {
            Ok(data.nav)
        } else if let Some(err) = body.error {
            anyhow::bail!("API Fehler {}: {}", err.code, err.message)
        } else {
            anyhow::bail!("[!] Unbekanntes Antwortformat bei Status {}", status)
        }
    }

    pub fn dock_ship(&self, ship_symbol: String) -> Result<ShipNavigation> {
        let path = format!("my/ships/{}/dock", ship_symbol);
        let url = format!("{}/{}", HOST_URL, path);

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()?;

        let status = response.status();
        let body = response.json::<SpaceTradersResponse<NavData>>()?;

        if let Some(data) = body.data {
            Ok(data.nav)
        } else if let Some(err) = body.error {
            anyhow::bail!("API Fehler {}: {}", err.code, err.message)
        } else {
            anyhow::bail!("[!] Unbekanntes Antwortformat bei Status {}", status)
        }
    }
}
