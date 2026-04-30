use solana_address::Address;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PdaResult {
    pub address: Address,
    pub bump: u8,
}

pub fn derive_course_pda(
    program_id: &Address,
    wallet: &Address,
    label: &str,
    index: u64,
) -> PdaResult {
    let _ = (program_id, wallet, label, index);
    todo!("derive a PDA from course, label, wallet address, and little-endian index seeds")
}
