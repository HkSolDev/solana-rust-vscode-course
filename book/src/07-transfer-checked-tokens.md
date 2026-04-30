# Transfer Checked Tokens

Token transfers should usually use the checked form because it includes the mint and decimal count in the instruction.

Edit:

```text
exercises/07_transfer_checked_tokens/src/lib.rs
```

Implement `build_transfer_checked_instruction`.

Build an SPL Token [`transfer_checked`](https://docs.rs/spl-token-interface/latest/spl_token_interface/instruction/fn.transfer_checked.html) instruction for:

- source token account
- mint
- destination token account
- owner authority
- raw amount
- decimals

Use an empty multisig signer slice for this single-authority exercise.
