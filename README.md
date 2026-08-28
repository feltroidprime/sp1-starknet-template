

# SP1 Starknet Template

[![Build Status](https://github.com/feltroidprime/sp1-starknet-template/workflows/Build%20Program/badge.svg)](https://github.com/feltroidprime/sp1-starknet-template/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **A complete template for building zero-knowledge applications on Starknet using SP1 zkVM**
> 
> **🔗 Based on the [SP1 Project Template](https://github.com/succinctlabs/sp1-project-template) with Starknet-specific integration**

This template demonstrates how to create an end-to-end [SP1](https://github.com/succinctlabs/sp1) project that generates proofs of RISC-V program execution and verifies them on Starknet. It includes a Fibonacci computation example with complete proof generation and on-chain verification.

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   SP1 Program   │───▶│  Proof Scripts  │───▶│ Starknet Verif. │
│   (Rust/RISC-V) │    │   (Groth16)     │    │   (Cairo)       │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

- **SP1 Program**: Rust program that computes Fibonacci numbers in the zkVM
- **Proof Scripts**: Generate and format proofs for Starknet verification ([detailed docs](script/README.md))
- **Starknet Contract**: Cairo smart contract that verifies SP1 proofs on-chain ([detailed docs](contracts/README.md))

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [SP1 Toolchain](https://docs.succinct.xyz/docs/sp1/getting-started/install)
- [Starknet Toolchain](https://github.com/software-mansion/starkup) (Scarb + Starknet Foundry)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/feltroidprime/sp1-starknet-template.git
   cd sp1-starknet-template
   ```

2. **Install SP1**
   ```bash
   curl -L https://sp1.succinct.xyz | bash
   ~/.sp1/bin/sp1up
   ```

3. **Install Starknet toolchain**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.starkup.sh | sh
   ```

## 📖 Usage

### Quick Test Workflow

```bash
# 1. Test program execution (fast)
cd script && cargo run --release -- --execute --n 10

# 2. Extract verification key
cargo run --release --bin vkey

# 3. Update contract with your verification key (see contracts/README.md)

# 4. Generate Starknet proof
cargo run --release --bin starknet -- --system groth16 --n 10

# 5. Test on-chain verification
cd ../contracts && snforge test
```

For detailed usage instructions:
- **Proof Generation**: See [script/README.md](script/README.md)
- **Contract Integration**: See [contracts/README.md](contracts/README.md)

## 📁 Project Structure

```
├── program/              # SP1 program (Rust/RISC-V)
│   └── src/main.rs      # Fibonacci computation logic
├── script/              # Proof generation scripts
│   ├── README.md        # 📚 Detailed script documentation
│   └── src/bin/         # Proof generation binaries
├── contracts/           # Starknet smart contracts (Cairo)
│   ├── README.md        # 📚 Detailed contract documentation
│   ├── src/lib.cairo    # Main verification contract
│   └── tests/           # Contract tests
├── lib/                 # Shared library
│   └── src/lib.rs       # Common types and utilities
└── .github/workflows/   # CI/CD configuration
```

## 🧪 Example: Fibonacci Computation

This template includes a complete example that:

1. **Computes Fibonacci numbers** in the SP1 zkVM
2. **Generates a zero-knowledge proof** of the computation
3. **Verifies the proof on Starknet** using a Cairo smart contract

The program takes an input `n` and computes the `n-1`th and `n`th Fibonacci numbers, proving the computation was done correctly without revealing the intermediate steps.

## 🌐 Production Deployment

### Using the Prover Network

For production use or complex computations, use the [Succinct Prover Network](https://docs.succinct.xyz/docs/network/introduction):

```bash
cp .env.example .env
# Edit .env with your network private key
SP1_PROVER=network NETWORK_PRIVATE_KEY=your_key cargo run --release --bin starknet
```

### Hardware Requirements

| Operation | RAM | Time | Documentation |
|-----------|-----|------|---------------|
| Execution | 1GB | Seconds | [script/README.md](script/README.md#performance-considerations) |
| Core Proof | 4GB | Minutes | [script/README.md](script/README.md#performance-considerations) |
| Groth16 Proof | 16GB | 10-30 min | [SP1 hardware requirements](https://docs.succinct.xyz/docs/sp1/getting-started/hardware-requirements#local-proving) |

## 🔍 Testing

Run the complete test suite:

```bash
# Test SP1 program execution
cd script && cargo run --release -- --execute

# Test proof generation  
cargo run --release -- --prove

# Test Starknet contract
cd ../contracts && snforge test
```

See component-specific testing documentation:
- [Script Testing](script/README.md#testing-and-validation)
- [Contract Testing](contracts/README.md#testing)

## 🆘 Troubleshooting

### Common Issues

- **"failed to generate proof"**: Check [script debugging guide](script/README.md#debugging)
- **"Wrong program" error**: See [contract configuration](contracts/README.md#configuration)
- **Hardware requirements**: Check [performance considerations](script/README.md#performance-considerations)

## 📚 Resources

- [SP1 Documentation](https://docs.succinct.xyz/)
- [Starknet Documentation](https://docs.starknet.io/)
- [Garaga Repository](https://github.com/keep-starknet-strange/garaga)
- [Cairo Book](https://book.cairo-lang.org/)

## 🆘 Support

- **SP1 Issues**: [SP1 GitHub Issues](https://github.com/succinctlabs/sp1/issues)
- **Template Issues**: [Create an issue](https://github.com/feltroidprime/sp1-starknet-template/issues)
- **Garaga Support**: [Garaga Support](https://garaga.gitbook.io/garaga/support)
- **Starknet Support**: [Starknet Support](https://www.starknet.io/online-communities/)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE-MIT](LICENSE-MIT) file for details.

---

**Built with ❤️ using [SP1](https://github.com/succinctlabs/sp1), [Garaga](https://github.com/keep-starknet-strange/garaga) and [Starknet](https://starknet.io/)**

---

**Based on the [SP1 Project Template](https://github.com/succinctlabs/sp1-project-template) by [Succinct Labs](https://github.com/succinctlabs) • Starknet integration by [feltroidprime](https://github.com/feltroidprime)**
