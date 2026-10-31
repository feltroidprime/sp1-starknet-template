use alloy_sol_types::SolType;
use clap::Parser;
use fibonacci_lib::{fibonacci, PublicValuesStruct};
use sp1_sdk::blocking::{ProveRequest, Prover, ProverClient};
use sp1_sdk::{include_elf, Elf, ProvingKey, SP1Stdin};

const ELF: Elf = include_elf!("fibonacci-program");

#[derive(Parser)]
struct Args {
    #[arg(long, conflicts_with = "prove", required_unless_present = "prove")]
    execute: bool,
    #[arg(long)]
    prove: bool,
    #[arg(long, default_value_t = 20)]
    n: u32,
}

fn main() {
    dotenv::dotenv().ok();
    let args = Args::parse();
    let mut stdin = SP1Stdin::new();
    stdin.write(&args.n);
    if args.execute {
        let client = ProverClient::builder().light().build();
        let (output, report) = client.execute(ELF, stdin).run().unwrap();
        let decoded = PublicValuesStruct::abi_decode(output.as_slice()).unwrap();
        let (a, b) = fibonacci(args.n);
        assert_eq!((decoded.n, decoded.a, decoded.b), (args.n, a, b));
        println!(
            "n={}, F(n)={}, F(n+1)={}, cycles={}",
            args.n,
            a,
            b,
            report.total_instruction_count()
        );
    } else {
        let client = ProverClient::from_env();
        let pk = client.setup(ELF).unwrap();
        let proof = client
            .prove(&pk, stdin)
            .run()
            .expect("core proof generation failed");
        client
            .verify(&proof, pk.verifying_key(), None)
            .expect("core proof verification failed");
        println!("Core proof verified. Use --bin starknet for a Groth16 receipt.");
    }
}
