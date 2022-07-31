Cryptobros - Gitpod version
==================================================

This README is specific to Gitpod and this example. For local development, please see [README.md](README.md).

## Description

In Gitpod, the cryptobros dApp will start automatically. Please look in the terminal for a link to follow.

This contract allows people follow and be followed by other people and consider bros with mutual following, backed by storage on blockchain.
Contract in `contract/src/lib.rs` provides methods to add or remove account as a bro and view current bros.

## To Test

```
cd contract
cargo test -- --nocapture
```

## To Explore

- `contract/src/lib.rs` for the contract code
- `src/index.html` for the front-end HTML
- `src/main.js` for the JavaScript front-end code and how to integrate contracts
- `src/test.js` for the JS tests for the contract


## Data collection

By using Gitpod in this project, you agree to opt-in to basic, anonymous analytics. No personal information is transmitted. Instead, these usage statistics aid in discovering potential bugs and user flow information.
