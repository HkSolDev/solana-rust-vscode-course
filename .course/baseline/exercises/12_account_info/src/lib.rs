use anyhow::Result;
use solana_address::Address;
use solana_client::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccountSnapshot {
    pub lamports: u64,
    pub owner: Address,
    pub executable: bool,
    pub data_len: usize,
}

pub fn fetch_account_snapshot(
    client: &RpcClient,
    address: &Address,
    commitment: CommitmentConfig,
) -> Result<AccountSnapshot> {
    let _ = (client, address, commitment);
    todo!("fetch the account and summarize lamports, owner, executable flag, and data length")
}
