use solana_address::Address;
use solana_keypair::Keypair;
#[allow(unused_imports)]
use solana_signer::Signer;

pub fn new_wallet() -> Keypair {
    todo!("create and return a new Solana keypair")
}

pub fn wallet_address(wallet: &Keypair) -> Address {
    let _ = wallet;
    todo!("return the account address for this wallet")
}
