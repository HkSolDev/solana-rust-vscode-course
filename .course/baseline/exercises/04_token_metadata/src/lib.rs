use anyhow::Result;
use solana_address::Address;
use solana_instruction::Instruction;
#[allow(unused_imports)]
use solana_system_interface::instruction as system_instruction;
#[allow(unused_imports)]
use spl_token_2022_interface::{
    extension::metadata_pointer::instruction as metadata_pointer_instruction,
    instruction as token_instruction, ID as TOKEN_2022_PROGRAM_ID,
};
#[allow(unused_imports)]
use spl_token_metadata_interface::{instruction as token_metadata_instruction, state::Field};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TokenMetadataConfig {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
    pub description: String,
}

pub fn build_token_2022_metadata_instructions(
    payer: &Address,
    mint: &Address,
    mint_space: u64,
    rent_lamports: u64,
    config: TokenMetadataConfig,
) -> Result<Vec<Instruction>> {
    let _ = (payer, mint, mint_space, rent_lamports, config);
    todo!("create a Token-2022 mint with metadata stored on the mint account")
}
