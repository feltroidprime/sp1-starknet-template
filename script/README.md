# Proof scripts

Run these commands from the repository root.
The guest commits `n`, `F(n)`, and `F(n+1)` as 96 ABI-encoded bytes.

```bash
cargo run --release --locked --bin fibonacci -- --execute --n 10
cargo run --release --locked --bin vkey
```

The first command executes the guest and checks its output.
The second command prints the program verification key for the application constructor.

## Local parameter preparation

This command downloads the roughly 6.2 GB parameter archive and the pinned Groth16 image:

```bash
cargo run --release --locked --bin starknet -- --prepare-only
```

It does not generate a proof.
The CPU generator also runs this preparation automatically before the expensive computation.
Network generation does not need these local resources.

## Proof generation

This command generates and verifies one real Groth16 receipt.
It then replaces `contracts/src/fixtures/groth16-fixture.json` and `groth16-calldata.txt`.
Docker must run for the default Groth16 backend.

```bash
SP1_PROVER=cpu cargo run --release --locked --bin starknet -- --n 3
```

SP1 SDK 6.10.0 uses circuit v6.1.0.
The receipt contains 356 bytes: selector, exit code, recursion root, nonce, and Groth16 points.
The selector is `4388a21c`.
Garaga requires exit code zero, its expected recursion root, and complete 32-byte public-value words.

## Network prover

Network requests can incur charges.
Configure a network private key in an untracked `.env` file.

```bash
cp .env.example .env
# Set SP1_PROVER=network and NETWORK_PRIVATE_KEY in .env.
cargo run --release --locked --features network --bin starknet -- --n 3
```

## Receipt verification

```bash
cargo test --release --locked --workspace --exclude fibonacci-program
```

The tests compile the guest and compare its key with the committed receipt.
They verify the receipt through `sp1-verifier` 6.10.0 and Garaga.
They also compare the generated calldata with the committed calldata.
CI runs these tests and never requests new proofs.

To regenerate only calldata from the stored receipt, run:

```bash
cargo run --release --locked --bin calldata
```

This command does not run a prover.
It excludes the outer array length from the calldata file.
The application adds this length during ABI serialization.
