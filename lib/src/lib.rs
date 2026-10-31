use alloy_sol_types::sol;

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        uint32 n;
        uint32 a;
        uint32 b;
    }
}

/// Compute the n'th fibonacci number (wrapping around on overflows), using normal Rust code.
pub fn fibonacci(n: u32) -> (u32, u32) {
    let mut a = 0u32;
    let mut b = 1u32;
    for _ in 0..n {
        let c = a.wrapping_add(b);
        a = b;
        b = c;
    }
    (a, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_sol_types::SolType;

    #[test]
    fn fibonacci_indices_and_wrapping_are_explicit() {
        assert_eq!(fibonacci(0), (0, 1));
        assert_eq!(fibonacci(3), (2, 3));
        assert_eq!(fibonacci(10), (55, 89));
        assert_eq!(fibonacci(48), (512559680, 3483774753));
    }

    #[test]
    fn public_values_use_three_complete_words() {
        let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct { n: 3, a: 2, b: 3 });
        assert_eq!(bytes.len(), 96);
        assert_eq!((bytes[31], bytes[63], bytes[95]), (3, 2, 3));
    }
}
