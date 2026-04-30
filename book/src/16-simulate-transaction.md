# Simulate Transaction

Simulation is the cheapest way to catch many client-side mistakes before sending a transaction for real.

Edit:

```text
exercises/16_simulate_transaction/src/lib.rs
```

Implement `simulate_lamport_transfer`.

Build a normal System Program transfer transaction, sign it, call [`simulate_transaction`](https://docs.rs/solana-client/latest/solana_client/rpc_client/struct.RpcClient.html#method.simulate_transaction), and summarize whether it succeeded, the logs, and any compute units consumed.

The transaction is not sent in this exercise. It is only simulated.
