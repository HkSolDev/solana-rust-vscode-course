# RPC Client Basics

Most Rust clients start by choosing an RPC endpoint and a commitment level. This exercise keeps that setup explicit.

Edit:

```text
exercises/10_rpc_client_basics/src/lib.rs
```

Implement:

- `confirmed_client`
- `fetch_rpc_status`

Use [`RpcClient::new_with_commitment`](https://docs.rs/solana-client/latest/solana_client/rpc_client/struct.RpcClient.html#method.new_with_commitment) with [`CommitmentConfig::confirmed()`](https://docs.rs/solana-commitment-config/latest/solana_commitment_config/struct.CommitmentConfig.html#method.confirmed). Then read basic node state with `get_slot` and `get_version`, and copy the results into `RpcStatus`.
