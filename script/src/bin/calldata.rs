use fibonacci_script::{fixtures_dir, write_fixture, Fixture};

fn main() {
    let fixture: Fixture = serde_json::from_str(
        &std::fs::read_to_string(fixtures_dir().join("groth16-fixture.json")).unwrap(),
    )
    .unwrap();
    fixture.verify();
    write_fixture(&fixture);
}
