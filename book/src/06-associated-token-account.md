# Associated Token Account

Most token clients need to derive a wallet's [Associated Token Account](https://solana.com/docs/tokens/basics/create-token-account) before transferring or minting tokens.

Edit:

```text
exercises/06_associated_token_account/src/lib.rs
```

Implement `build_create_ata_plan`.

Return:

- the derived ATA address from [`get_associated_token_address_with_program_id`](https://docs.rs/spl-associated-token-account-interface/latest/spl_associated_token_account_interface/address/fn.get_associated_token_address_with_program_id.html)
- an idempotent create instruction from [`create_associated_token_account_idempotent`](https://docs.rs/spl-associated-token-account-interface/latest/spl_associated_token_account_interface/instruction/fn.create_associated_token_account_idempotent.html)

Use the idempotent create form so setup code can be safely retried.
