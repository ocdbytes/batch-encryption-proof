use ark_bls12_381::Bls12_381;
use ark_ec::bls12::Bls12;
use ark_ec::pairing::Pairing;
use risc0_zkvm::guest::env;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

type E = Bls12_381;
type Fr = <Bls12<ark_bls12_381::Config> as Pairing>::ScalarField;
type G1 = <Bls12<ark_bls12_381::Config> as Pairing>::G1;
type G2 = <Bls12<ark_bls12_381::Config> as Pairing>::G2;

fn main() {
    let input: core::PrivateInputs = env::read();

    let x = Fr::deserialize_compressed_unchecked(&*input.x).expect("Failed to deserialize compressed input");
    let hid = G1::deserialize_compressed(&*input.hid).expect("Failed to deserialize compressed input");
    let h_tau = G2::deserialize_compressed(&*input.htau).expect("Failed to deserialize compressed input");
    let pk = G2::deserialize_compressed(&*input.pk).expect("Failed to deserialize compressed input");

    let encrypt_result = batch_threshold::encryption::encrypt::<Bls12_381>(input.msg, x, hid, h_tau.clone(), pk.clone());
    &encrypt_result.verify(h_tau, pk);
    env::commit(&1);
}
