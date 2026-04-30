use mpl_core::ID as MPL_CORE_PROGRAM_ID;
use solana_keypair::Keypair;
use solana_signer::Signer;

#[test]
fn builds_metaplex_core_create_instruction() {
    let payer = Keypair::new();
    let owner = Keypair::new();
    let asset = Keypair::new();
    let config = exercise_09_metaplex_core_nft::CoreNftConfig {
        name: "Course Core Asset".to_string(),
        uri: "https://example.com/core-asset.json".to_string(),
    };

    let instruction = exercise_09_metaplex_core_nft::build_core_nft_create_instruction(
        &payer.pubkey(),
        &owner.pubkey(),
        &asset.pubkey(),
        config,
    );

    assert_eq!(instruction.program_id, MPL_CORE_PROGRAM_ID);
    assert!(
        instruction
            .accounts
            .iter()
            .any(|meta| meta.pubkey == asset.pubkey() && meta.is_signer && meta.is_writable),
        "the new asset account must sign and be writable"
    );
    assert!(
        instruction
            .accounts
            .iter()
            .any(|meta| meta.pubkey == payer.pubkey() && meta.is_signer && meta.is_writable),
        "the payer must sign and fund the asset account"
    );
}
