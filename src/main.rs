use anyhow::Result;
use space_trader::api::SpaceTradersClient;

fn main() -> Result<()> {
    let mut client = SpaceTradersClient::new();
    let ships = client.get_ships()?;
    println!("{:?}", ships[0].symbol);
    Ok(())
}
