use anyhow::Result;
use solana_address::Address;
use solana_client::rpc_client::RpcClient;
use solana_keypair::Keypair;
use solana_signature::Signature;
#[allow(unused_imports)]
use solana_signer::Signer;
#[allow(unused_imports)]
use solana_system_interface::instruction as system_instruction;
#[allow(unused_imports)]
use solana_transaction::Transaction;

pub fn send_lamports(
    client: &RpcClient,
    payer: &Keypair,
    recipient: &Address,
    lamports: u64,
) -> Result<Signature> {
    let _ = (client, payer, recipient, lamports);
    todo!("build, sign, send, and confirm a System Program transfer")
}
