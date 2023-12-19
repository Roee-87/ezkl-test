use core_ezkl::*;
use halo2curves::bn256::{Bn256, Fr};
use halo2curves::CurveAffine;
use halo2_proofs::poly::commitment::{Params, CommitmentScheme};
use halo2_proofs::poly::kzg::commitment::{KZGCommitmentScheme, ParamsKZG};

use std::fs;
use std::fs::File;
use std::io::Read;

// use halo2curves::bn256::fr::Fr;
// use halo2_proofs::poly::kzg::commitment::{KZGCommitmentScheme, ParamsKZG};

const SETTINGS_JSON: &str = include_str!("../settings.json");
const KZG_SRS: &str = "kzg.srs";
const VK: &str = "test.vk";
const PROOF: &str = include_str!("../proof.json");

fn main() {
    let v_param = get_verifier_params(SETTINGS_JSON, KZG_SRS);
    //println!("params k: {:?}", param_bytes);
    let vk = get_verifier_key(VK);)  
    let ezkl_snark = Snark::<Fr>::load(PROOF);
    // let proof_bytes = &ezkl_snark.proof[..];
    //let instances = ezkl_snark.instances;
    //let _ = ezkl_snark.format_instances();
    // println!("Proof bytes is: {:?}", &ezkl_snark.instances);


}


