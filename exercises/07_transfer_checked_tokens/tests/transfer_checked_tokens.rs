use solana_keypair::Keypair;
use solana_signer::Signer;
use spl_token_interface::ID as TOKEN_PROGRAM_ID;

#[test]
fn builds_transfer_checked_instruction() -> anyhow::Result<()> {
    let source = Keypair::new();
    let mint = Keypair::new();
    let destination = Keypair::new();
    let authority = Keypair::new();

    let instruction = exercise_07_transfer_checked_tokens::build_transfer_checked_instruction(
        &source.pubkey(),
        &mint.pubkey(),
        &destination.pubkey(),
        &authority.pubkey(),
        1_500_000,
        6,
    )?;

    assert_eq!(instruction.program_id, TOKEN_PROGRAM_ID);
    assert_eq!(instruction.accounts[0].pubkey, source.pubkey());
    assert!(instruction.accounts[0].is_writable);
    assert_eq!(instruction.accounts[1].pubkey, mint.pubkey());
    assert!(!instruction.accounts[1].is_writable);
    assert_eq!(instruction.accounts[2].pubkey, destination.pubkey());
    assert!(instruction.accounts[2].is_writable);
    assert_eq!(instruction.accounts[3].pubkey, authority.pubkey());
    assert!(instruction.accounts[3].is_signer);
    assert!(!instruction.data.is_empty());

    Ok(())
}
