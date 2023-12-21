use graphsettings::GraphSettings;
use halo2curves::bn256::Bn256;
use halo2_proofs::poly::commitment::{Params, CommitmentScheme};
use halo2_proofs::poly::kzg::commitment::{KZGCommitmentScheme, ParamsKZG};


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