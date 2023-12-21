use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::utils::F32;
use clap::Args;

/// The denominator in the fixed point representation used when quantizing inputs
pub type Scale = i32;

/// Parameters specific to a proving run
#[derive(Debug, Args, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct RunArgs {
    /// The tolerance for error on model outputs
    #[arg(short = 'T', long, default_value = "0")]
    pub tolerance: Tolerance,
    /// The denominator in the fixed point representation used when quantizing inputs
    #[arg(short = 'S', long, default_value = "7", allow_hyphen_values = true)]
    pub input_scale: Scale,
    /// The denominator in the fixed point representation used when quantizing parameters
    #[arg(long, default_value = "7", allow_hyphen_values = true)]
    pub param_scale: Scale,
    /// if the scale is ever > scale_rebase_multiplier * input_scale then the scale is rebased to input_scale (this a more advanced parameter, use with caution)
    #[arg(long, default_value = "1")]
    pub scale_rebase_multiplier: u32,
    /// The min and max elements in the lookup table input column
    #[arg(short = 'B', long, value_parser = parse_tuple::<i128>, default_value = "(-32768,32768)")]
    pub lookup_range: (i128, i128),
    /// The log_2 number of rows
    #[arg(short = 'K', long, default_value = "17")]
    pub logrows: u32,
    /// The log_2 number of rows
    #[arg(short = 'N', long, default_value = "2")]
    pub num_inner_cols: usize,
    /// Hand-written parser for graph variables, eg. batch_size=1
    #[arg(short = 'V', long, value_parser = parse_key_val::<String, usize>, default_value = "batch_size=1", value_delimiter = ',')]
    pub variables: Vec<(String, usize)>,
    /// Flags whether inputs are public, private, hashed
    #[arg(long, default_value = "private")]
    pub input_visibility: Visibility,
    /// Flags whether outputs are public, private, hashed
    #[arg(long, default_value = "public")]
    pub output_visibility: Visibility,
    /// Flags whether params are public, private, hashed
    #[arg(long, default_value = "private")]
    pub param_visibility: Visibility,
}

/// Label enum to track whether model input, model parameters, and model output are public, private, or hashed
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Visibility {
    /// Mark an item as private to the prover (not in the proof submitted for verification)
    #[default]
    Private,
    /// Mark an item as public (sent in the proof submitted for verification)
    Public,
    /// Mark an item as publicly committed to (hash sent in the proof submitted for verification)
    Hashed {
        /// Whether the hash is used as an instance (sent in the proof submitted for verification)
        /// if false the hash is used as an advice (not in the proof submitted for verification) and is then sent to the computational graph
        /// if true the hash is used as an instance (sent in the proof submitted for verification) the *inputs* to the hashing function are then sent to the computational graph
        hash_is_public: bool,
        ///
        outlets: Vec<usize>,
    },
    /// Mark an item as publicly committed to (KZG commitment sent in the proof submitted for verification)
    KZGCommit,
    /// assigned as a constant in the circuit
    Fixed,
}

#[allow(missing_docs)]
/// An enum representing the tolerance we can accept for the accumulated arguments, either absolute or percentage
#[derive(Clone, Default, Debug, PartialEq, PartialOrd, Serialize, Deserialize, Copy)]
pub struct Tolerance {
    pub val: f32,
    pub scale: F32,
}

/// Parse a single key-value pair
fn parse_key_val<T, U>(
    s: &str,
) -> Result<(T, U), Box<dyn std::error::Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: std::error::Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

/// Parse a tuple
fn parse_tuple<T>(s: &str) -> Result<(T, T), Box<dyn std::error::Error + Send + Sync + 'static>>
where
    T: std::str::FromStr + Clone,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    let res = s.trim_matches(|p| p == '(' || p == ')').split(',');

    let res = res
        .map(|x| {
            // remove blank space
            let x = x.trim();
            x.parse::<T>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    if res.len() != 2 {
        return Err("invalid tuple".into());
    }
    Ok((res[0].clone(), res[1].clone()))
}