use ark_bls12_381::Bls12_381;
use ark_ec::bls12::Bls12;
use ark_ec::pairing::Pairing;
use ark_poly::{EvaluationDomain, Radix2EvaluationDomain};
use ark_std::UniformRand;
use batch_threshold::dealer::Dealer;
use rand::thread_rng;
use ark_serialize::{CanonicalSerialize};
// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::{BATCH_ENCRYPT_CIRCUIT_ELF, BATCH_ENCRYPT_CIRCUIT_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};

type E = Bls12_381;
type Fr = <Bls12<ark_bls12_381::Config> as Pairing>::ScalarField;
type G1 = <Bls12<ark_bls12_381::Config> as Pairing>::G1;
type G2 = <Bls12<ark_bls12_381::Config> as Pairing>::G2;

fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let mut rng = thread_rng();
    let batch_size = 1 << 5;
    let n = 1 << 4;
    let tx_domain = Radix2EvaluationDomain::<Fr>::new(batch_size).unwrap();

    let mut dealer = Dealer::<E>::new(batch_size, n, n / 2 - 1);
    let (crs, _) = dealer.setup(&mut rng);
    let pk = dealer.get_pk();

    let msg = [1u8; 32];
    let x = tx_domain.group_gen;

    let hid = G1::rand(&mut rng);

    let mut x_compressed_bytes = Vec::new();
    x.serialize_compressed(&mut x_compressed_bytes).expect("Unable to compress x");

    let mut hid_compressed_bytes = Vec::new();
    hid.serialize_compressed(&mut hid_compressed_bytes).expect("Unable to compress hid");

    let mut htau_compressed_bytes = Vec::new();
    crs.htau.serialize_compressed(&mut htau_compressed_bytes).expect("Unable to compress htau");

    let mut pk_compressed_bytes = Vec::new();
    pk.serialize_compressed(&mut pk_compressed_bytes).expect("Unable to compress pk");

    // For example:
    let input: core::PrivateInputs = core::PrivateInputs {
        msg,
        x : x_compressed_bytes,
        hid: hid_compressed_bytes,
        htau: htau_compressed_bytes,
        pk: pk_compressed_bytes,
    };
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();
    let prove_info = prover.prove(env, BATCH_ENCRYPT_CIRCUIT_ELF).unwrap();
    let receipt = prove_info.receipt;
    let output: u32 = receipt.journal.decode().unwrap();
    println!(">>> prover output: {}", output);
    receipt.verify(BATCH_ENCRYPT_CIRCUIT_ID).unwrap();
}
