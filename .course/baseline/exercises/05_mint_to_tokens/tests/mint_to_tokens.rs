use solana_keypair::Keypair;
use solana_signer::Signer;
use spl_token_2022_interface::ID as TOKEN_2022_PROGRAM_ID;

#[test]
fn builds_token_2022_mint_to_checked_instruction() -> anyhow::Result<()> {
    let mint = Keypair::new();
    let destination = Keypair::new();
    let mint_authority = Keypair::new();

    let instruction = exercise_05_mint_to_tokens::build_mint_to_checked_instruction(
        &mint.pubkey(),
        &destination.pubkey(),
        &mint_authority.pubkey(),
        42_000_000,
        6,
    )?;

    assert_eq!(instruction.program_id, TOKEN_2022_PROGRAM_ID);
    assert_eq!(instruction.accounts[0].pubkey, mint.pubkey());
    assert!(instruction.accounts[0].is_writable);
    assert_eq!(instruction.accounts[1].pubkey, destination.pubkey());
    assert!(instruction.accounts[1].is_writable);
    assert_eq!(instruction.accounts[2].pubkey, mint_authority.pubkey());
    assert!(instruction.accounts[2].is_signer);
    assert!(!instruction.data.is_empty());

    Ok(())
}
