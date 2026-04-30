# Confirm Signature

Submitting a transaction is not the same as knowing it landed. A client should keep the [`Signature`](https://docs.rs/solana-signature/latest/solana_signature/struct.Signature.html) and confirm it at the commitment level the workflow needs.

Edit:

```text
exercises/13_confirm_signature/src/lib.rs
```

Implement `request_airdrop_and_confirm`.

Request the airdrop, call [`confirm_transaction_with_commitment`](https://docs.rs/solana-client/latest/solana_client/rpc_client/struct.RpcClient.html#method.confirm_transaction_with_commitment), and return an error if the confirmation response is false. The test reads the balance only after your function has confirmed the signature.
