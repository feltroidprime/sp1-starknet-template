use snforge_std::fs::{FileTrait, read_txt};
use snforge_std::{ContractClassTrait, DeclareResultTrait, declare};
use sp1_app::{IHelloStarknetDispatcher, IHelloStarknetDispatcherTrait};

fn fixture() -> Array<felt252> {
    read_txt(@FileTrait::new("src/fixtures/groth16-calldata.txt"))
}

fn deploy(program: u256) -> IHelloStarknetDispatcher {
    declare("UniversalECIP").unwrap();
    let verifier = declare("SP1Groth16VerifierBN254").unwrap().contract_class();
    let app = declare("HelloStarknet").unwrap().contract_class();
    let mut args = array![];
    verifier.class_hash.serialize(ref args);
    program.serialize(ref args);
    let (contract_address, _) = app.deploy(@args).unwrap();
    IHelloStarknetDispatcher { contract_address }
}

fn program_vkey() -> u256 {
    let values = fixture();
    // Garaga serializes the program vkey after the 32 proof-point limbs.
    {
        let mut words = values.span().slice(32, 2);
        Serde::<u256>::deserialize(ref words).unwrap()
    }
}

#[test]
fn valid_fibonacci_receipt() {
    let dispatcher = deploy(program_vkey());
    let result = dispatcher.verify_sp1_proof(fixture()).unwrap();
    assert_eq!(result, array![3_u256, 2_u256, 3_u256].span());
}

#[test]
#[should_panic(expected: 'Wrong program')]
fn reject_other_program() {
    let dispatcher = deploy(program_vkey() + 1);
    let _ = dispatcher.verify_sp1_proof(fixture());
}

fn changed(index: usize, value: felt252) -> Array<felt252> {
    let original = fixture();
    let mut result = array![];
    for i in 0..original.len() {
        result.append(if i == index {
            value
        } else {
            *original.at(i)
        });
    }
    result
}

#[test]
fn reject_failed_guest() {
    let dispatcher = deploy(program_vkey());
    assert_eq!(
        dispatcher.verify_sp1_proof(changed(34, 1)), Result::Err('SP1 exit code must be zero')
    );
}

#[test]
fn reject_wrong_recursion_root() {
    let dispatcher = deploy(program_vkey());
    assert_eq!(dispatcher.verify_sp1_proof(changed(36, 1)), Result::Err('Invalid SP1 vk root'));
}

#[test]
#[should_panic]
fn reject_changed_nonce() {
    let dispatcher = deploy(program_vkey());
    let original = fixture();
    let nonce = if *original.at(38) == 0 {
        1
    } else {
        0
    };
    dispatcher.verify_sp1_proof(changed(38, nonce)).unwrap();
}

#[test]
fn reject_noncanonical_nonce() {
    let dispatcher = deploy(program_vkey());
    assert_eq!(
        dispatcher.verify_sp1_proof(changed(39, 0xffffffffffffffffffffffffffffffff)),
        Result::Err('SP1 public input out of field'),
    );
}

#[test]
#[should_panic]
fn reject_changed_public_values() {
    let dispatcher = deploy(program_vkey());
    // Le premier mot ABI se termine à cet indice; changer n doit invalider la preuve.
    dispatcher.verify_sp1_proof(changed(48, 4)).unwrap();
}
