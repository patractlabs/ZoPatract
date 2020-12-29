use ark_groth16::{
    prepare_verifying_key, verify_proof, PreparedVerifyingKey, Proof as ArkProof, ProvingKey,
    VerifyingKey,
};

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use zokrates_field::{ArkFieldExtensions, Bw6_761Field, Field};

use crate::proof_system::ark::Computation;
use crate::proof_system::ark::{parse_fr, parse_g1, parse_g2};
use ir::{Prog, Witness};
use proof_system::ark::{Ark, serialization};
use proof_system::groth16::{NotBw6_761Field, ProofPoints, VerificationKey, G16};
use proof_system::Scheme;
use proof_system::{Backend, Proof, SetupKeypair};

impl<T: Field + ArkFieldExtensions + NotBw6_761Field> Backend<T, G16> for Ark {
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

    fn generate_proof(
        program: Prog<T>,
        witness: Witness<T>,
        proving_key: Vec<u8>,
    ) -> Proof<<G16 as Scheme<T>>::ProofPoints> {
        let computation = Computation::with_witness(program, witness);
        let params = ProvingKey::<<T as ArkFieldExtensions>::ArkEngine>::deserialize_uncompressed(
            &mut proving_key.as_slice(),
        )
            .unwrap();

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
}

impl Backend<Bw6_761Field, G16> for Ark {
    fn setup(
        program: Prog<Bw6_761Field>,
    ) -> SetupKeypair<<G16 as Scheme<Bw6_761Field>>::VerificationKey> {
        let parameters = Computation::without_witness(program).groth16_setup();

        let mut pk: Vec<u8> = Vec::new();
        parameters.serialize_uncompressed(&mut pk).unwrap();

        let vk = VerificationKey {
            alpha: parse_g1::<Bw6_761Field>(&parameters.vk.alpha_g1),
            beta: parse_g2::<Bw6_761Field>(&parameters.vk.beta_g2),
            gamma: parse_g2::<Bw6_761Field>(&parameters.vk.gamma_g2),
            delta: parse_g2::<Bw6_761Field>(&parameters.vk.delta_g2),
            gamma_abc: parameters
                .vk
                .gamma_abc_g1
                .iter()
                .map(|g1| parse_g1::<Bw6_761Field>(g1))
                .collect(),
        };

        SetupKeypair::new(vk, pk)
    }

    fn generate_proof(
        program: Prog<Bw6_761Field>,
        witness: Witness<Bw6_761Field>,
        proving_key: Vec<u8>,
    ) -> Proof<<G16 as Scheme<Bw6_761Field>>::ProofPoints> {
        let computation = Computation::with_witness(program, witness);
        let params =
            ProvingKey::<<Bw6_761Field as ArkFieldExtensions>::ArkEngine>::deserialize_uncompressed(
                &mut proving_key.as_slice(),
            ).unwrap();

        let proof = computation.clone().groth16_prove(&params);
        let proof_points = ProofPoints {
            a: parse_g1::<Bw6_761Field>(&proof.a),
            b: parse_g2::<Bw6_761Field>(&proof.b),
            c: parse_g1::<Bw6_761Field>(&proof.c),
        };

        let inputs = computation
            .public_inputs_values()
            .iter()
            .map(parse_fr::<Bw6_761Field>)
            .collect::<Vec<_>>();

        Proof::new(proof_points, inputs)
    }

    fn verify(
        vk: <G16 as Scheme<Bw6_761Field>>::VerificationKey,
        proof: Proof<<G16 as Scheme<Bw6_761Field>>::ProofPoints>,
    ) -> bool {
        let vk = VerifyingKey {
            alpha_g1: serialization::to_g1::<Bw6_761Field>(vk.alpha),
            beta_g2: serialization::to_g2::<Bw6_761Field>(vk.beta),
            gamma_g2: serialization::to_g2::<Bw6_761Field>(vk.gamma),
            delta_g2: serialization::to_g2::<Bw6_761Field>(vk.delta),
            gamma_abc_g1: vk
                .gamma_abc
                .into_iter()
                .map(|g1| serialization::to_g1::<Bw6_761Field>(g1))
                .collect(),
        };

        let ark_proof = ArkProof {
            a: serialization::to_g1::<Bw6_761Field>(proof.proof.a),
            b: serialization::to_g2::<Bw6_761Field>(proof.proof.b),
            c: serialization::to_g1::<Bw6_761Field>(proof.proof.c),
        };

        let pvk: PreparedVerifyingKey<<Bw6_761Field as ArkFieldExtensions>::ArkEngine> =
            prepare_verifying_key(&vk);

        let public_inputs: Vec<_> = proof
            .inputs
            .iter()
            .map(|s| {
                Bw6_761Field::try_from_str(s.trim_start_matches("0x"), 16)
                    .unwrap()
                    .into_ark()
            })
            .collect::<Vec<_>>();

        verify_proof(&pvk, &ark_proof, &public_inputs).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::flat_absy::FlatVariable;
    use crate::ir::{Function, Interpreter, Prog, Statement};

    use super::*;
    use zokrates_field::{Bls12_377Field, /*Bw6_761Field*/};

    #[test]
    fn verify_bls12_377_field() {
        let program: Prog<Bls12_377Field> = Prog {
            main: Function {
                id: String::from("main"),
                arguments: vec![FlatVariable::new(0)],
                returns: vec![FlatVariable::public(0)],
                statements: vec![Statement::Constraint(
                    FlatVariable::new(0).into(),
                    FlatVariable::public(0).into(),
                )],
            },
            private: vec![false],
        };

        let keypair = <Ark as Backend<Bls12_377Field, G16>>::setup(program.clone());
        let interpreter = Interpreter::default();

        let witness = interpreter
            .execute(&program, &vec![Bls12_377Field::from(42)])
            .unwrap();

        let proof =
            <Ark as Backend<Bls12_377Field, G16>>::generate_proof(program, witness, keypair.pk);
        let ans = <Ark as Backend<Bls12_377Field, G16>>::verify(keypair.vk, proof);

        assert!(ans);
    }
}