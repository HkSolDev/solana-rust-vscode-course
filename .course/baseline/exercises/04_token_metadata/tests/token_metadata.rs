use solana_keypair::Keypair;
use solana_signer::Signer;
use solana_system_interface::program as system_program;
use spl_token_2022_interface::ID as TOKEN_2022_PROGRAM_ID;

#[test]
fn builds_token_2022_metadata_instruction_flow() -> anyhow::Result<()> {
    let payer = Keypair::new();
    let mint = Keypair::new();
    let config = exercise_04_token_metadata::TokenMetadataConfig {
        name: "Course Token".to_string(),
        symbol: "CRSE".to_string(),
        uri: "https://example.com/course-token.json".to_string(),
        decimals: 9,
        description: "A compact Token-2022 metadata exercise".to_string(),
    };

    let instructions = exercise_04_token_metadata::build_token_2022_metadata_instructions(
        &payer.pubkey(),
        &mint.pubkey(),
        430,
        1_500_000,
        config,
    )?;

    assert_eq!(instructions.len(), 5);
    assert_eq!(instructions[0].program_id, system_program::ID);
    assert!(
        instructions[1..]
            .iter()
            .all(|instruction| instruction.program_id == TOKEN_2022_PROGRAM_ID),
        "Token metadata on the mint is handled by the Token-2022 program"
    );
    assert!(
        instructions.iter().any(|instruction| instruction
            .accounts
            .iter()
            .any(|meta| meta.pubkey == mint.pubkey())),
        "the mint should be present in the instruction accounts"
    );

    Ok(())
}
