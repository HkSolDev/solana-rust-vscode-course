use solana_address::Address;
use solana_signer::Signer;

#[test]
fn generated_wallet_has_matching_public_key() {
    let wallet = exercise_01_keypairs::new_wallet();
    let address = exercise_01_keypairs::wallet_address(&wallet);

    assert_eq!(address, wallet.pubkey());
    assert_ne!(address, Address::default());
}
