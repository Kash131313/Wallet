//! Minimal stub of the official `tvm-sdk` crate.
//! Provides only the types and methods that `acki_nest` uses in the
//! real implementation (mnemonic parsing, key‑pair derivation, address conversion).
//!
//! The stub does **not** perform any cryptographic operations – it simply
//! stores the provided strings and returns deterministic placeholders.
//!
//! This allows the project to compile in an offline environment while
//! keeping the same API surface. When you have network access you can replace
//! this directory with the real `tvm-sdk` crate (e.g. by removing the `path`
//! entry in Cargo.toml).

pub mod crypto {
    pub mod mnemonic {
        use std::error::Error;

        /// Simple wrapper around the mnemonic string.
        #[derive(Debug, Clone)]
        pub struct Mnemonic(pub String);

        impl Mnemonic {
            /// In the real SDK this validates the 24‑word phrase.
            /// Here we just store the string as‑is.
            pub fn from_phrase(phrase: &str) -> Result<Self, Box<dyn Error>> {
                // Minimal validation: ensure it is not empty.
                if phrase.trim().is_empty() {
                    return Err("mnemonic phrase is empty".into());
                }
                Ok(Mnemonic(phrase.to_string()))
            }
        }
    }

    pub mod keypair {
        use super::mnemonic::Mnemonic;
        use std::error::Error;

        /// Dummy key‑pair representation.
        #[derive(Debug, Clone)]
        pub struct KeyPair(pub String);

        impl KeyPair {
            /// Derive a key‑pair from a mnemonic.
            /// The real SDK uses proper HD‑derivation – we just echo the mnemonic.
            pub fn from_mnemonic(mnemonic: &Mnemonic) -> Result<Self, Box<dyn Error>> {
                Ok(KeyPair(mnemonic.0.clone()))
            }
        }
    }
}

pub mod address {
    use super::crypto::keypair::KeyPair;
    use std::fmt;

    /// Dummy address – in the real SDK it encodes the public key.
    #[derive(Debug, Clone)]
    pub struct Address(pub String);

    impl Address {
        /// Build an address from a key‑pair.
        /// For the stub we just prefix the stored key with "addr-".
        pub fn from_keypair(keypair: &KeyPair) -> Self {
            Address(format!("addr-{}", keypair.0))
        }
    }

    // Implement Display so that `to_string()` works like the real crate.
    impl fmt::Display for Address {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
}
