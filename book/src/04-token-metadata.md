# SPL Token With Metadata

This exercise builds the instruction flow for a Token-2022 mint that stores metadata on the mint account itself.

Edit:

```text
exercises/04_token_metadata/src/lib.rs
```

Implement `build_token_2022_metadata_instructions`.

The compact flow is:

- Create the mint account owned by the Token-2022 program.
- Initialize the [`MetadataPointer`](https://docs.rs/spl-token-2022-interface/latest/spl_token_2022_interface/extension/metadata_pointer/struct.MetadataPointer.html) extension so the mint points at itself.
- Initialize the mint.
- Initialize token metadata with name, symbol, and URI.
- Add a custom `description` field with the [token metadata interface](https://docs.rs/spl-token-metadata-interface/latest/spl_token_metadata_interface/).

In a full client, the caller calculates the mint account space and rent before building the instructions. The exercise keeps those values as parameters so the first pass stays focused on instruction composition.
