use solana_client::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_keypair::Keypair;
use solana_signer::Signer;

#[test]
fn simulates_transfer_before_sending() -> anyhow::Result<()> {
    let rpc_url =
        std::env::var("SOLANA_RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8899".to_string());
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let payer = Keypair::new();
    let recipient = Keypair::new();

    let airdrop = client.request_airdrop(&payer.pubkey(), 1_000_000_000)?;
    let mut confirmed = false;
    for _ in 0..20 {
        if client.confirm_transaction(&airdrop)? {
            confirmed = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    anyhow::ensure!(confirmed, "airdrop was not confirmed");

    let summary = exercise_16_simulate_transaction::simulate_lamport_transfer(
        &client,
        &payer,
        &recipient.pubkey(),
        1_000_000,
    )?;

    assert!(
        summary.ok,
        "simulation should not contain a transaction error"
    );
    assert!(
        summary.units_consumed.is_some() || !summary.logs.is_empty(),
        "simulation should return useful diagnostics"
    );

    Ok(())
}
