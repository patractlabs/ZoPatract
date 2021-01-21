use proof_system::Scheme;
use zopatract_field::{Bn128Field, Field};

pub trait InkCompatibleField: Field {}
impl InkCompatibleField for Bn128Field {}

pub trait InkCompatibleScheme<T: InkCompatibleField>: Scheme<T> {
    fn export_ink_verifier(vk: Self::VerificationKey, abi: InkAbi) -> String;
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
Cargo.toml:
hex = "0.4.2"
num-bigint = { version = "0.3", default-features = false }
megaclite-arkworks = { git = "https://github.com/patractlabs/megaclite", default-features = false }

use hex;
use megaclite_arkworks::{call, result::Result, CurveBasicOperations, Error, ErrorKind, SerializationError};
use alloc::{string::ToString, vec::Vec};
use num_bigint::BigUint;

fn ink_contract_template() {
    use crate::curve::Bls12_381;

    // VK = [alpha beta gamma delta]
    const VK:[&str;14] = [<%vk_alpha%>,
                        <%vk_beta%>,
                        <%vk_gamma%>,
                        <%vk_delta%>];
    const VK_GAMMA_ABC:[&str;<%vk_gamma_abc_len%>] =[<%vk_gamma_abc%>];

    // proof that ZoPatract generate under the chain. 
    let proof_and_input = "";

    assert(
        preprocessed_verify_proof::<Bls12_381>(VK, VK_GAMMA_ABC, proof_and_input).unwrap(),
    );
}

/// preprocess vk and proof to verify proof
pub fn preprocessed_verify_proof<C: CurveBasicOperations>(
    vk: [&str; 14],
    vk_gamma_abc: [&str; 6],
    proof_and_input: &'static str,
) -> Result<bool> {
    let bytes = hex::decode(proof_and_input).map_err(|e| format!("hex decode error:{}", e))?;
    let (proof, input) = bytes.split_at(2 * C::G1_LEN + C::G2_LEN);

    let mut vk_vec = Vec::new();
    vk_vec.append(&mut g2_pad_infinity(vk[6], vk[7], vk[8], vk[9]));
    vk_vec.append(&mut g2_pad_infinity(vk[10], vk[11], vk[12], vk[13]));
    vk_vec.append(&mut g1_pad_infinity(vk[0], vk[1]));
    vk_vec.append(&mut g2_pad_infinity(vk[2], vk[3], vk[4], vk[5]));

    verify_proof::<C>(
        (0..vk_gamma_abc.len() / 2)
            .map(|i| g1_pad_infinity(vk_gamma_abc[i * 2], vk_gamma_abc[i * 2 + 1]))
            .collect(),
        vk_vec,
        proof.to_vec(),
        (0..input.len() / C::SCALAR_LEN)
            .map(|i| input[i * C::SCALAR_LEN..(i + 1) * C::SCALAR_LEN].to_vec())
            .collect(),
    )
}

/// Groth16 verification
pub fn verify_proof<C: CurveBasicOperations>(
    vk_gamma_abc: Vec<Vec<u8>>,
    vk: Vec<u8>,
    proof: Vec<u8>,
    public_inputs: Vec<Vec<u8>>,
) -> Result<bool> {
    let g1_len = C::G1_LEN;
    let g2_len = C::G2_LEN;
    let g1_g2_len = C::G2_LEN + C::G1_LEN;
    let scalar_len = C::SCALAR_LEN;

    if (public_inputs.len() + 1) != vk_gamma_abc.len() {
        return Err(Error::VerifyParcelFailed);
    }

    // First two fields are used as the sum
    let mut acc = vk_gamma_abc[0].to_vec();

    // Compute the linear combination vk_x:[(βui(x)+αvi(x)+wi(x))/γ] ∈ G1
    // acc = sigma(i:0~l) * [(βui(x)+αvi(x)+wi(x))/γ] ∈ G1
    for (i, b) in public_inputs.iter().zip(vk_gamma_abc.iter().skip(1)) {
        let mut mul_input = Vec::with_capacity(scalar_len + g1_len);
        mul_input.extend_from_slice(b);
        mul_input.extend_from_slice(i);

        // Check if invalid length
        if mul_input.len() != g1_len + scalar_len {
            return Err(format!(
                "Invalid input length {} for mul operation, should be {}",
                mul_input.len(),
                g1_len + scalar_len
            )
            .into());
        }
        let mul_ic = call(0x01000001 + C::CURVE_ID, &mul_input)?;

        let mut acc_mul_ic = Vec::with_capacity(g1_len * 2);
        acc_mul_ic.extend_from_slice(acc.as_ref());
        acc_mul_ic.extend_from_slice(mul_ic.as_ref());

        // Check if invalid length
        if acc_mul_ic.len() != g1_len * 2 {
            return Err(format!(
                "Invalid input length {} for add operation, should be {}",
                acc_mul_ic.len(),
                g1_len * 2
            )
            .into());
        }
        acc = call(0x01000000 + C::CURVE_ID, &*acc_mul_ic)?;
    }

    // The original verification equation is:
    // A * B = alpha * beta + acc * gamma + C * delta
    // ... however, we rearrange it so that it is:
    // A * B - acc * gamma - C * delta = alpha * beta
    // or equivalently:
    //    A   *    B    +  (-acc) * gamma +  (-C) * delta  +   (-alpha) * beta = 0
    let pairings = [
        (
            &proof[0..g1_len / 2],           // G1 x
            &proof[g1_len / 2..g1_len - 1],  // G1 y
            &proof[g1_len - 1..g1_len],      // G1 infinity
            &proof[g1_len..g1_len + g2_len], // G2
        ),
        (
            &acc[0..g1_len / 2],
            &*negate_y::<C>(&acc[g1_len / 2..g1_len - 1]),
            &acc[g1_len - 1..g1_len],
            &vk[0..g2_len],
        ),
        (
            &proof[g1_g2_len..g1_g2_len + g1_len / 2],
            &*negate_y::<C>(&proof[g1_g2_len + g1_len / 2..g1_g2_len + g1_len - 1]),
            &proof[g1_g2_len + g1_len - 1..g1_g2_len + g1_len],
            &vk[g2_len..g2_len * 2],
        ),
        (
            &vk[g2_len * 2..g2_len * 2 + g1_len / 2],
            &*negate_y::<C>(&vk[g2_len * 2 + g1_len / 2..g2_len * 2 + g1_len - 1]),
            &vk[g2_len * 2 + g1_len - 1..g2_len * 2 + g1_len],
            &vk[g2_len * 2 + g1_len..g2_len * 3 + g1_len],
        ),
    ];

    let mut input = Vec::with_capacity((g1_len + g2_len) * 4);
    pairings.iter().for_each(|(x, y, infinity, g2)| {
        input.extend_from_slice(x);
        input.extend_from_slice(y);
        input.extend_from_slice(infinity);
        input.extend_from_slice(g2);
    });

    // Return the result of computing the pairing check
    // e(p1[0], p2[0]) *  .... * e(p1[n], p2[n]) == 1.
    // For example pairing([P1(), P1().negate()], [P2(), P2()]) should return true.
    Ok(call(0x01000002 + C::CURVE_ID, &input)?[0] == 0)
}

fn g1_pad_infinity(x: &str, y: &str) -> Vec<u8> {
    let mut bytes = vec![];
    bytes.append(&mut decode_hex(x.to_string()));
    bytes.append(&mut decode_hex(y.to_string()));
    bytes.push(0u8); // infinity flag
    bytes
}

fn g2_pad_infinity(x1: &str, y1: &str, x2: &str, y2: &str) -> Vec<u8> {
    let mut bytes = vec![];
    bytes.append(&mut decode_hex(x1.to_string()));
    bytes.append(&mut decode_hex(y1.to_string()));
    bytes.append(&mut decode_hex(x2.to_string()));
    bytes.append(&mut decode_hex(y2.to_string()));
    bytes.push(0u8); // infinity flag
    bytes
}

fn decode_hex(value: String) -> Vec<u8> {
    let mut bytes = hex::decode(value.strip_prefix("0x").unwrap()).unwrap();
    bytes.reverse();
    bytes
}

fn negate_y_based_curve(y: BigUint, MODULUS: &[u8]) -> BigUint {
    let q = BigUint::from_bytes_le(MODULUS);
    q.clone() - y % q
}

fn negate_y<C: CurveBasicOperations>(y: &[u8]) -> Vec<u8> {
    let neg_y = negate_y_based_curve(BigUint::from_bytes_le(y), C::MODULUS).to_bytes_le();

    // Because of randomness, Negate_y vector might not satisfy g1_y_len bytes.
    let mut neg_y_fill_with_zero = vec![0; y.len()];
    neg_y_fill_with_zero[0..neg_y.len()].copy_from_slice(&*neg_y);

    neg_y_fill_with_zero
}
"#;
