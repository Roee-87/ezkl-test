use halo2curves::ff::{FromUniformBytes, PrimeField, WithSmallOrderMulGroup};
use halo2curves::serde::SerdeObject;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use clap::ValueEnum;
//use snark_verifier::verifier::plonk::PlonkProtocol;


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