use proof_system::Scheme;
use zopatract_field::{Bn128Field, Field};

pub trait InkCompatibleField: Field {}
impl InkCompatibleField for Bn128Field {}

pub trait InkCompatibleScheme<T: InkCompatibleField>: Scheme<T> {
    fn export_ink_verifier(vk: Self::VerificationKey, abi: InkAbi) -> (String, String);
}

pub enum InkAbi {
    V1,
    V2,
}

impl InkAbi {
    pub fn from(v: &str) -> Result<Self, &str> {
        match v {
            "v1" => Ok(InkAbi::V1),
            "v2" => Ok(InkAbi::V2),
            _ => Err("Invalid ABI version"),
        }
    }
}

pub const INK_CONTRACT_TEMPLATE: &str = r#"
#![cfg_attr(not(feature = "std"), no_std)]
use ink_lang as ink;

extern crate alloc;

#[ink::contract]
mod zop {
    use alloc::vec::Vec;
    use megaclite_arkworks::{groth16, curve::<%curve%>};

    // VK = [alpha beta gamma delta]
    static VK:[&str;14] = [
        <%vk_alpha%>,
        <%vk_beta%>,
        <%vk_gamma%>,
        <%vk_delta%>,
    ];
    static VK_GAMMA_ABC:[&str;<%vk_gamma_abc_len%>] = [<%vk_gamma_abc%>];

    #[ink(storage)]
    pub struct Zop {
        // Stores the ZK result
        result: bool,
    }

    impl Zop {
        /// Use false as initial value
        #[ink(constructor)]
        pub fn default() -> Self {
            Self { result: false }
        }

        #[ink(message)]
        pub fn verify(&self, proof_and_input: Vec<u8>) -> Result<bool, &'static str> {
            groth16::preprocessed_verify_proof::<<%curve%>>(
                VK, VK_GAMMA_ABC, proof_and_input.as_slice(),
            ).map_err(|_| "verify failed")
        }
    }
}
"#;

pub const CARGO_TOML: &str = r#"
[package]
name = "zop"
version = "0.1.0"
authors = ["[your_name] <[your_email]>"]
edition = "2018"

[dependencies]
ink_primitives = { version = "3.0.0-rc2", default-features = false }
ink_metadata = { version = "3.0.0-rc2", default-features = false, features = ["derive"], optional = true }
ink_env = { version = "3.0.0-rc2", default-features = false }
ink_storage = { version = "3.0.0-rc2", default-features = false }
ink_lang = { version = "3.0.0-rc2", default-features = false }

scale = { package = "parity-scale-codec", version = "1.3", default-features = false, features = ["derive"] }
scale-info = { version = "0.4.1", default-features = false, features = ["derive"], optional = true }

# megalicte zk library
megaclite-arkworks = { git = "https://github.com/patractlabs/megaclite.git", default-features = false }

[lib]
name = "zop"
path = "lib.rs"
crate-type = [
	# Used for normal contract Wasm blobs.
	"cdylib",
]

[features]
default = ["std"]
std = [
    "ink_metadata/std",
    "ink_env/std",
    "ink_storage/std",
    "ink_primitives/std",
    "scale/std",
    "scale-info/std",
]
ink-as-dependency = []
"#;
