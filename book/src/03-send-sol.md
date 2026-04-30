# Send SOL

This exercise turns the airdrop lesson into a real transaction. The learner builds a System Program [`transfer`](https://docs.rs/solana-system-interface/latest/solana_system_interface/instruction/fn.transfer.html), signs it with the payer, submits it through [`RpcClient`](https://docs.rs/solana-client/latest/solana_client/rpc_client/struct.RpcClient.html), and confirms the returned signature.

Edit:

```text
exercises/03_send_sol/src/lib.rs
```

Implement `send_lamports`.

The important client-side pieces are:

- `system_instruction::transfer` to create the lamport transfer instruction.
- [`client.get_latest_blockhash()`](https://docs.rs/solana-client/latest/solana_client/rpc_client/struct.RpcClient.html#method.get_latest_blockhash) to make the transaction recent.
- [`Transaction::new_signed_with_payer`](https://docs.rs/solana-transaction/latest/solana_transaction/struct.Transaction.html#method.new_signed_with_payer) to sign with the payer wallet.
- [`client.send_and_confirm_transaction`](https://docs.rs/solana-client/latest/solana_client/rpc_client/struct.RpcClient.html#method.send_and_confirm_transaction) to submit and wait for confirmation.

The test creates two fresh local wallets, funds the payer through Surfpool, then checks that the recipient received the transferred lamports.
