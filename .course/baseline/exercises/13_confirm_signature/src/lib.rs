use anyhow::Result;
use solana_address::Address;
use solana_client::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_signature::Signature;

pub fn request_airdrop_and_confirm(
    client: &RpcClient,
    recipient: &Address,
    lamports: u64,
    commitment: CommitmentConfig,
) -> Result<Signature> {
    let _ = (client, recipient, lamports, commitment);
    todo!("request an airdrop and confirm the returned signature with the requested commitment")
}
