use anyhow::Result;
use solana_address::Address;
use solana_client::rpc_client::RpcClient;
use solana_keypair::Keypair;
use solana_signature::Signature;
#[allow(unused_imports)]
use solana_signer::Signer;
#[allow(unused_imports)]
use solana_system_interface::instruction as system_instruction;
use solana_system_interface::instruction::transfer;
#[allow(unused_imports)]
use solana_transaction::Transaction;

pub fn send_lamports(
    client: &RpcClient,
    payer: &Keypair,
    recipient: &Address,
    lamports: u64,
) -> Result<Signature> {
    let _ = (client, payer, recipient, lamports);
    // todo!("build, sign, send, and confirm a System Program transfer")
    let inx = transfer(&payer.pubkey(), recipient, lamports);

    let recent_blockhash = client.get_latest_blockhash()?;

    let tx = Transaction::new_signed_with_payer(
        &[inx],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );

    let client_x = client.send_and_confirm_transaction(&tx)?;
    Ok(client_x)
}
