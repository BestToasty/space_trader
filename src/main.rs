use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};
use space_trader::{get_my_agent, get_starting_waypoint, get_contract_data};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let agent_data = get_my_agent(&client)?;
    let starting_waypoint = get_starting_waypoint(&client, &agent_data)?;
    let contracts = get_contract_data(&client, &agent_data)?;
    println!("{:?}", contracts);
    Ok(())
}
