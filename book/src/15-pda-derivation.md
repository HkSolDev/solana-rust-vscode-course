# PDA Derivation

[Program Derived Addresses](https://solana.com/docs/core/pda) are deterministic account addresses controlled by a program. Clients derive them before reading or writing program state.

Edit:

```text
exercises/15_pda_derivation/src/lib.rs
```

Implement `derive_course_pda`.

Use these seeds in order:

- `b"course"`
- `label.as_bytes()`
- `wallet.as_ref()`
- `index.to_le_bytes()`

Return both the derived `Address` and its bump from [`Address::find_program_address`](https://docs.rs/solana-address/latest/solana_address/struct.Address.html#method.find_program_address).
