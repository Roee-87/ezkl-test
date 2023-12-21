use serde::{Serialize, Deserialize};
use utils::Scale;
use std::collections::BTreeMap;
use halo2curves::bn256::Fr as Fp;

/// A struct for loading from an Onnx file and converting a computational graph to a circuit.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Model {
    /// input indices
    pub graph: ParsedNodes,
    /// Defines which inputs to the model are public and private (params, inputs, outputs) using [VarVisibility].
    pub visibility: VarVisibility,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
/// A set of EZKL nodes that represent a computational graph.
pub struct ParsedNodes {
    /// The nodes in the graph.
    pub nodes: BTreeMap<usize, NodeType>,
    inputs: Vec<usize>,
    outputs: Vec<Outlet>,
}

/// Enables model as subnode of other models
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum NodeType {
    /// A node in the model
    Node(Node),
    /// A submodel
    SubGraph {
        /// The subgraph
        model: Model,
        /// The subgraph's inputs
        inputs: Vec<Outlet>,
        /// the subgraph's idx within the parent graph
        idx: usize,
        /// output mappings
        output_mappings: Vec<Vec<OutputMapping>>,
        /// input mappings
        input_mappings: Vec<InputMapping>,
        ///
        out_dims: Vec<Vec<usize>>,
        ///
        out_scales: Vec<Scale>,
    },
}

///
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum OutputMapping {
    ///
    Single {
        ///
        outlet: usize,
        ///
        is_state: bool,
    },
    ///
    Stacked {
        ///
        outlet: usize,
        ///
        axis: usize,
        ///
        is_state: bool,
    },
}

///
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum InputMapping {
    ///
    Full,
    ///
    State,
    ///
    Stacked {
        ///
        axis: usize,
        ///
        chunk: usize,
    },
}

/// A node's input is a tensor from another node's output.
pub type Outlet = (usize, usize);

/// Represents whether the model input, model parameters, and model output are Public or Private to the prover.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct VarVisibility {
    /// Input to the model or computational graph
    pub input: Visibility,
    /// Parameters, such as weights and biases, in the model
    pub params: Visibility,
    /// Output of the model or computational graph
    pub output: Visibility,
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

/// A single operation in a [crate::graph::Model].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    /// [Op] i.e what operation this node represents.
    pub opkind: SupportedOp,
    /// The denominator in the fixed point representation for the node's output. Tensors of differing scales should not be combined.
    pub out_scale: i32,
    // Usually there is a simple in and out shape of the node as an operator.  For example, an Affine node has three input_shapes (one for the input, weight, and bias),
    // but in_dim is [in], out_dim is [out]
    /// The indices of the node's inputs.
    pub inputs: Vec<Outlet>,
    /// Dimensions of output.
    pub out_dims: Vec<usize>,
    /// The node's unique identifier.
    pub idx: usize,
    /// The node's num of uses
    pub num_uses: usize,
}

/// A single operation in a [crate::graph::Model].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SupportedOp {
    /// A linear operation.
    Linear(PolyOp<Fp>),
    /// A nonlinear operation.
    Nonlinear(LookupOp),
    /// A hybrid operation.
    Hybrid(HybridOp),
    ///
    Input(Input),
    ///
    Constant(Constant<Fp>),
    ///
    Unknown(Unknown),
    ///
    Rescaled(Rescaled),
    ///
    RebaseScale(RebaseScale),
}