use halo2curves::ff::{FromUniformBytes, PrimeField, WithSmallOrderMulGroup};
use halo2curves::serde::SerdeObject;
use halo2curves::bn256::Bn256;
use halo2_proofs::poly::commitment::{Params, CommitmentScheme};
use halo2_proofs::poly::kzg::commitment::{KZGCommitmentScheme, ParamsKZG};
use halo2_proofs::poly::commitment::ParamsProver;
use halo2_proofs::plonk::{
    verify_proof, Circuit, VerifyingKey,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use clap::{Args, ValueEnum};
use halo2curves::CurveAffine;
use std::ops::Deref;

// use std::{
//     fs,
//     fs::File,
//     io::{BufReader, BufWriter, Read, Write},
// };

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

#[derive(Debug, Args, Deserialize, Serialize, Clone, Default, PartialEq, PartialOrd)]
pub struct RunArgs {
    #[serde(skip)]
    pub tolerance: String,
    #[serde(skip)]
    pub input_scale: String,
    #[serde(skip)]
    pub param_scale: String,
    #[serde(skip)]
    pub scale_rebase_multiplier: u32,
    /// The min and max elements in the lookup table input column
    #[serde(skip)]
    pub lookup_range: i128,
    /// The log_2 number of rows
    #[arg(short = 'K', long, default_value = "17")]
    pub logrows: u32,
    /// The log_2 number of rows
    #[arg(short = 'N', long, default_value = "2")]
    #[serde(skip)]
    pub num_inner_cols: usize,
    /// Hand-written parser for graph variables, eg. batch_size=1
    #[serde(skip)]
    pub variables: Vec<String>,
    /// Flags whether inputs are public, private, hashed
    #[arg(long, default_value = "private")]
    #[serde(skip)]
    pub input_visibility: String,
    /// Flags whether outputs are public, private, hashed
    #[arg(long, default_value = "public")]
    #[serde(skip)]
    pub output_visibility: String,
    /// Flags whether params are public, private, hashed
    #[arg(long, default_value = "private")]
    #[serde(skip)]
    pub param_visibility: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct GraphSettings {
    /// run args
    pub run_args: RunArgs,
    /// the potential number of rows used by the circuit
    pub num_rows: usize,
    /// total linear coordinate of assignments
    pub total_assignments: usize,
    /// total const size
    pub total_const_size: usize,
    /// the shape of public inputs to the model (in order of appearance)
    #[serde(skip)]
    pub model_instance_shapes: Vec<Vec<usize>>,
    /// model output scales
    #[serde(skip)]
    pub model_output_scales: Vec<String>,
    /// model input scales
    #[serde(skip)]
    pub model_input_scales: Vec<String>,
    /// the of instance cells used by modules
    #[serde(skip)]
    pub module_sizes: String,
    /// required_lookups
    #[serde(skip)]
    pub required_lookups: String,
    /// check mode
    #[serde(skip)]
    pub check_mode: String,
    /// ezkl version used
    pub version: String,
    /// num blinding factors
    pub num_blinding_factors: Option<usize>,
}

fn get_log_rows(settings_path: &str) -> u32 {
    let settings: GraphSettings = serde_json::from_str(&settings_path).unwrap();
    settings.run_args.logrows
}

pub fn get_verifier_params(settings_path: &str, srs_path: &str) -> ParamsKZG<Bn256> {
    // read in log_rows from teh settings struct
    let logrows = get_log_rows(settings_path);

    // read in the params binary file as bytes
    let mut f = File::open(srs_path).expect("File not found");
    let metadata = fs::metadata(srs_path).expect("unable to read metadata");
    let mut buf: Vec<u8> = vec![0; metadata.len() as usize];
    f.read(&mut buf).expect("Buffer overflow");

    // deserialize the params and downsize if necessary
    let mut params: ParamsKZG<Bn256> = Params::read::<_>(&mut &buf[..]).unwrap();
    if logrows < params.k() {
       params.downsize(logrows);
    }
    let vparams = params.verifier_params();
    vparams.clone()
}

pub fn v_params_to_bytes(params: ParamsKZG<Bn256>) -> Vec<u8> {
    // obtain the verifier params and serialize to bytes
    let mut v_params_bytes: Vec<u8> = Vec::new();
    let _ = <ParamsKZG<_> as Params<_>>::write(&params, &mut v_params_bytes);
    v_params_bytes
}