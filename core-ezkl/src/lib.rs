#![deny(
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    missing_debug_implementations,
    unsafe_code
)]
// we allow this for our dynamic range based indexing scheme
#![allow(clippy::single_range_in_vec_init)]
#![feature(round_ties_even)]

pub mod snark;
pub mod runargs;
pub mod graphsettings;


pub fn get_verifier_key<C>(vk_path: &str, params: ParamsKZG<Bn256>) -> VerifyingKey<Bn256>
where 
    C: Circuit<Scalar: Serialize + DeserializeOwned> 
    {
    //read in the path
    let mut f = File::open(vk_path).expect("File not found");
    let mut reader = BufReader::new(f);
    VerifyingKey::Bn256::read::<_, C>(
        &mut reader,
        halo2_proofs::SerdeFormat::RawBytes,
        params,
    )
    .expect()
}
