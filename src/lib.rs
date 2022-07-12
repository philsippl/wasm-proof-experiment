use std::panic;

use semaphore::{identity::Identity, Field, poseidon_tree::PoseidonTree, hash_to_field, protocol::{generate_nullifier_hash, generate_proof, verify_proof}};

use wasm_bindgen::prelude::*;
use js_sys::Promise;

pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn xxx(n: usize) -> Promise {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    init_thread_pool(n)
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    let id = Identity::from_seed(b"secret");

    // generate merkle tree
    let leaf = Field::from(0_u32);
    let mut tree = PoseidonTree::new(21, leaf);
    tree.set(0, id.commitment());

    let merkle_proof = tree.proof(0).expect("proof should exist");

    // change signal and external_nullifier here
    let signal_hash = hash_to_field(b"xxx");
    let external_nullifier_hash = hash_to_field(b"appId");

    let nullifier_hash = generate_nullifier_hash(&id, external_nullifier_hash);

    let witness = semaphore::protocol::generate_witness(&id, &merkle_proof, external_nullifier_hash, signal_hash).unwrap();

    //////
    
    let proof = semaphore::protocol::generate_proof_with_witness(witness).unwrap();

    // generate_proof(&id, &merkle_proof, external_nullifier_hash, signal_hash);

    return format!("{:?}", proof);
}
