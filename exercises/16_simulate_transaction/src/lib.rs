use anyhow::Result;
use solana_address::Address;
use solana_client::rpc_client::RpcClient;
use solana_keypair::Keypair;
#[allow(unused_imports)]
use solana_signer::Signer;
#[allow(unused_imports)]
use solana_system_interface::instruction as system_instruction;
#[allow(unused_imports)]
use solana_transaction::Transaction;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimulationSummary {
    pub ok: bool,
    pub logs: Vec<String>,
    pub units_consumed: Option<u64>,
}

pub fn simulate_lamport_transfer(
    client: &RpcClient,
    payer: &Keypair,
    recipient: &Address,
    lamports: u64,
) -> Result<SimulationSummary> {
    let _ = (client, payer, recipient, lamports);
    todo!("build a transfer transaction, simulate it, and summarize the result")
}
