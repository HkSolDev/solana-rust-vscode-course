use solana_keypair::Keypair;
use solana_signer::Signer;
use spl_token_interface::ID as TOKEN_PROGRAM_ID;

#[test]
fn builds_close_token_account_instruction() -> anyhow::Result<()> {
    let token_account = Keypair::new();
    let destination = Keypair::new();
    let owner = Keypair::new();

    let instruction = exercise_08_close_token_account::build_close_token_account_instruction(
        &token_account.pubkey(),
        &destination.pubkey(),
        &owner.pubkey(),
    )?;

    assert_eq!(instruction.program_id, TOKEN_PROGRAM_ID);
    assert_eq!(instruction.accounts[0].pubkey, token_account.pubkey());
    assert!(instruction.accounts[0].is_writable);
    assert_eq!(instruction.accounts[1].pubkey, destination.pubkey());
    assert!(instruction.accounts[1].is_writable);
    assert_eq!(instruction.accounts[2].pubkey, owner.pubkey());
    assert!(instruction.accounts[2].is_signer);
    assert!(!instruction.data.is_empty());

    Ok(())
}
