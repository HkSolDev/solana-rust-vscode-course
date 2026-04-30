use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

use solana_keypair::Keypair;
use solana_signer::Signer;

#[test]
fn loads_solana_cli_keypair_file_without_printing_secret() -> anyhow::Result<()> {
    let wallet = Keypair::new();
    let unique = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let path = std::env::temp_dir().join(format!(
        "course-wallet-{}-{unique}.json",
        std::process::id()
    ));
    let keypair_bytes = wallet.to_bytes().to_vec();

    fs::write(&path, serde_json::to_string(&keypair_bytes)?)?;

    let loaded = exercise_11_wallet_file::load_keypair_file(&path)?;
    let address = exercise_11_wallet_file::wallet_address_from_file(&path)?;

    fs::remove_file(&path)?;

    assert_eq!(loaded.pubkey(), wallet.pubkey());
    assert_eq!(address, wallet.pubkey());

    Ok(())
}
