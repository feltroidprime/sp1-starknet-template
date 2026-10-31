use sp1_sdk::blocking::{Prover, ProverClient};
use sp1_sdk::{include_elf, HashableKey, ProvingKey};

fn main() {
    let client = ProverClient::builder().light().build();
    let pk = client.setup(include_elf!("fibonacci-program")).unwrap();
    println!("{}", pk.verifying_key().bytes32());
}
