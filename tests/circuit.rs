// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use parity_plonk::prelude::*;
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Implements a circuit that checks:
// 1) a + b = c where C is a PI
// 2) a <= 2^6
// 3) b <= 2^5
// 4) a * b = d where D is a PI
// 5) JubJub::GENERATOR * e(JubJubScalar) = f where F is a PI
#[derive(Debug, Default)]
pub struct TestCircuit {
    a: BlsScalar,
    b: BlsScalar,
    c: BlsScalar,
    d: BlsScalar,
    e: JubJubScalar,
    f: JubJubAffine,
}

impl Circuit for TestCircuit {
    const CIRCUIT_ID: [u8; 32] = [0xff; 32];
    fn gadget(
        &mut self,
        composer: &mut TurboComposer,
    ) -> std::result::Result<(), Error> {
        let a = composer.append_witness(self.a);
        let b = composer.append_witness(self.b);

        // Make first constraint a + b = c
        let constraint =
            Constraint::new().left(1).right(1).public(-self.c).a(a).b(b);
        composer.append_gate(constraint);

        // Check that a and b are in range
        composer.component_range(a, 1 << 6);
        composer.component_range(b, 1 << 5);

        // Make second constraint a * b = d
        let constraint = Constraint::new()
            .mult(1)
            .output(1)
            .public(-self.d)
            .a(a)
            .b(b);
        composer.append_gate(constraint);

        let e = composer.append_witness(self.e);
        let scalar_mul_result = composer
            .component_mul_generator(e, dusk_jubjub::GENERATOR_EXTENDED);

        // Apply the constrain
        composer.assert_equal_public_point(scalar_mul_result, self.f);

        Ok(())
    }

    fn public_inputs(&self) -> Vec<PublicInputValue> {
        vec![self.c.into(), self.d.into(), self.f.into()]
    }

    fn padded_gates(&self) -> usize {
        1 << 11
    }
}

#[test]
fn test_full() -> Result<()> {
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;
    use tempdir::TempDir;

    let tmp = TempDir::new("plonk-keys-test-full").unwrap().into_path();
    let pp_path = tmp.join("pp_testcirc");
    let pk_path = tmp.join("pk_testcirc");
    let vd_path = tmp.join("vd_testcirc");
    let rng = XorShiftRng::from_seed([
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32,
        0x54, 0x06, 0xbc, 0xe5,
    ]);
    // Generate CRS
    let pp_p = PublicParameters::setup(1 << 12, rng).unwrap();
    fs::write(&pp_path, &pp_p.to_raw_var_bytes()).unwrap();

    // Read PublicParameters
    let pp = fs::read(pp_path).unwrap();
    let pp = unsafe { PublicParameters::from_slice_unchecked(&pp) };

    // Initialize the circuit
    let mut circuit = TestCircuit::default();

    // Compile the circuit
    let (pk_p, vd_p) = circuit.compile(&pp).unwrap();

    // Write the keys
    fs::write(&pk_path, &pk_p.to_var_bytes()).unwrap();

    // Read ProverKey
    let pk = fs::read(pk_path).unwrap();
    let pk = ProverKey::from_slice(&pk).unwrap();

    assert_eq!(pk, pk_p);

    // Store the VerifierData just for the verifier side:
    // (You could also store public_inputs_indexes and VerifierKey
    // sepparatedly).
    fs::write(&vd_path, &vd_p.to_var_bytes()).unwrap();
    let vd = fs::read(vd_path).unwrap();
    let vd = VerifierData::from_slice(&vd).unwrap();

    assert_eq!(vd_p.key(), vd.key());
    assert_eq!(vd_p.public_inputs_indexes(), vd.public_inputs_indexes());

    // Prover POV
    let proof = {
        let mut circuit = TestCircuit {
            a: BlsScalar::from(20u64),
            b: BlsScalar::from(5u64),
            c: BlsScalar::from(25u64),
            d: BlsScalar::from(100u64),
            e: JubJubScalar::from(2u64),
            f: JubJubAffine::from(
                dusk_jubjub::GENERATOR_EXTENDED * JubJubScalar::from(2u64),
            ),
        };

        circuit.prove(&pp, &pk, b"Test")
    }
    .unwrap();

    // Verifier POV
    let public_inputs: Vec<PublicInputValue> = vec![
        BlsScalar::from(25u64).into(),
        BlsScalar::from(100u64).into(),
        JubJubAffine::from(
            dusk_jubjub::GENERATOR_EXTENDED * JubJubScalar::from(2u64),
        )
        .into(),
    ];

    Ok(TestCircuit::verify(&pp, &vd, &proof, &public_inputs, b"Test").unwrap())
}
