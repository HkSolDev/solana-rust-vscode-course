# Mint Tokens

After creating a Token-2022 mint and its metadata, clients usually need to mint supply into a token account.

Edit:

```text
exercises/05_mint_to_tokens/src/lib.rs
```

Implement `build_mint_to_checked_instruction`.

Use the Token-2022 [`mint_to_checked`](https://docs.rs/spl-token-2022-interface/latest/spl_token_2022_interface/instruction/fn.mint_to_checked.html) instruction. The checked form includes the mint decimals in the instruction, which helps clients avoid silently minting with the wrong precision.

The instruction needs:

- mint address
- destination token account
- mint authority
- raw amount
- decimals

This exercise only builds the instruction. A later client flow can combine mint creation, ATA creation, and minting into one transaction.
