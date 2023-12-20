use core_ezkl::*;
use halo2curves::bn256::{Fr};
use halo2curves::CurveAffine;
use halo2_proofs::poly::commitment::{Params, CommitmentScheme};
use halo2_proofs::poly::kzg::commitment::{KZGCommitmentScheme, ParamsKZG};


// use halo2curves::bn256::fr::Fr;
// use halo2_proofs::poly::kzg::commitment::{KZGCommitmentScheme, ParamsKZG};

const SETTINGS_JSON: &str = include_str!("../settings.json");
const KZG_SRS: &str = "kzg.srs";
const VK: &str = "test.vk";
const PROOF: &str = include_str!("../proof.json");

fn main() {
    let v_params = get_verifier_params(SETTINGS_JSON, KZG_SRS);
    let v_params_bytes = v_params_to_bytes(v_params);
    let ezkl_snark = Snark::<Fr>::load(PROOF);
    //println!("params bytes: {:?}", v_params_bytes.len());
    // let _ = ezkl_snark.format_instances();
    //println!("Proof bytes is: {:?}", &ezkl_snark.proof[..]);
}
