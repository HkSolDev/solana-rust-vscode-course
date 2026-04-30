use anyhow::Result;
use solana_address::Address;
use solana_instruction::Instruction;
#[allow(unused_imports)]
use spl_token_interface::{instruction as token_instruction, ID as TOKEN_PROGRAM_ID};

pub fn build_close_token_account_instruction(
    token_account: &Address,
    destination: &Address,
    owner: &Address,
) -> Result<Instruction> {
    let _ = (token_account, destination, owner);
    todo!("build an SPL Token close_account instruction")
}
