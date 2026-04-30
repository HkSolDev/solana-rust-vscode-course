# Account Info

Before decoding anything fancy, a client should be able to fetch an account and inspect the basic fields the runtime stores for every account.

Edit:

```text
exercises/12_account_info/src/lib.rs
```

Implement `fetch_account_snapshot`.

Use [`get_account_with_commitment`](https://docs.rs/solana-client/latest/solana_client/rpc_client/struct.RpcClient.html#method.get_account_with_commitment), handle the missing-account case, and return:

- lamports
- owner
- executable flag
- data length

This is the shape you will reuse before decoding token accounts, program state, or NFT assets.
