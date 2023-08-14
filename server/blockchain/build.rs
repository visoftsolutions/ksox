use std::{path::PathBuf, str::FromStr};

use ethers::prelude::Abigen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let abi_source = "./src/contracts/Treasury.json";
    let out_file: PathBuf = PathBuf::from_str("./src/contracts/treasury.rs")?;
    Abigen::new("Treasury", abi_source)?
        .generate()?
        .write_to_file(out_file)?;

    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(&["proto/base.proto"], &["proto"])?;

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(&["../engine/proto/base.proto"], &["../engine/proto"])?;

    Ok(())
}
