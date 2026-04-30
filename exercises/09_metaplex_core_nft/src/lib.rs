#[allow(unused_imports)]
use mpl_core::instructions::CreateV1Builder;
use solana_address::Address;
use solana_instruction::Instruction;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoreNftConfig {
    pub name: String,
    pub uri: String,
}

pub fn build_core_nft_create_instruction(
    payer: &Address,
    owner: &Address,
    asset: &Address,
    config: CoreNftConfig,
) -> Instruction {
    let _ = (payer, owner, asset, config);
    todo!("build a Metaplex Core create asset instruction")
}
