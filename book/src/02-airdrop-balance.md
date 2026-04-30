# Airdrops and Balances

This exercise connects to local Surfpool RPC, requests an airdrop, confirms it, and returns the recipient balance.

The recipient is an `Address`, the current Solana account-address type. Older examples often call this a public key or `Pubkey`; in this course, use `Address` for account addresses and reserve keypair language for signing keys.

Edit:

```text
exercises/02_airdrop_balance/src/lib.rs
```

Implement `request_airdrop_and_get_balance`.
