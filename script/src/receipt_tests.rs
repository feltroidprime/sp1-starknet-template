use crate::{fixtures_dir, Fixture};
use alloy_sol_types::SolType;
use fibonacci_lib::{fibonacci, PublicValuesStruct};
use sha2::{Digest, Sha256};
use sp1_sdk::blocking::{Prover, ProverClient};
use sp1_sdk::{include_elf, HashableKey, ProvingKey, SP1Stdin};

fn fixture() -> Fixture {
    serde_json::from_str(
        &std::fs::read_to_string(fixtures_dir().join("groth16-fixture.json")).unwrap(),
    )
    .unwrap()
}

#[test]
fn stored_receipt_matches_guest_and_calldata() {
    let fixture = fixture();
    let integration: serde_json::Value =
        serde_json::from_str(include_str!("../../integration.json")).unwrap();
    assert_eq!(fixture.sdk_version, "6.10.0");
    assert_eq!(fixture.sdk_version, integration["sp1_sdk"]);
    assert_eq!(fixture.circuit_version, integration["sp1_circuit"]);
    assert_eq!(fixture.circuit_version, sp1_sdk::SP1_CIRCUIT_VERSION);
    assert_eq!(
        fixture.guest_elf_sha256,
        hex::encode(Sha256::digest(&*include_elf!("fibonacci-program")))
    );
    let client = ProverClient::builder().light().build();
    let pk = client.setup(include_elf!("fibonacci-program")).unwrap();
    assert_eq!(
        fixture.vkey,
        pk.verifying_key().bytes32(),
        "Rebuild and regenerate the receipt after guest changes"
    );
    let public_values = fixture.bytes(&fixture.public_values);
    let values = PublicValuesStruct::abi_decode(&public_values).unwrap();
    assert_eq!((values.n, values.a, values.b), (3, 2, 3));
    assert_eq!(fibonacci(values.n), (values.a, values.b));
    sp1_verifier::Groth16Verifier::verify(
        &fixture.bytes(&fixture.proof),
        &public_values,
        &fixture.vkey,
        &sp1_verifier::GROTH16_VK_BYTES,
    )
    .unwrap();
    fixture.verify();
    assert_eq!(
        fixture.calldata(),
        std::fs::read_to_string(fixtures_dir().join("groth16-calldata.txt")).unwrap()
    );
}

#[test]
fn sdk_rejects_modified_receipt_and_public_values() {
    let fixture = fixture();
    let raw = fixture.bytes(&fixture.proof);
    let public_values = fixture.bytes(&fixture.public_values);
    for offset in [0, 35, 67, 99, 355] {
        let mut changed = raw.clone();
        changed[offset] ^= 1;
        assert!(sp1_verifier::Groth16Verifier::verify(
            &changed,
            &public_values,
            &fixture.vkey,
            &sp1_verifier::GROTH16_VK_BYTES
        )
        .is_err());
    }
    for length in [0, 3, 260, 355] {
        assert!(sp1_verifier::Groth16Verifier::verify(
            &raw[..length],
            &public_values,
            &fixture.vkey,
            &sp1_verifier::GROTH16_VK_BYTES,
        )
        .is_err());
    }
    let mut changed = public_values;
    changed[31] ^= 1;
    assert!(sp1_verifier::Groth16Verifier::verify(
        &raw,
        &changed,
        &fixture.vkey,
        &sp1_verifier::GROTH16_VK_BYTES
    )
    .is_err());
}

#[test]
fn guest_execution_matches_fibonacci() {
    let client = ProverClient::builder().light().build();
    let mut stdin = SP1Stdin::new();
    stdin.write(&10_u32);
    let (output, report) = client
        .execute(include_elf!("fibonacci-program"), stdin)
        .run()
        .unwrap();
    let values = PublicValuesStruct::abi_decode(output.as_slice()).unwrap();
    assert_eq!((values.n, values.a, values.b), (10, 55, 89));
    assert_eq!(report.exit_code, 0);
    println!(
        "Guest executed: n=10, F(n)=55, F(n+1)=89, cycles={}",
        report.total_instruction_count()
    );
}
