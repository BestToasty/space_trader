use crate::{
    api::SpaceTradersClient,
    cache::{
        cache_system_shipyard, load_shipyard_system_waypoints, save_shipyard,
        save_shipyard_system_waypoints,
    },
    models::WaypointData,
};
use std::{
    i32,
    io::{self, IsTerminal, Read},
};

pub fn get_input_from_pipe() -> anyhow::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let clean = buffer.trim().to_string();

    Ok(clean)
}

pub fn fetch_and_cache_shipyards(
    client: SpaceTradersClient,
    system_symbol: String,
) -> anyhow::Result<()> {
    let shipyards = client.fetch_shipyards_in_system(system_symbol.clone())?;
    save_shipyard_system_waypoints(shipyards, &system_symbol.clone())?;
    Ok(())
}

pub fn fetch_and_cache_shipyard_data(
    client: SpaceTradersClient,
    waypoint_symbol: String,
) -> anyhow::Result<()> {
    let shipyard = client.fetch_shipyard_data(waypoint_symbol.clone())?;
    save_shipyard(waypoint_symbol, shipyard)?;
    Ok(())
}

pub fn fetch_and_cache_system_shipyards(
    client: SpaceTradersClient,
    system_symbol: String,
) -> anyhow::Result<()> {
    // for each shipyard in SYSTEM
    let shipyards = load_shipyard_system_waypoints(&system_symbol)?;
    for shipyard_waypoint in shipyards {
        let shipyard_data = client.fetch_shipyard_data(shipyard_waypoint.symbol)?;
        cache_system_shipyard(system_symbol.clone(), shipyard_data)?;
    }
    Ok(())
}

pub fn resolve_waypoint_symbol(arg_symbol: Option<String>) -> anyhow::Result<String> {
    if let Some(symbol) = arg_symbol {
        return Ok(symbol);
    }
    if !io::stdin().is_terminal() {
        return Ok(get_input_from_pipe()?);
    }
    Err(anyhow::anyhow!("[!] Kein Waypoint-Symbol gefunden!"))
}

pub fn resolve_system_symbol(arg_symbol: Option<String>) -> anyhow::Result<String> {
    if let Some(symbol) = arg_symbol {
        return Ok(symbol);
    }
    if !io::stdin().is_terminal() {
        return Ok(get_input_from_pipe()?);
    }
    Err(anyhow::anyhow!("[!] Kein System-Symbol gefunden!"))
}

pub fn resolve_ship_symbol(arg_symbol: Option<String>) -> anyhow::Result<String> {
    if let Some(symbol) = arg_symbol {
        return Ok(symbol);
    }
    if !io::stdin().is_terminal() {
        return Ok(get_input_from_pipe()?);
    }
    Err(anyhow::anyhow!("[!] Kein Ship-Symbol gefunden!"))
}

pub fn resolve_contract_id(arg_id: Option<String>) -> anyhow::Result<String> {
    if let Some(id) = arg_id {
        return Ok(id);
    }

    if !io::stdin().is_terminal() {
        return Ok(get_input_from_pipe()?);
    }

    Err(anyhow::anyhow!("Keine Contract-ID gefunden!"))
}

pub fn distance_between_waypoints(x1: i32, y1: i32, x2: i32, y2: i32) -> anyhow::Result<i32> {
    let x_distance = (x1 - x2) as f64;
    let y_distance = (y1 - y2) as f64;
    let radius = x_distance.powi(2) + y_distance.powi(2);
    let hypotenuse = radius.sqrt() as i32;
    Ok(hypotenuse)
}

pub fn find_nearest_shipyard(
    shipyards: Vec<WaypointData>,
    x: i32,
    y: i32,
) -> anyhow::Result<WaypointData> {
    let clostes_shipyard = shipyards
        .into_iter()
        .min_by_key(|s| distance_between_waypoints(x, y, s.x, s.y).unwrap_or(i32::MAX));
    clostes_shipyard.ok_or_else(|| anyhow::anyhow!("[!] Keine Shipyards in der Liste gefunden"))
}

pub fn resolve_coordinates(x: Option<i32>, y: Option<i32>) -> anyhow::Result<(i32, i32)> {
    if let Some(x) = x {
        if let Some(y) = y {
            return Ok((x, y));
        }
    }
    let parts: Vec<String> = if !io::stdin().is_terminal() {
        let input = get_input_from_pipe()?;
        input.split(':').map(|s| s.to_string()).collect()
    } else {
        Vec::new()
    };

    if parts.len() >= 2 {
        let px = parts[0].parse::<i32>()?;
        let py = parts[1].parse::<i32>()?;
        return Ok((px, py));
    }

    anyhow::bail!("[!] Keine Koordinaten angegeben.")
}

pub fn split_waypoint(waypoint_symbol: &String) -> anyhow::Result<Vec<String>> {
    let parts: Vec<String> = waypoint_symbol.split('-').map(|s| s.to_string()).collect();
    Ok(parts)
}
