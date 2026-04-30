use solana_address::Address;
use solana_instruction::Instruction;
#[allow(unused_imports)]
use spl_associated_token_account_interface::{
    address::get_associated_token_address_with_program_id,
    instruction::create_associated_token_account_idempotent,
};
#[allow(unused_imports)]
use spl_token_interface::ID as TOKEN_PROGRAM_ID;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AtaPlan {
    pub address: Address,
    pub instruction: Instruction,
}

pub fn build_create_ata_plan(payer: &Address, wallet: &Address, mint: &Address) -> AtaPlan {
    let _ = (payer, wallet, mint);
    todo!("derive the wallet ATA and build the idempotent create instruction")
}
