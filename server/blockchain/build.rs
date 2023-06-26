use std::{path::PathBuf, str::FromStr};

use ethers::prelude::Abigen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let abi_source =
        "../../blockchain/@app/treasury/artifacts/contracts/Treasury.sol/Treasury.json";
    let out_file: PathBuf = PathBuf::from_str("./src/contracts/abigen/treasury.rs")?;
    Abigen::new("Treasury", abi_source)?
        .generate()?
        .write_to_file(out_file)?;
    Ok(())
}
