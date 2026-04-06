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

pub fn find_shipyard_near_waypoint(symbol: String) -> anyhow::Result<ShipCargo> {
    todo!()
}
