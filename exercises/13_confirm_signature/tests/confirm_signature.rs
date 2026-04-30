use solana_client::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_keypair::Keypair;
use solana_signer::Signer;

#[test]
fn confirms_signature_before_reading_balance() -> anyhow::Result<()> {
    let rpc_url =
        std::env::var("SOLANA_RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8899".to_string());
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let wallet = Keypair::new();
    let lamports = 600_000_000;

    let signature = exercise_13_confirm_signature::request_airdrop_and_confirm(
        &client,
        &wallet.pubkey(),
        lamports,
        CommitmentConfig::confirmed(),
    )?;

    assert!(client.confirm_transaction(&signature)?);
    assert!(client.get_balance(&wallet.pubkey())? >= lamports);

    Ok(())
}
