# Contract verification

`HelloStarknet` calls the Garaga SP1 v6 verifier through a library call.
The constructor fixes its verifier class hash and program verification key.
There are no methods that change these values.

The entry point returns `Ok(public_values)` after proof verification and the program-key comparison.
It returns `Err(error)` for the verifier's explicit validation errors.
Malformed proofs and failed pairing checks can revert.
The public values contain `(n, F(n), F(n+1))` as `u256` words.

## Offline tests

Use Scarb 2.21.0 and Starknet Foundry 0.64.1.

```bash
cd contracts
snforge test
```

The tests declare `UniversalECIP` and `SP1Groth16VerifierBN254` locally.
They do not fork Mainnet or Sepolia.
They verify the stored Fibonacci receipt and its exact public values.
They also reject another program key, a failed guest, a wrong recursion root, and a changed nonce.

`verifier/src/groth16_verifier.cairo` is an unmodified copy from the pinned Garaga commit.
The standalone package retains the upstream package name, module path, and compiler options.
The Git resolver does not expose this generated package directly.
The integration check compares its source with upstream.
The devnet check also compares both compiled class hashes with `integration.json`.

## Isolated devnet

Install starknet-devnet 0.10.1 and `uv`.
Add the devnet binary, Scarb, and Foundry to `PATH`.
Run this command from the repository root:

```bash
uv run --locked scripts/devnet.py
```

The script compiles the release artifacts and checks both verifier hashes.
It starts its own devnet on a local port, declares all classes, and deploys the application.
It then verifies the stored receipt through the deployed application and checks the exact outputs.
The script stops the devnet after completion.
It accepts no remote RPC URL or private key.

## Live network prerequisites

On October 31, 2026, RPC reads found these classes absent from both Mainnet and Sepolia:

- SP1 v6: `0x02f5bde36cc36fe00ca97d181dd3c1a26bb92076818d2304c329488608e0db1f`.
- UniversalECIP: `0x20dc39fc30f4d62db17cc86a911f1f27135d255357ce99d3b40e2da78f800dd`.

The providers returned error 28, `Class hash not found`.
Older known classes existed on the same providers.
The reads used Mainnet block 17232846 and Sepolia block 17433642.
Explicit declarations of both classes are necessary before deployment.
The Garaga release pipeline checks availability but does not submit these declarations.
The published older classes cannot verify this receipt.

After explicit authorization for live transactions, an operator can declare the release artifacts.
Build and inspect them first:

```bash
scarb --manifest-path contracts/verifier/Scarb.toml --release build
scarb --manifest-path contracts/Scarb.toml --release build
sncast utils class-hash --sierra-file contracts/verifier/target/release/sp1_verifier_bn254_UniversalECIP.contract_class.json
sncast utils class-hash --sierra-file contracts/verifier/target/release/sp1_verifier_bn254_SP1Groth16VerifierBN254.contract_class.json
```

Configure a separate sncast account and RPC for the intended chain.
From `contracts/verifier`, estimate the declarations before submission:

```bash
sncast --account "$ACCOUNT" --scarb-profile release declare --contract-name UniversalECIP --url "$RPC_URL" --dry-run
sncast --account "$ACCOUNT" --scarb-profile release declare --contract-name SP1Groth16VerifierBN254 --url "$RPC_URL" --dry-run
```

After approval, submit the declarations in this order:

```bash
sncast --account "$ACCOUNT" --scarb-profile release --wait declare --contract-name UniversalECIP --url "$RPC_URL"
sncast --account "$ACCOUNT" --scarb-profile release --wait declare --contract-name SP1Groth16VerifierBN254 --url "$RPC_URL"
```

These commands use the configured sncast account file.
`ACCOUNT` identifies the approved account, and `RPC_URL` identifies the approved chain.
Declare and deploy `HelloStarknet` with the SP1 class hash and the key from `cargo run --release --locked --bin vkey`.
The calldata file contains the array elements without the outer ABI length.
Serialize this array for `verify_sp1_proof`; the devnet script shows the exact payload.
Live declarations and deployment spend funds.
Neither CI nor the devnet script performs these transactions.
