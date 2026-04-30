# Rent-Exempt Account

When a client creates an account directly, it must fund the new account with enough lamports for rent exemption.

Edit:

```text
exercises/14_rent_create_account/src/lib.rs
```

Implement `create_rent_exempt_account`.

The flow is:

- ask RPC for [`get_minimum_balance_for_rent_exemption(space)`](https://docs.rs/solana-client/latest/solana_client/rpc_client/struct.RpcClient.html#method.get_minimum_balance_for_rent_exemption)
- build a System Program [`create_account`](https://docs.rs/solana-system-interface/latest/solana_system_interface/instruction/fn.create_account.html) instruction
- fetch a latest blockhash
- sign with both the payer and the new account
- send and confirm the transaction

This is a useful building block for clients that create non-token accounts or program-owned state.
