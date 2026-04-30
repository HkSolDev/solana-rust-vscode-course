use solana_client::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_keypair::Keypair;
use solana_signer::Signer;

#[test]
fn sends_lamports_between_wallets() -> anyhow::Result<()> {
    let rpc_url =
        std::env::var("SOLANA_RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8899".to_string());
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let payer = Keypair::new();
    let recipient = Keypair::new();
    let transfer_lamports = 250_000_000;

    let airdrop = client.request_airdrop(&payer.pubkey(), 1_000_000_000)?;
    client.confirm_transaction(&airdrop)?;

    exercise_03_send_sol::send_lamports(&client, &payer, &recipient.pubkey(), transfer_lamports)?;

    let recipient_balance = client.get_balance(&recipient.pubkey())?;
    assert!(
        recipient_balance >= transfer_lamports,
        "recipient should receive the transferred lamports"
    );

    Ok(())
}
