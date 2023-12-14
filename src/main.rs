use core_ezkl::{Snark};
use serde_json;
use std::{
    fs,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};
use halo2curves::bn256::{Bn256, G1Affine, Fr};
// use halo2curves::bn256::fr::Fr;
use halo2_proofs::poly::kzg::commitment::{KZGCommitmentScheme, ParamsKZG};
use halo2curves::ff::{FromUniformBytes, PrimeField, WithSmallOrderMulGroup};
use halo2curves::serde::SerdeObject;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};



fn main() {
    let ezkl_snark: Snark<Fr> = load<Fr>("proof.json");
    let proof_bytes = &ezkl_snark.proof[..];
    let instances = ezkl_snark.instances;
    println!("Proof bytes is: {:?}", &instances);

}

fn load<F: PrimeField + SerdeObject + Serialize + FromUniformBytes<64> + DeserializeOwned>(
    proof_path: &str,
) -> Snark<F> {
    let mut f1 = File::open(proof_path).expect("Reason");
    let mut json_file = String::new();
    f1.read_to_string(&mut json_file).expect("Reason");
    serde_json::from_str(&json_file).unwrap()
}
