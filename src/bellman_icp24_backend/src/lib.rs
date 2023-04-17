use bellman::{
    groth16, Circuit, ConstraintSystem, SynthesisError,
};
use bls12_381::Bls12;
use ff::PrimeField;
use rand::rngs::StdRng;
use rand::SeedableRng;

struct MultiplyDemo<Scalar: PrimeField> {
    a: Option<Scalar>,
    b: Option<Scalar>,
}

impl<Scalar: PrimeField> Circuit<Scalar> for MultiplyDemo<Scalar> {
    fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        // Allocate the first value (private)
        let a = cs.alloc(|| "a", || {
            self.a.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Allocate the second value (private)
        let b = cs.alloc(|| "b", || {
            self.b.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let result = Scalar::from(24);

        // a * b = 24
        cs.enforce(
            || "mult",
            |lc| lc + a,
            |lc| lc + b,
            |lc| lc + (result, CS::one()),
        );

        Ok(())
    }
}
#[ic_cdk_macros::query]
fn test_groth16() -> bool {
    let rng = &mut StdRng::seed_from_u64(0u64);

    let pk = {
        let c = MultiplyDemo {
            a: None,
            b: None,
        };

        groth16::generate_random_parameters::<Bls12, _, _>(c, rng).unwrap()
    };

    let pvk = groth16::prepare_verifying_key(&pk.vk);

    let assignment = MultiplyDemo {
        a: Some(2.into()),
        b: Some(12.into()),
    };

    let proof = groth16::create_random_proof(assignment, &pk, rng).unwrap();

    let verification_result = groth16::verify_proof(&pvk, &proof, &[]);
    return verification_result.is_ok();
}