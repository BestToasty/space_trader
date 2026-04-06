use crate::models::*;
use crate::{AGENT_CACHE_FILE, CACHE_DIR, CONTRACT_CACHE_FILE, SHIPS_CACHE_FILE};
use std::fs;

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
