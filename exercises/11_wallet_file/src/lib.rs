use std::path::Path;

use anyhow::Result;
use solana_address::Address;
use solana_keypair::Keypair;
#[allow(unused_imports)]
use solana_signer::Signer;

pub fn load_keypair_file(path: &Path) -> Result<Keypair> {
    let _ = path;
    todo!("read a Solana CLI-style keypair JSON file")
}

pub fn wallet_address_from_file(path: &Path) -> Result<Address> {
    let _ = path;
    todo!("load the keypair file and return its public account address")
}
