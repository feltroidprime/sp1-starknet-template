# Dependency audit

On October 31, 2026, `cargo audit` reported four advisories in the pinned SP1 SDK dependency graph.
The template does not suppress these advisories.
The audit command exits with a failure status.

- [`RUSTSEC-2026-0387`](https://rustsec.org/advisories/RUSTSEC-2026-0387.html) affects `crypto-bigint 0.5.5` through SP1's native curve dependencies.
  Its correction requires `crypto-bigint >=0.7.6`, outside the SDK dependency constraints.
  The default host build includes the affected package.
  The advisory concerns timing behavior with secret integers.
- [`RUSTSEC-2026-0391`](https://rustsec.org/advisories/RUSTSEC-2026-0391.html) affects `zstd 0.13.3`.
  Its correction requires `zstd >=0.14.3`.
- [`RUSTSEC-2026-0398`](https://rustsec.org/advisories/RUSTSEC-2026-0398.html) and [`RUSTSEC-2026-0403`](https://rustsec.org/advisories/RUSTSEC-2026-0403.html) affect `zstd-safe 7.3.0`.
  Their corrections require `zstd-safe >=8.2.0`.

The compression packages enter through SP1's optional `network` feature.
They are absent from the default native build.
The SDK currently calls `zstd::encode_all` for network uploads.
The advisories describe context reuse, seekable decompression, and legacy streaming decompression.
This call-site observation does not establish safety for all SDK operations or future changes.

The Cairo application does not contain these Rust dependencies.
A newer compatible SP1 SDK must correct the dependency constraints before the host audit can pass without a fork.

Reproduce the dependency paths with:

```bash
cargo audit
cargo tree --locked -i crypto-bigint
cargo tree --locked --features network -i zstd
cargo tree --locked --features network -i zstd-safe
```
