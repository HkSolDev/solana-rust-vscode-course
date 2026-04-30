use anyhow::Result;
use solana_address::Address;
use solana_client::rpc_client::RpcClient;

pub fn request_airdrop_and_get_balance(
    client: &RpcClient,
    recipient: &Address,
    lamports: u64,
) -> Result<u64> {
    let _ = (client, recipient, lamports);
    todo!("request an airdrop, confirm it, then return the recipient balance")
}
