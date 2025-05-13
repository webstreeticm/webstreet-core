
# WebStreet Core (Solana Anchor Version)

This repo contains the smart contracts for the WebStreet protocol on Solana, including:

- SPL-compatible token staking system
- Future extensibility for AI strategy registry

## Features

- Anchor-based Rust program
- `initialize`: Sets up a user staking account
- `stake`: Stake SPL tokens (off-chain logic to transfer in)
- `unstake`: Unstake tokens with balance check

## Project Structure

- `programs/webstreet_core` - Anchor program in Rust
- `tests/` - JavaScript/TypeScript Anchor test hooks (TBD)
- `migrations/` - Anchor deployment scripts

## Getting Started

```bash
anchor build
anchor test
```

## License

MIT © WebStreet Labs
