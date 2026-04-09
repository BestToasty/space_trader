use crate::models::*;
use crate::{
    AGENT_CACHE_FILE, CACHE_DIR, CONTRACT_CACHE_FILE, SHIPS_CACHE_FILE,
    SHIPYARD_LOCATIONS_CACHE_FILE,
};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn load_contracts() -> anyhow::Result<Vec<ContractData>> {
    let path = format!("{}/{}", CACHE_DIR, CONTRACT_CACHE_FILE);

    let json = fs::read_to_string(path)?;
    let contracts = serde_json::from_str(&json)?;

    Ok(contracts)
}

pub fn load_ships() -> anyhow::Result<Vec<ShipData>> {
    let path = format!("{}/{}", CACHE_DIR, SHIPS_CACHE_FILE);

    let json = fs::read_to_string(path)?;
    let ships = serde_json::from_str(&json)?;

    Ok(ships)
}

pub fn load_agent() -> anyhow::Result<AgentData> {
    let path = format!("{}/{}", CACHE_DIR, AGENT_CACHE_FILE);

    let json = fs::read_to_string(path)?;
    let agent = serde_json::from_str(&json)?;

    Ok(agent)
}

pub fn save_shipyard_system_waypoints(
    waypoints: Vec<WaypointData>,
    system_symbol: &str,
) -> anyhow::Result<()> {
    let file_path = "cache/shipyard_waypoints.json";

    let mut cache: HashMap<String, Vec<WaypointData>> = if Path::new(file_path).exists() {
        let content = fs::read_to_string(file_path)?;
        serde_json::from_str(&content).unwrap_or_else(|_| HashMap::new())
    } else {
        HashMap::new()
    };

    cache.insert(system_symbol.to_string(), waypoints);

    let json = serde_json::to_string_pretty(&cache)?;
    fs::write(file_path, json)?;
    Ok(())
}

pub fn save_shipyard(system_symbol: String, shipyard: Shipyard) -> anyhow::Result<()> {
    let file_path = "cache/shipyards.json";

    let mut cache: HashMap<String, Shipyard> = if Path::new(file_path).exists() {
        let content = fs::read_to_string(file_path)?;
        serde_json::from_str(&content).unwrap_or_else(|_| HashMap::new())
    } else {
        HashMap::new()
    };

    cache.insert(system_symbol, shipyard);

    let json = serde_json::to_string_pretty(&cache)?;
    fs::write(file_path, json)?;
    Ok(())
}

pub fn save_system(system_symbol: String, system: System) -> anyhow::Result<()> {
    let file_path = format!("cache/{}.json", system_symbol);
    let waypoints = system.waypoints;

    let waypoint_map: HashMap<String, SystemWaypoint> = waypoints
        .into_iter()
        .map(|wp| (wp.symbol.clone(), wp))
        .collect();

    let json = serde_json::to_string_pretty(&waypoint_map)?;
    fs::write(file_path, json)?;
    Ok(())
}

pub fn load_shipyard_system_waypoints(system_symbol: &str) -> anyhow::Result<Vec<WaypointData>> {
    let path = format!("{}/{}", CACHE_DIR, SHIPYARD_LOCATIONS_CACHE_FILE);

    let json = fs::read_to_string(path)?;
    let shipyards: std::collections::HashMap<String, Vec<WaypointData>> =
        serde_json::from_str(&json)?;
    shipyards
        .get(system_symbol)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("System {} nicht im Cache gefunden", system_symbol))
}

pub fn save_contracts(contract_data: &Vec<ContractData>) -> anyhow::Result<()> {
    fs::create_dir_all("cache")?;

    let json = serde_json::to_string_pretty(&contract_data)?;
    let path = "cache/contract_cache.json";

    fs::write(path, json)?;
    Ok(())
}

pub fn save_agent(agent: &AgentData) -> anyhow::Result<()> {
    fs::create_dir_all("cache")?;

    let json = serde_json::to_string_pretty(&agent)?;
    let path = "cache/agent_cache.json";

    fs::write(path, json)?;
    Ok(())
}

pub fn save_ships(ships: &Vec<ShipData>) -> anyhow::Result<()> {
    fs::create_dir_all("cache")?;

    let json = serde_json::to_string_pretty(&ships)?;
    let path = "cache/ships_cache.json";

    fs::write(path, json)?;

    Ok(())
}

pub fn status() -> anyhow::Result<()> {
    let agent = load_agent()?;
    eprintln!("Symbol: {}, Credits: {}", agent.symbol, agent.credits);
    Ok(())
}

pub fn get_ship_by_index(index: usize) -> anyhow::Result<ShipData> {
    let ships = load_ships()?;
    ships
        .get(index)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("Index nicht gefunden"))
}

pub fn get_ship_by_symbol(symbol: &str) -> anyhow::Result<ShipData> {
    let ships = load_ships()?;
    ships
        .into_iter()
        .find(|c| c.symbol == symbol)
        .ok_or_else(|| anyhow::anyhow!("Ship mit Symbol {} nicht im Cache gefunden", symbol))
}

pub fn get_contract_by_index(index: usize) -> anyhow::Result<ContractData> {
    let contracts = load_contracts()?;
    contracts
        .get(index)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("Index nicht gefunden"))
}

pub fn get_contract_by_id(id: &str) -> anyhow::Result<ContractData> {
    let contracts = load_contracts()?;
    contracts
        .into_iter()
        .find(|c| c.id == id)
        .ok_or_else(|| anyhow::anyhow!("Vertrag mit ID {} nicht im Cache gefunden", id))
}

pub fn update_contract_in_cache(updated: ContractData) -> anyhow::Result<()> {
    let mut contracts = load_contracts()?;

    if let Some(index) = contracts.iter().position(|c| c.id == updated.id) {
        contracts[index] = updated;
    } else {
        anyhow::bail!("Vetrag {} wurde nicht im Cache gefunden!", updated.id);
    }

    save_contracts(&contracts)?;
    Ok(())
}

pub fn update_ship_in_cache(updated: ShipData) -> anyhow::Result<()> {
    let mut ships = load_ships()?;

    if let Some(index) = ships.iter().position(|c| c.symbol == updated.symbol) {
        ships[index] = updated;
    } else {
        anyhow::bail!("Schiff {} wurde nicht im Cache gefunden!", updated.symbol);
    }

    save_ships(&ships)?;
    Ok(())
}
