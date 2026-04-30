use anyhow::Result;
use solana_client::rpc_client::RpcClient;
#[allow(unused_imports)]
use solana_commitment_config::CommitmentConfig;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RpcStatus {
    pub url: String,
    pub slot: u64,
    pub solana_core: String,
}

pub fn confirmed_client(rpc_url: &str) -> RpcClient {
    let _ = rpc_url;
    todo!("create an RpcClient using confirmed commitment")
}

pub fn fetch_rpc_status(client: &RpcClient) -> Result<RpcStatus> {
    let _ = client;
    todo!("fetch the current slot and node version from the RPC client")
}
