use solana_client::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_keypair::Keypair;
use solana_signer::Signer;
use solana_system_interface::program as system_program;

#[test]
fn fetches_basic_account_fields() -> anyhow::Result<()> {
    let rpc_url =
        std::env::var("SOLANA_RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8899".to_string());
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let wallet = Keypair::new();
    let lamports = 500_000_000;

    let signature = client.request_airdrop(&wallet.pubkey(), lamports)?;
    client.confirm_transaction(&signature)?;

    let snapshot = exercise_12_account_info::fetch_account_snapshot(
        &client,
        &wallet.pubkey(),
        CommitmentConfig::confirmed(),
    )?;

    assert!(snapshot.lamports >= lamports);
    assert_eq!(snapshot.owner, system_program::ID);
    assert!(!snapshot.executable);
    assert_eq!(snapshot.data_len, 0);

    Ok(())
}
