use solana_client::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_keypair::Keypair;
use solana_signer::Signer;
use solana_system_interface::program as system_program;

#[test]
fn creates_rent_exempt_system_account() -> anyhow::Result<()> {
    let rpc_url =
        std::env::var("SOLANA_RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8899".to_string());
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let payer = Keypair::new();
    let new_account = Keypair::new();
    let space = 8;

    let airdrop = client.request_airdrop(&payer.pubkey(), 1_000_000_000)?;
    client.confirm_transaction(&airdrop)?;

    exercise_14_rent_create_account::create_rent_exempt_account(
        &client,
        &payer,
        &new_account,
        space,
        &system_program::ID,
    )?;

    let account = client.get_account(&new_account.pubkey())?;
    assert_eq!(account.owner, system_program::ID);
    assert_eq!(account.data.len(), space);
    assert!(account.lamports >= client.get_minimum_balance_for_rent_exemption(space)?);

    Ok(())
}
