use core_ezkl::Snark;
use halo2curves::bn256::Fr;

// use halo2curves::bn256::fr::Fr;
// use halo2_proofs::poly::kzg::commitment::{KZGCommitmentScheme, ParamsKZG};

fn main() {
    let ezkl_snark = Snark::<Fr>::load("proof.json");
    let proof_bytes = &ezkl_snark.proof[..];
    let instances = ezkl_snark.instances;
    println!("Proof bytes is: {:?}", &proof_bytes);
    println!("Instances is: {:?}", &instances);
}
