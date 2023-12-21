use utils::{F32, Scale};

/// model parameters
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
    pub model_instance_shapes: Vec<Vec<usize>>,
    /// model output scales
    pub model_output_scales: Vec<Scale>,
    /// model input scales
    pub model_input_scales: Vec<Scale>,
    /// the of instance cells used by modules
    pub module_sizes: ModuleSizes,
    /// required_lookups
    pub required_lookups: Vec<LookupOp>,
    /// check mode
    pub check_mode: CheckMode,
    /// ezkl version used
    pub version: String,
    /// num blinding factors
    pub num_blinding_factors: Option<usize>,
    /// unix time timestamp
    pub timestamp: Option<u128>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
///
pub struct ModuleSizes {
    kzg: Vec<usize>,
    poseidon: (usize, Vec<usize>),
} 

pub enum CheckMode {
    #[default]
    SAFE,
    UNSAFE,
}

#[allow(missing_docs)]
/// An enum representing the operations that can be used to express more complex operations via accumulation
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize, Serialize)]
pub enum LookupOp {
    Abs,
    Div { denom: utils::F32 },
    ReLU,
    Max { scale: utils::F32, a: utils::F32 },
    Min { scale: utils::F32, a: utils::F32 },
    Ceil { scale: utils::F32 },
    Floor { scale: utils::F32 },
    Round { scale: utils::F32 },
    RoundHalfToEven { scale: utils::F32 },
    Sqrt { scale: utils::F32 },
    Rsqrt { scale: utils::F32 },
    Recip { scale: utils::F32 },
    LeakyReLU { slope: utils::F32 },
    Sigmoid { scale: utils::F32 },
    Ln { scale: utils::F32 },
    Exp { scale: utils::F32 },
    Cos { scale: utils::F32 },
    ACos { scale: utils::F32 },
    Cosh { scale: utils::F32 },
    ACosh { scale: utils::F32 },
    Sin { scale: utils::F32 },
    ASin { scale: utils::F32 },
    Sinh { scale: utils::F32 },
    ASinh { scale: utils::F32 },
    Tan { scale: utils::F32 },
    ATan { scale: utils::F32 },
    Tanh { scale: utils::F32 },
    ATanh { scale: utils::F32 },
    Erf { scale: utils::F32 },
    GreaterThan { a: utils::F32 },
    LessThan { a: utils::F32 },
    GreaterThanEqual { a: utils::F32 },
    LessThanEqual { a: utils::F32 },
    Sign,
    KroneckerDelta,
    Pow { scale: utils::F32, a: utils::F32 },
}