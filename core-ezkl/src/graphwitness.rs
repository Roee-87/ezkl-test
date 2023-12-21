use halo2curves::bn256::{Bn256, Fr as Fp, G1Affine};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct GraphWitness {
    /// The inputs of the forward pass
    pub inputs: Vec<Vec<Fp>>,
    /// The prettified outputs of the forward pass, we use a String to maximize compatibility with Python and JS clients
    pub pretty_elements: Option<PrettyElements>,
    /// The output of the forward pass
    pub outputs: Vec<Vec<Fp>>,
    /// Any hashes of inputs generated during the forward pass
    pub processed_inputs: Option<ModuleForwardResult>,
    /// Any hashes of params generated during the forward pass
    pub processed_params: Option<ModuleForwardResult>,
    /// Any hashes of outputs generated during the forward pass
    pub processed_outputs: Option<ModuleForwardResult>,
    /// max lookup input
    pub max_lookup_inputs: i128,
    /// max lookup input
    pub min_lookup_inputs: i128,
}

/// Result from a forward pass
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModuleForwardResult {
    /// The inputs of the forward pass for poseidon
    pub poseidon_hash: Option<Vec<Fp>>,
    /// The outputs of the forward pass for KZG
    pub kzg_commit: Option<Vec<Vec<G1Affine>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
/// Contains the instances of the circuit in human readable form
pub struct PrettyElements {
    /// the inputs as rescaled floats -- represented as a String for maximum compatibility with Python and JS
    pub rescaled_inputs: Vec<Vec<String>>,
    /// the inputs as felts but 0x strings -- represented as a String for maximum compatibility with Python and JS
    pub inputs: Vec<Vec<String>>,
    /// the processed inputs (eg. hash of the inputs) -- stays as a felt represented as a 0x string for maximum compatibility with Python and JS
    pub processed_inputs: Vec<Vec<String>>,
    /// the processed params (eg. hash of the params) -- stays as a felt represented as a 0x string for maximum compatibility with Python and JS
    pub processed_params: Vec<Vec<String>>,
    /// the processed outputs (eg. hash of the outputs) -- stays as a felt represented as a 0x string for maximum compatibility with Python and JS
    pub processed_outputs: Vec<Vec<String>>,
    /// the outputs as rescaled floats (if any) -- represented as a String for maximum compatibility with Python and JS
    pub rescaled_outputs: Vec<Vec<String>>,
    /// the outputs as felts but 0x strings (if any) -- represented as a String for maximum compatibility with Python and JS
    pub outputs: Vec<Vec<String>>,
}