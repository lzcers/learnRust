mod calc;
mod data;
use anyhow::Result;

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    Ok(())
}
