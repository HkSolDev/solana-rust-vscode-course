use solana_client::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_keypair::Keypair;
use solana_signer::Signer;

#[test]
fn requests_airdrop_and_reads_balance() -> anyhow::Result<()> {
    let rpc_url =
        std::env::var("SOLANA_RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8899".to_string());
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let wallet = Keypair::new();
    let lamports = 1_000_000_000;

    let balance = exercise_02_airdrop_balance::request_airdrop_and_get_balance(
        &client,
        &wallet.pubkey(),
        lamports,
    )?;

    assert!(balance >= lamports);
    Ok(())
}
