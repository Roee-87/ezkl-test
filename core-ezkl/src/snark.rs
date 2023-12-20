use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use halo2curves::serde::SerdeObject;
use halo2curves::ff::{FromUniformBytes, PrimeField, WithSmallOrderMulGroup};


/// An application snark with proof and instance variables ready for aggregation (raw field element)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snark<F: PrimeField + SerdeObject> {
    #[serde(skip)]
    pub protocol: String,
    /// public instances of the snark
    pub instances: Vec<Vec<F>>,
    /// the proof
    pub proof: Vec<u8>,
    /// transcript type
    #[serde(skip)]
    pub transcript_type: String,
    /// the split proof
    #[serde(skip)]
    pub split: String,
}

impl<F: PrimeField + SerdeObject + Serialize + FromUniformBytes<64> + DeserializeOwned> Snark<F>
{
    pub fn load(
        proof_path: &str,
    ) -> Self 
    {
        // let mut f1 = File::open(proof_path).expect("Could not find path");
        // let mut json_file = String::new();
        // f1.read_to_string(&mut json_file).expect("Unable to read to string");
        serde_json::from_str(proof_path).unwrap()
    }
}
impl<Scalar:  SerdeObject + PrimeField + FromUniformBytes<64> + WithSmallOrderMulGroup<3> + Ord + Serialize + DeserializeOwned> Snark<Scalar> {
    pub fn format_instances(&self)
    {
        let pi_inner = self
        .instances
        .iter()
        .map(|e| e.deref())
        .collect::<Vec<&[Scalar]>>();
        let instances: &[&[&[Scalar]]] = &[&pi_inner];
        println!("Instances: {:?}", &instances);
    }
}