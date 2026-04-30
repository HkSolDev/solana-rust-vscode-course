# Keypairs

Solana clients sign transactions with [keypairs](https://docs.rs/solana-keypair/latest/solana_keypair/struct.Keypair.html). In current Rust client code, a keypair exposes an account [`Address`](https://docs.rs/solana-address/latest/solana_address/struct.Address.html). In this exercise, create a new wallet and return its address.

Edit:

```text
exercises/01_keypairs/src/lib.rs
```

Implement:

- `new_wallet`
- `wallet_address`
