use std::io::{self, IsTerminal, Read};

pub fn request_new_contract() -> anyhow::Result<()> {
    todo!()
}

pub fn get_input_from_pipe() -> anyhow::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let clean = buffer.trim().to_string();

    Ok(clean)
}

pub fn find_shipyard_near_waypoint(symbol: String) -> anyhow::Result<()> {
    todo!()
}

pub fn resolve_ship_symbol(arg_symbol: Option<String>) -> anyhow::Result<String> {
    if let Some(symbol) = arg_symbol {
        return Ok(symbol);
    }
    if !io::stdin().is_terminal() {
        return Ok(get_input_from_pipe()?);
    }
    Err(anyhow::anyhow!("Kein Ship-Symbol gefunden!"))
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
