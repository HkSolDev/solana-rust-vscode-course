use solana_address::Address;
use solana_keypair::Keypair;
use solana_signer::Signer;

#[test]
fn derives_deterministic_pda_with_bump() {
    let program = Keypair::new();
    let wallet = Keypair::new();
    let index = 42_u64;
    let index_bytes = index.to_le_bytes();

    let first = exercise_15_pda_derivation::derive_course_pda(
        &program.pubkey(),
        &wallet.pubkey(),
        "vault",
        index,
    );
    let second = exercise_15_pda_derivation::derive_course_pda(
        &program.pubkey(),
        &wallet.pubkey(),
        "vault",
        index,
    );
    let recreated = Address::create_program_address(
        &[
            b"course",
            b"vault",
            wallet.pubkey().as_ref(),
            &index_bytes,
            &[first.bump],
        ],
        &program.pubkey(),
    )
    .expect("bump should recreate the PDA");

    assert_eq!(first, second);
    assert_eq!(first.address, recreated);
    assert_ne!(first.address, wallet.pubkey());
}
