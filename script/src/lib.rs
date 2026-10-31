use garaga_rs::calldata::full_proof_with_hints::groth16::{
    get_groth16_calldata, get_sp1_vk, Groth16Proof,
};
use garaga_rs::definitions::CurveID;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const GNARK_IMAGE: &str = "ghcr.io/succinctlabs/sp1-gnark@sha256:fd3e39f4995735567932b145580c70f87552dec0181f14a6898451a7ec5fa409";

#[cfg(feature = "prover")]
pub fn prepare_local_prover() {
    // Ces ressources doivent être disponibles avant le calcul coûteux du reçu wrap.
    let present = std::process::Command::new("docker")
        .args(["image", "inspect", GNARK_IMAGE])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .expect("Docker is required for local Groth16 proofs");
    if !present.success() {
        assert!(std::process::Command::new("docker")
            .args(["pull", GNARK_IMAGE])
            .status()
            .unwrap()
            .success());
    }
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(sp1_sdk::install::try_install_circuit_artifacts("groth16"))
        .expect("Groth16 parameter preparation failed");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fixture {
    pub sdk_version: String,
    pub circuit_version: String,
    pub guest_elf_sha256: String,
    pub vkey: String,
    #[serde(rename = "publicValues")]
    pub public_values: String,
    pub proof: String,
}

impl Fixture {
    pub fn bytes(&self, value: &str) -> Vec<u8> {
        hex::decode(value.strip_prefix("0x").unwrap()).unwrap()
    }

    pub fn verify(&self) {
        assert_eq!(self.bytes(&self.proof).len(), 356);
        assert_eq!(self.bytes(&self.public_values).len(), 96);
        // Le reçu ne doit jamais être accepté uniquement parce que ses métadonnées semblent valides.
        let proof = Groth16Proof::from_sp1(
            self.bytes(&self.vkey),
            self.bytes(&self.public_values),
            self.bytes(&self.proof),
        );
        get_groth16_calldata(&proof, &get_sp1_vk(), CurveID::BN254)
            .expect("Garaga rejected receipt");
    }

    pub fn calldata(&self) -> String {
        let proof = Groth16Proof::from_sp1(
            self.bytes(&self.vkey),
            self.bytes(&self.public_values),
            self.bytes(&self.proof),
        );
        get_groth16_calldata(&proof, &get_sp1_vk(), CurveID::BN254)
            .unwrap()
            .iter()
            .skip(1)
            .map(|n| format!("0x{n:x}\n"))
            .collect()
    }
}

pub fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../contracts/src/fixtures")
}

pub fn write_fixture(fixture: &Fixture) {
    let path = fixtures_dir();
    std::fs::create_dir_all(&path).unwrap();
    std::fs::write(
        path.join("groth16-fixture.json"),
        serde_json::to_string_pretty(fixture).unwrap() + "\n",
    )
    .unwrap();
    std::fs::write(path.join("groth16-calldata.txt"), fixture.calldata()).unwrap();
}

#[cfg(test)]
mod receipt_tests;
