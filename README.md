# SP1 Starknet Template

This template proves a Fibonacci computation with SP1 and verifies the receipt on Starknet through Garaga.
The guest commits `(n, F(n), F(n+1))` as three ABI-encoded 32-byte words.
The computation uses wrapping `u32` arithmetic.

## Compatibility

The pinned SDK is **SP1 6.10.0**, which uses circuit **v6.1.0**.
The template requires SHA-256 receipts with 96 public-value bytes.
Blake3 receipts and SP1 v4/v5 receipts are incompatible.

`integration.json` records the integration versions and expected verifier class hashes.
Rust and Cairo dependencies pin an immutable Garaga commit from [Garaga PR #589](https://github.com/keep-starknet-strange/garaga/pull/589).
The published Garaga `1.1.0` package does not identify this integration reliably.
After a compatible release, update both dependency manifests and lockfiles together.

## Install

Install Rust through rustup, Docker for reproducible guest builds and local Groth16 proofs, and a protobuf compiler (`apt install protobuf-compiler` on Debian).
Install `uv` for the integration and devnet scripts.
The repository pins Rust 1.91.1.
The guest build uses the official SP1 Docker image with an immutable digest.
A local SP1 compiler installation is unnecessary for the normal Cargo commands.

If you need the standalone `cargo prove` CLI, install the pinned toolchain:

```bash
bash scripts/install-sp1.sh
export PATH="$HOME/.sp1/bin:$PATH"
```

Install Scarb 2.21.0 and Starknet Foundry 0.64.1.
Foundry requires Universal Sierra Compiler 2.10.0 or newer.

## Verify the stored receipt

These commands compile the guest and verify the committed receipt.
They do not generate a proof or require a public Starknet RPC endpoint.
The first run downloads compiler dependencies and the guest build image.

```bash
cargo test --release --locked --workspace --exclude fibonacci-program
cargo run --release --locked --bin fibonacci -- --execute --n 10
uv run --no-project python scripts/check-integration.py
cd contracts
snforge test
```

The Rust test derives the program key from the compiled guest.
It compares this key with the stored receipt and verifies the receipt with the official SDK verifier.
It also checks the public outputs, Garaga calldata, and rejection of altered receipts.
The Cairo tests declare both verifier classes locally and exercise the application contract.
CI runs these checks without repeated proof generation.

## Generate a new receipt

Generate a receipt only after a guest change or when another input is necessary.
Local Groth16 proof generation requires Docker and substantial memory.
The first local proof also needs a roughly 6.2 GB parameter archive, plus space for extraction.
The generator prepares these parameters before it starts the expensive computation.

```bash
SP1_PROVER=cpu cargo run --release --locked --bin starknet -- --n 3
cargo test --release --locked --workspace --exclude fibonacci-program
(cd contracts && snforge test)
```

The generator verifies the receipt before it replaces the JSON and calldata files.
The receipt records its SDK version, circuit version, and guest ELF SHA-256.
The committed contract test expects input `3` and outputs `(2, 3)`.
Update that test if you change the fixture input.
Mock receipts cannot pass the independent verification tests.

For the Succinct network, compile with `--features network` and configure `.env`.
Network proof requests can incur charges.
See [script instructions](script/README.md) for the commands.

## Deploy

The application constructor accepts an immutable verifier class hash and an immutable program verification key.
The verifier performs the cryptographic check before the application compares the program key.
The entry point returns `Result<Span<u256>, felt252>`.
Malformed proofs can revert.

As of October 31, 2026, RPC reads found neither new verifier class on Mainnet or Sepolia.
Both calls returned `Class hash not found`.
The older published verifier classes cannot verify these receipts.
Explicit declarations of both new classes are necessary before live deployment.
The Garaga release pipeline checks class availability but does not submit declarations.
No live transaction forms part of the tests or CI.
See [contract instructions](contracts/README.md) for local deployment and declaration commands.

## Dependency audit

The pinned SDK has four upstream advisories in its lockfile.
One affects the default native graph; three concern the optional network compression dependencies.
See [the audit notes](SECURITY.md) for scope, dependency paths, and correction constraints.

## Layout

- `program/`: SP1 guest.
- `lib/`: Fibonacci computation and public-value ABI.
- `script/`: execution, proof generation, key extraction, and receipt verification.
- `contracts/`: application and offline Cairo tests.
- `contracts/verifier/`: unmodified verifier source from the pinned Garaga commit.
- `contracts/src/fixtures/`: committed receipt and derived calldata.

The verifier source carries the Garaga MIT license.
`scripts/check-integration.py` compares it with the pinned upstream source.

[MIT license](LICENSE-MIT)
