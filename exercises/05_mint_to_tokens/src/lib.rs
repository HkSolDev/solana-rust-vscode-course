use anyhow::Result;
use solana_address::Address;
use solana_instruction::Instruction;
#[allow(unused_imports)]
use spl_token_2022_interface::{instruction as token_instruction, ID as TOKEN_2022_PROGRAM_ID};

pub fn build_mint_to_checked_instruction(
    mint: &Address,
    destination_token_account: &Address,
    mint_authority: &Address,
    amount: u64,
    decimals: u8,
) -> Result<Instruction> {
    let _ = (
        mint,
        destination_token_account,
        mint_authority,
        amount,
        decimals,
    );
    todo!("build a Token-2022 mint_to_checked instruction")
}
