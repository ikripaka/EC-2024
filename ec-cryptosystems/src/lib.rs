use num_bigint::{BigUint, UniformBigUint};
use once_cell::sync::Lazy;
use rand::distributions::uniform::UniformSampler;
use sha3::{Digest, Sha3_256};
use rust_ec::ECurve;
use rust_ec::projective_point::EcPointP;

mod diffie_hellman;
mod digital_signature;
mod directed_encryption;

pub(crate) static TWO: Lazy<BigUint> = Lazy::new(|| {
    BigUint::from(2_u8)
});

#[derive(Clone, Debug)]
pub struct EcInfo{
    pub bp: EcPointP,
    /// **n** -- order of EC
    pub n: BigUint,
    pub ecurve: ECurve,
}

pub(crate) fn hash(m: &[u8]) -> Vec<u8>{
    let mut hasher = Sha3_256::new();
    hasher.update(m);
    hasher.finalize()[..].to_vec()
}

pub(crate) fn gen_random_biguint(low: &BigUint, high_inclusive: &BigUint) -> BigUint{
    let mut rng = rand::thread_rng();
    UniformBigUint::new_inclusive(low, high_inclusive).sample(&mut rng)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
