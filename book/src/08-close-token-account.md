# Close Token Account

Closing an empty token account returns its rent lamports to a destination account. Clients do this during cleanup or when removing unused ATAs.

Edit:

```text
exercises/08_close_token_account/src/lib.rs
```

Implement `build_close_token_account_instruction`.

The token account and destination must be writable. The owner authority signs. Build the SPL Token [`close_account`](https://docs.rs/spl-token-interface/latest/spl_token_interface/instruction/fn.close_account.html) instruction; the check verifies the instruction shape and does not need a live token account.
