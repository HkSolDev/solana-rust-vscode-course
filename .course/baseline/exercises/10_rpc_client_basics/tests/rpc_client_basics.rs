use solana_commitment_config::CommitmentConfig;

#[test]
fn creates_confirmed_client_and_reads_status() -> anyhow::Result<()> {
    let rpc_url =
        std::env::var("SOLANA_RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8899".to_string());
    let client = exercise_10_rpc_client_basics::confirmed_client(&rpc_url);

    assert_eq!(client.url(), rpc_url);
    assert_eq!(client.commitment(), CommitmentConfig::confirmed());

    let status = exercise_10_rpc_client_basics::fetch_rpc_status(&client)?;
    assert_eq!(status.url, rpc_url);
    assert!(!status.solana_core.is_empty());

    Ok(())
}
