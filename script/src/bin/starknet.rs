use clap::Parser;
use fibonacci_script::{prepare_local_prover, write_fixture, Fixture, GNARK_IMAGE};
use sha2::{Digest, Sha256};
use sp1_sdk::blocking::{ProveRequest, Prover, ProverClient};
use sp1_sdk::{include_elf, HashableKey, ProvingKey, SP1Stdin};

#[derive(Parser)]
struct Args {
    /// Download local parameters and the Groth16 backend without generating a proof.
    #[arg(long)]
    prepare_only: bool,
    #[arg(long, default_value_t = 3)]
    n: u32,
}

fn main() {
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();
    let args = Args::parse();
    std::env::set_var("SP1_GNARK_IMAGE", GNARK_IMAGE);
    if args.prepare_only {
        prepare_local_prover();
        println!("Local Groth16 resources are ready. No proof was generated.");
        return;
    }
    assert_ne!(
        std::env::var("SP1_PROVER").unwrap_or_default(),
        "mock",
        "Mock receipts cannot be used on Starknet"
    );
    if std::env::var("SP1_PROVER").unwrap_or_else(|_| "cpu".into()) == "cpu" {
        prepare_local_prover();
    }
    let client = ProverClient::from_env();
    let pk = client.setup(include_elf!("fibonacci-program")).unwrap();
    let mut stdin = SP1Stdin::new();
    stdin.write(&args.n);
    let proof = client
        .prove(&pk, stdin)
        .groth16()
        .run()
        .expect("Groth16 proof generation failed");
    client
        .verify(&proof, pk.verifying_key(), None)
        .expect("SDK proof verification failed");
    let fixture = Fixture {
        sdk_version: "6.10.0".into(),
        circuit_version: sp1_sdk::SP1_CIRCUIT_VERSION.into(),
        guest_elf_sha256: hex::encode(Sha256::digest(&*include_elf!("fibonacci-program"))),
        vkey: pk.verifying_key().bytes32(),
        public_values: format!("0x{}", hex::encode(proof.public_values.as_slice())),
        proof: format!("0x{}", hex::encode(proof.bytes())),
    };
    fixture.verify();
    write_fixture(&fixture);
}
