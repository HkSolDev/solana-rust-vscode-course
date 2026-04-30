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

pub fn create_rent_exempt_account(
    client: &RpcClient,
    payer: &Keypair,
    new_account: &Keypair,
    space: usize,
    owner: &Address,
) -> Result<Signature> {
    let _ = (client, payer, new_account, space, owner);
    todo!(
        "calculate rent exemption, create the account, sign with payer and new account, then send"
    )
}
