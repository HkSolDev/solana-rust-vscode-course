use solana_keypair::Keypair;
use solana_signer::Signer;
use spl_associated_token_account_interface::program as associated_token_program;

#[test]
fn derives_ata_and_builds_create_instruction() {
    let payer = Keypair::new();
    let wallet = Keypair::new();
    let mint = Keypair::new();

    let plan = exercise_06_associated_token_account::build_create_ata_plan(
        &payer.pubkey(),
        &wallet.pubkey(),
        &mint.pubkey(),
    );

    assert_eq!(plan.instruction.program_id, associated_token_program::ID);
    assert_eq!(plan.instruction.accounts[0].pubkey, payer.pubkey());
    assert!(plan.instruction.accounts[0].is_signer);
    assert_eq!(plan.instruction.accounts[1].pubkey, plan.address);
    assert!(plan.instruction.accounts[1].is_writable);
    assert_eq!(plan.instruction.accounts[2].pubkey, wallet.pubkey());
    assert_eq!(plan.instruction.accounts[3].pubkey, mint.pubkey());
    assert_eq!(
        plan.instruction.data,
        vec![1],
        "use the idempotent create form"
    );
}
