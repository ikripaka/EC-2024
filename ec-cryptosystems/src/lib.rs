use num_bigint::{BigUint, UniformBigUint};
use rand::distributions::uniform::UniformSampler;
use sha3::{Digest, Sha3_256};

pub mod diffie_hellman;
pub mod digital_signature;
pub mod directed_encryption;

pub(crate) fn hash(m: &[u8]) -> Vec<u8> {
    let mut hasher = Sha3_256::new();
    hasher.update(m);
    hasher.finalize()[..].to_vec()
}

pub(crate) fn gen_random_biguint(low: &BigUint, high_inclusive: &BigUint) -> BigUint {
    let mut rng = rand::thread_rng();
    UniformBigUint::new_inclusive(low, high_inclusive).sample(&mut rng)
}
