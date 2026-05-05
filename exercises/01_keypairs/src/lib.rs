use solana_address::Address;
use solana_keypair::Keypair;
#[allow(unused_imports)]
use solana_signer::Signer;

pub fn new_wallet() -> Keypair {

    let keypair = Keypair::new();
    keypair


}

pub fn wallet_address(wallet: &Keypair) -> Address {
    let _ = wallet;
wallet.pubkey()
}
