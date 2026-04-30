use anyhow::Result;
use solana_address::Address;
use solana_instruction::Instruction;
#[allow(unused_imports)]
use spl_token_interface::{instruction as token_instruction, ID as TOKEN_PROGRAM_ID};

pub fn build_transfer_checked_instruction(
    source: &Address,
    mint: &Address,
    destination: &Address,
    authority: &Address,
    amount: u64,
    decimals: u8,
) -> Result<Instruction> {
    let _ = (source, mint, destination, authority, amount, decimals);
    todo!("build an SPL Token transfer_checked instruction")
}
