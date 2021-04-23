## 1.Introduction

A complete zero-knowledge application is mainly composed of On-chain verification calculation and Off-chain verification calculation. Without the help of the auxiliary toolbox and deep professional domain knowledge and skills, the off-chain part of the program is difficult to be mastered and widely used by ordinary developers.

Therefore, in order to further reduce the threshold and cost of developing ink! zero-knowledge applications, we will build an off-chain cryptography toolbox in the next v0.2 version to help developers use high-level languages to generate Off-chain computable proofs, and Verify the proof in the ink! environment of On-chain. Connect On-chain and Off-chain to create a closed-loop zero-knowledge application development ecosystem for developers.

[ZoKrates](https://github.com/Zokrates/ZoKrates) is a toolbox on Ethereum that supports zkSNARKs application construction. It helps developers generate computable proofs using high-level languages and verify the proofs in the Solidity environment. The ZoKrates community is active, with many developers, and iterative upgrades are fast. In addition to the following advantages:

- Simple and easy-to-use high-level programming language and reusable standard library (including Hasher, Elliptic curve cryptography, Multiplexer, etc.)
- Powerful basic functions (supported Curves are ALT_BN128, BLS12_381, BLS12_377, BW6_761, Schemes support G16, GM17, PGHR13, Backends support Bellman, Libsnark, Arkworks)
- Complete development components (Javascript toolkit)
- Complete documentation and rich use cases


Therefore, we will transplant and transform the toolbox based on ZoKrates to create ZoPatract that is compatible with the Ink! smart contract environment. Achieve the following main goals of v0.2:

- Make ZoPatract's Arkworks Proving schemes support G16, and Curves support bls12_381, bn254 (aligned to v0.1)

- Enable ZoPatract to support the complete commands of the zkSNARKs protocol
- Provide ZoPatract Javascript toolkit
- Provide ZoPatract documentation and sample programs

In the future, Patract will integrate ZoPatract into Online IDE products through plug-ins, providing a lighter development environment.

## 2. Realized deliverable prodects

### 2.1 ZoPatract project

ZoPatract is a zksnark toolbox adapted to the jupiter ink environment based on [zokrates](https://zokrates.github.io/).

### 2.2 Online and manual installation of ZoPatract

### One-line installation

We provide one-line installation for Linux, MacOS and FreeBSD:

```bash
curl -LSfs get.zoprat.es | sh
```

### From source

You can build ZoPatract from [source](https://github.com/patractlabs/ZoPatract/) with the following commands:

```bash
git clone https://github.com/patractlabs/ZoPatract
cd ZoPatract
cargo +nightly build --release
cd target/release
```

### 2.3 Javascript Toolkit

JavaScript bindings for [ZoPatract](https://github.com/patractlabs/ZoPatract).

```bash
npm install zopatract-js
```

#### Importing

##### Bundlers

**Note:** As this library uses a model where the wasm module itself is natively an ES module, you will need a bundler of some form. 
Currently the only known bundler known to be fully compatible with `zopatract-js` is [Webpack](https://webpack.js.org/). 
The choice of this default was done to reflect the trends of the JS ecosystem.

```js
import { initialize } from 'zopatract-js';
```

##### Node

```js
const { initialize } = require('zopatract-js/node');
```

#### Example

```js
initialize().then((zopatractProvider) => {
    const source = "def main(private field a) -> field: return a * a";

    // compilation
    const artifacts = zopatractProvider.compile(source);

    // computation
    const { witness, output } = zopatractProvider.computeWitness(artifacts, ["2"]);

    // run setup
    const keypair = zopatractProvider.setup(artifacts.program);

    // generate proof
    const proof = zopatractProvider.generateProof(artifacts.program, witness, keypair.pk);

    // export solidity verifier
    const verifier = zopatractProvider.exportSolidityVerifier(keypair.vk, "v1");
});
```

### 2.4 [Detailed usage document of ZoPatract](https://github.com/patractlabs/ZoPatract/blob/master/zopatract_book/src/SUMMARY.md)

You can get usage information of related commands, high-level languages, toolkits, and sample codes
(Project code/toolkit/document/sample program)

### 2.5 Simple zk sample application developed by ZoPatract

#### ZoPatract use bls12_381-based arkworks-groth16 algorithm case:

First, create the text-file square_root.zop and implement your program. In this example, we will prove knowledge of the square root a of a number b:

```
def main(private field a, field b) -> bool:
  return a * a == b
```

Some observations:

* The keyword field is the basic type we use, which is an element of a given prime field.
* The keyword private signals that we do not want to reveal this input, but still prove that we know its value.

Then run the different phases of the protocol:
compile: Flatten the zok source code into logical conditional statement form, and generate two files (default out, out.ztf), of which the .ztf file is the readable version.

```
# compile, select bls12_381 curve
./zopatract compile -i square_root.zop -c bls12_381
```

setup: Execute trusted setup operation to generate CRS (Common Reference String) of arkworks-groth16 algorithm.

The input is out generated by compile. Before generating CRS, operations such as R1CS will be generated first, and finally two files will be output: proving.key and verification.key.

```
# perform the setup phase
./zopatract setup -b ark -s g16
```

compute-witness: The input of the command is the out generated by compile, and the input parameters of the calculation problem; a file is output, and the default file name is witness.

```
# execute the program
./zopatract compute-witness -a 12 144
```

generate-proof: Generate the corresponding zero-knowledge proof proof.json based on the constrained system (computation problem) and witness.

```
# use arkworks groth16 algorithm to generate a proof of computation
./zopatract generate-proof -b ark -s g16
```

verify: verify proof.json (the proof.json file in the current path is selected by default)

```
# through -b, -s, -c choice arkworks scheme, groth16 algorithm, bls12_381 curve
./zopatract verify -b ark -s g16 -c bls12_381
```

## 3. Detailed implementation (show core code)

ZoPatract helps you use verifiable computation in your DApp, from the specification of your program in a high level language to generating proofs of computation to verifying those proofs in ink!.

### 3.1 Detailed implementation of ZoPatract

#### 3.1.1 Integrate arkworks-groth16 (https://github.com/arkworks-rs/groth16) to ZoPatract

The top level of the entire design mainly uses different zero-knowledge proof libraries to implement the Backend trait to trigger the use of different libraries:

```rust
pub trait Backend<T: Field, S: Scheme<T>> {
    fn setup(program: ir::Prog<T>) -> SetupKeypair<S::VerificationKey>;

    fn generate_proof(
        program: ir::Prog<T>,
        witness: ir::Witness<T>,
        proving_key: Vec<u8>,
    ) -> Proof<S::ProofPoints>;

    fn verify(vk: S::VerificationKey, proof: Proof<S::ProofPoints>) -> bool;
}
```

We give Ark struct (against the arkworks library) three methods to implement the Backend trait:
Initially generate CRS (provingkey, verifykey)

```rust
fn setup(program: Prog<T>) -> SetupKeypair<<G16 as Scheme<T>>::VerificationKey> {
    let parameters = Computation::without_witness(program).groth16_setup();

    let mut pk: Vec<u8> = Vec::new();
    parameters.serialize_uncompressed(&mut pk).unwrap();

    let vk = VerificationKey {
        alpha: parse_g1::<T>(&parameters.vk.alpha_g1),
        beta: parse_g2::<T>(&parameters.vk.beta_g2),
        gamma: parse_g2::<T>(&parameters.vk.gamma_g2),
        delta: parse_g2::<T>(&parameters.vk.delta_g2),
        gamma_abc: parameters
            .vk
            .gamma_abc_g1
            .iter()
            .map(|g1| parse_g1::<T>(g1))
            .collect(),
    };
    SetupKeypair::new(vk, pk)
}
```

Mainly used to convert Prog generated by dsl into computation, and then generate proof through groth16_prove

```rust
fn generate_proof(
        program: Prog<T>,
        witness: Witness<T>,
        proving_key: Vec<u8>,
    ) -> Proof<<G16 as Scheme<T>>::ProofPoints> {
    let computation = Computation::with_witness(program, witness);
    let params = ProvingKey::<<T as ArkFieldExtensions>::ArkEngine>::deserialize_uncompressed(
        &mut proving_key.as_slice(),
    ).unwrap();

    let proof = computation.clone().groth16_prove(&params);
    let proof_points = ProofPoints {
        a: parse_g1::<T>(&proof.a),
        b: parse_g2::<T>(&proof.b),
        c: parse_g1::<T>(&proof.c),
    };

    let inputs = computation
        .public_inputs_values()
        .iter()
        .map(parse_fr::<T>)
        .collect::<Vec<_>>();

    Proof::new(proof_points, inputs)
}
```

Mainly did the format conversion of verifykey and proof, and encapsulated the verify_proof function of arkworks-groth16.

```rust
fn verify(
        vk: <G16 as Scheme<T>>::VerificationKey,
        proof: Proof<<G16 as Scheme<T>>::ProofPoints>,
    ) -> bool {
    let vk = VerifyingKey {
        alpha_g1: serialization::to_g1::<T>(vk.alpha),
        beta_g2: serialization::to_g2::<T>(vk.beta),
        gamma_g2: serialization::to_g2::<T>(vk.gamma),
        delta_g2: serialization::to_g2::<T>(vk.delta),
        gamma_abc_g1: vk
            .gamma_abc
            .into_iter()
            .map(|g1| serialization::to_g1::<T>(g1))
            .collect(),
    };

    let ark_proof = ArkProof {
        a: serialization::to_g1::<T>(proof.proof.a),
        b: serialization::to_g2::<T>(proof.proof.b),
        c: serialization::to_g1::<T>(proof.proof.c),
    };

    let pvk: PreparedVerifyingKey<<T as ArkFieldExtensions>::ArkEngine> =
        prepare_verifying_key(&vk);

    let public_inputs: Vec<_> = proof
        .inputs
        .iter()
        .map(|s| {
            T::try_from_str(s.trim_start_matches("0x"), 16)
                .unwrap()
                .into_ark()
        })
        .collect::<Vec<_>>();
    verify_proof(&pvk, &ark_proof, &public_inputs).unwrap()
}
```

The setup prove that encapsulates the arkworks groth16 and gm17 algorithms is implemented for Computation through the computation_basic macro.

```rust
macro_rules! computation_basic {
    ($algorithm:tt, $name:ident) => {
        paste::item! {
            impl<T: Field + ArkFieldExtensions> Computation<T> {
                pub fn [<$name _prove>](self, params: &$algorithm::ProvingKey<T::ArkEngine>) -> $algorithm::Proof<T::ArkEngine> {
                    let rng = &mut rand_0_7::rngs::StdRng::from_entropy();

                    let proof = $algorithm::create_random_proof(self.clone(), params, rng).unwrap();

                    let pvk = $algorithm::prepare_verifying_key(&params.vk);

                    // extract public inputs
                    let public_inputs = self.public_inputs_values();

                    assert!($algorithm::verify_proof(&pvk, &proof, &public_inputs).unwrap());

                    proof
                }

                pub fn [<$name _setup>](self) -> $algorithm::ProvingKey<T::ArkEngine> {
                    let rng = &mut rand_0_7::rngs::StdRng::from_entropy();

                    // run setup phase
                    $algorithm::generate_random_parameters(self, rng).unwrap()
                }
            }
        }
    }
}

computation_basic!(ark_gm17, gm17);
computation_basic!(ark_groth16, groth16);
```

#### 3.1.2 Integrate four arkworks curves [Bls12_377, Bls12_381, Bn254, BW6_761](https://github.com/arkworks-rs/curves) into ZoPatract's arkworks groth16 algorithm:

Add arkworks curve through the ArkFieldExtensions trait:

```rust
pub trait ArkFieldExtensions {
    /// An associated type to be able to operate with ark ff traits
    type ArkEngine: PairingEngine;

    fn from_ark(e:<Self::ArkEngine as ark_ec::PairingEngine>::Fr) -> Self;
    fn into_ark(self) -> <Self::ArkEngine as ark_ec::PairingEngine>::Fr;
}
```

Implement the ArkFieldExtensions trait for each curve through the ark_extensions macro:

```rust
ark_extensions!(Bls12_377);
ark_extensions!(Bls12_381);
ark_extensions!(Bn254);
ark_extensions!(BW6_761);
```

Different arkworks curves can be selected through T: trait bound:

```rust
impl<T: Field + ArkFieldExtensions + NotBw6_761Field> Backend<T, G16> for Ark {/*Omited*/}
```

#### 3.2 The implementation of the verfify template for the integrated ink contract (where is the template code, the core changes are displayed in the code, including export_ink_verifier, including the changes to the parameter part of the command line)

The ink_verifier.rs contract template can be exported through the `./zopatract export-verifier -t ink` command.
The realization is processed through the [export_ink_verifier](https://github.com/patractlabs/ZoPatract/blob/master/zopatract_core/src/proof_system/ink.rs) method of the InkCompatibleScheme trait [INK_CONTRACT_TEMPLATE](https://github.com/patractlabs /ZoPatract/blob/master/zopatract_core/src/proof_system/ink.rs) constant strings and CARGO_TOML commonly used strings.

```rust
impl<T: InkCompatibleField> InkCompatibleScheme<T> for G16 {
    fn export_ink_verifier(vk: <G16 as Scheme<T>>::VerificationKey,abi: InkAbi) -> (String,String) {
        let (mut template_text,toml_text) =  match abi {
            InkAbi::V1 => (String::from(INK_CONTRACT_TEMPLATE),String::from(CARGO_TOML)),
            InkAbi::V2 => (String::from(INK_CONTRACT_TEMPLATE),String::from(CARGO_TOML))
        };
        let vk_regex = Regex::new(r#"(<%vk_[^i%]*%>)"#).unwrap();
        let vk_gamma_abc_len_regex = Regex::new(r#"(<%vk_gamma_abc_len%>)"#).unwrap();
        let vk_gamma_abc_regex = Regex::new(r#"(<%vk_gamma_abc%>)"#).unwrap();

        let format_g2affine = |g2:G2Affine|{
            format!(
                "\"{}\", \"{}\", \"{}\", \"{}\"",
                (g2.0).0, (g2.0).1,
                (g2.1).0, (g2.1).1
        )};

        template_text = vk_regex
            .replace(template_text.as_str(),format!("\"{}\",\"{}\"",vk.alpha.0,vk.alpha.1).as_str())
            .into_owned();
        template_text = vk_regex
            .replace(template_text.as_str(), format_g2affine(vk.beta).as_str())
            .into_owned();
        template_text = vk_regex
            .replace(template_text.as_str(),format_g2affine(vk.gamma).as_str())
            .into_owned();
        template_text = vk_regex
            .replace(template_text.as_str(),format_g2affine(vk.delta).as_str())
            .into_owned();
        template_text = vk_gamma_abc_len_regex
            .replace(template_text.as_str(),format!("{}", vk.gamma_abc.len()*2).as_str())
            .into_owned();

        let mut vk_gamma_abc = String::new();
        vk.gamma_abc.iter().for_each(|g1| {
                vk_gamma_abc.extend(format!("\"{}\",\"{}\",",g1.0,g1.1).chars());
        });
        template_text = vk_gamma_abc_regex
            .replace(template_text.as_str(),vk_gamma_abc.strip_suffix(",").unwrap())
            .into_owned();
        (template_text, toml_text)
    }
}
```

### ink contract template([ink_verifier.rs](https://github.com/patractlabs/ZoPatract/blob/master/zopatract_core/src/proof_system/ink.rs)):

```rust
#![cfg_attr(not(feature = "std"), no_std)]
use ink_lang as ink;
use megaclite_arkworks::{groth16, curve::<%curve%>, result::Error};

// VK = [alpha beta gamma delta]
static VK:[&str;14] = [<%vk_alpha%>,
                        <%vk_beta%>,
                        <%vk_gamma%>,
                        <%vk_delta%>];
static VK_GAMMA_ABC:[&str;<%vk_gamma_abc_len%>] =[<%vk_gamma_abc%>];

#[ink::contract]
mod zop {
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
        pub fn verify(&self, proof_and_input: &[u8]) -> Result<bool, Error> {
            groth16::preprocessed_verify_proof::<<%curve%>>(VK, VK_GAMMA_ABC, proof_and_input)
        }
    }
}
```

Cargo.toml templa:

```rust
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
```

## 4. Use ZoPatract to develop zk applications on Jupiter

### off-chain:

First, create the text-file square_root.zop and implement your program. In this example, we will prove knowledge of the square root a of a number b:

```
def main(private field a, field b) -> bool:
  return a * a == b
```

#### Then run the different phases of the protocol:

compile: Flatten the zok source code into logical conditional statement form, and generate two files (default out, out.ztf), of which the .ztf file is the readable version.

```
# compile, select bls12_381 curve
./zopatract compile -i square_root.zop -c bls12_381
```

setup: Execute trusted setup operation to generate CRS (Common Reference String) of arkworks-groth16 algorithm.
The input is the out generated by compile. It will generate R1CS and other operations before generating the CRS, and finally output two files: proving.key and verification.key.

```
# perform the setup phase
./zopatract setup -b ark -s g16
```

#### Export the ink_verifier.rs contract and deploy it to the chain:

export-verifier: select the curve at compile phase to export ink contract-type ink_verifier.rs

```
./zopatract export-verifier -t ink -c bls12_381
```

deploy:
//With pictures

#### generate proof:

compute-witness: The input of the command is the out generated by compile, and the input parameters of the calculation problem; a file is output, and the default file name is witness.

```
# execute the program
./zopatract compute-witness -a 12 144
```

Generate corresponding zero-knowledge proof proof.json and **proof.txt** (hex encode all data) based on the constrained system (computation problem) and witness.

```
# use arkworks groth16 algorithm to generate a proof of computation
./zopatract generate-proof -b ark -s g16
```

cli printed:

```
Generating proof...
Proof hex:
822a26fa4c0a7fbcc725dd45f89d9a33fd69f0545702c55dfe6e5c36f987b9de3a48b53df6e9c2ce04e51dc479307f0281fbdbec9b1510435f8d3b1b6649d408e71f7e61a78d00156e42d7eef6a68f1e6b14b3a0c209e133e5d0fecbf17c2d1500647ec3b72e31d59ed2dc3d4ac84111db3d505c7d0d376e2f5b406c302d927c939e01f76a6298f3e751d7624a72c5d3196abea9d14509701344da6eb3b10d235068dd1f113d78a63b108f64da5c4a13117776a2a6cb8a765f020f569e56172c15cc94eb9d5aba92ec0ad775b14beeca44b9f6db7e6d74d9594a731c40e7cc31b13b140d12e04e0b087315f72624dd97188c9dd182e1607cf18ae48981be0a86a9fa62a696a88e57eee3dad0c5a24f6a5df938a48c77265f595a9765c0cad25c110052c05fa24dc8058811bbeeeced871472451c23370f924854e328198088e533f070f9b7e5636bcd9b4dfd1af96d6b7a0564d4660f7f0e1e75cc25f6c55cd1f1e8db29f105286fd48a5c90394b50b8b2641949f8d62b22778b0bc3b56ee12cbb050090000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000
```



#### Send proof to the chain:

Send the hex proof or proof.txt content to the chain as a transaction via postman
