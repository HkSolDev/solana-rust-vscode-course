# Metaplex Core NFT

Metaplex Core represents digital assets directly as Core assets rather than as a classic SPL mint plus metadata account. For a client-side Rust course, the first useful step is learning to build the create-asset instruction.

Edit:

```text
exercises/09_metaplex_core_nft/src/lib.rs
```

Implement `build_core_nft_create_instruction`.

The exercise uses [`mpl_core::instructions::CreateV1Builder`](https://docs.rs/mpl-core/latest/mpl_core/instructions/struct.CreateV1Builder.html). The builder produces a Solana instruction that can be added to a normal transaction with the payer and asset keypairs as signers.

The test checks the shape of the instruction: it must target the Metaplex Core program, mark the new asset as writable and signing, and include the payer as a writable signer.
