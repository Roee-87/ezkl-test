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




