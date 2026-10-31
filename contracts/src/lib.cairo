#[starknet::interface]
pub trait IHelloStarknet<TContractState> {
    fn verify_sp1_proof(
        self: @TContractState, proof: Array<felt252>,
    ) -> Result<Span<u256>, felt252>;
}

#[starknet::contract]
mod HelloStarknet {
    use core::num::traits::Zero;
    use starknet::storage::{StoragePointerReadAccess, StoragePointerWriteAccess};
    use starknet::syscalls::library_call_syscall;
    use starknet::{ClassHash, SyscallResultTrait};

    #[storage]
    struct Storage {
        verifier: ClassHash,
        program_vkey: u256,
    }

    // Le constructeur fixe ces deux engagements; aucun setter ne peut les remplacer.
    #[constructor]
    fn constructor(ref self: ContractState, verifier: ClassHash, program_vkey: u256) {
        assert(verifier.is_non_zero(), 'Zero verifier class hash');
        self.verifier.write(verifier);
        self.program_vkey.write(program_vkey);
    }

    #[abi(embed_v0)]
    impl HelloStarknetImpl of super::IHelloStarknet<ContractState> {
        fn verify_sp1_proof(
            self: @ContractState, proof: Array<felt252>,
        ) -> Result<Span<u256>, felt252> {
            let mut calldata = array![];
            proof.serialize(ref calldata);
            let mut serialized = library_call_syscall(
                self.verifier.read(), selector!("verify_sp1_groth16_proof_bn254"), calldata.span(),
            )
                .unwrap_syscall();
            let result = Serde::<Result<(u256, Span<u256>), felt252>>::deserialize(ref serialized)
                .unwrap();
            assert(serialized.is_empty(), 'Unexpected verifier data');
            match result {
                Result::Ok((vkey, public_values)) => {
                    assert(vkey == self.program_vkey.read(), 'Wrong program');
                    Result::Ok(public_values)
                },
                Result::Err(error) => Result::Err(error),
            }
        }
    }
}
