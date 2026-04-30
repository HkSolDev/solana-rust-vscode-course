# Wallet File

Solana CLI keypair files are JSON arrays containing the 64 bytes of a keypair. Client code often needs to load one, derive the account address, and avoid logging the secret material.

Edit:

```text
exercises/11_wallet_file/src/lib.rs
```

Implement:

- `load_keypair_file`
- `wallet_address_from_file`

Use [`solana_keypair::read_keypair_file`](https://docs.rs/solana-keypair/latest/solana_keypair/fn.read_keypair_file.html) instead of parsing the bytes by hand. Return the `Address` with the [`Signer`](https://docs.rs/solana-signer/latest/solana_signer/trait.Signer.html) trait's `pubkey()` method.
