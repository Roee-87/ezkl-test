use halo2curves::ff::{FromUniformBytes, PrimeField, WithSmallOrderMulGroup};
use halo2curves::serde::SerdeObject;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use clap::ValueEnum;
//use snark_verifier::verifier::plonk::PlonkProtocol;
use std::{
    fs,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};


#[allow(missing_docs)]
#[derive(
    ValueEnum, Default, Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize, PartialOrd,
)]
pub enum TranscriptType {
    //Poseidon,
    #[default]
    EVM,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A proof split commit
pub struct ProofSplitCommit {
    /// The start index of the output in the witness
    start: usize,
    /// The end index of the output in the witness
    end: usize,
}

/// An application snark with proof and instance variables ready for aggregation (raw field element)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snark<F: PrimeField + SerdeObject>
{
    #[serde(skip)]
    pub protocol: String,
    /// public instances of the snark
    pub instances: Vec<Vec<F>>,
    /// the proof
    pub proof: Vec<u8>,
    /// transcript type
    #[serde(skip)]
    pub transcript_type: TranscriptType,
    /// the split proof
    #[serde(skip)]
    pub split: Option<ProofSplitCommit>,
}

impl<F: PrimeField + SerdeObject + Serialize + FromUniformBytes<64> + DeserializeOwned> Snark<F> {
    pub fn load(
        proof_path: &str,
    ) -> Self {
        let mut f1 = File::open(proof_path).expect("Could not find path");
        let mut json_file = String::new();
        f1.read_to_string(&mut json_file).expect("Unable to read to string");
        serde_json::from_str(&json_file).unwrap()
    }
}