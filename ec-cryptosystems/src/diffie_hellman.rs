use num_bigint::{BigUint};
use num_traits::One;
use once_cell::sync::Lazy;
use rust_ec::projective_point::EcPointP;
use crate::{EcInfo, gen_random_biguint, TWO};

pub struct PublicKey {
    pub(crate) ec_info: EcInfo,
    shared_point: EcPointP,
}

pub struct SharedSecret {
    ec_info: EcInfo,
    shared_point: EcPointP,
}

/// **EphemeralSecret** -- A short-lived Diffie-Hellman secret key that can only be used to compute a single SharedSecret.
pub struct EphemeralSecret {
    pub(crate) ec_info: EcInfo,
    k: BigUint,
}

impl EphemeralSecret {
    pub fn random(ec_info: &EcInfo) -> EphemeralSecret {
        let k = gen_random_biguint(&Lazy::<BigUint>::get(&TWO).unwrap(),&(ec_info.n.clone() - BigUint::one()));
        EphemeralSecret{ ec_info: ec_info.clone(), k}
    }
    pub fn diffie_hellman(&self, pub_key: PublicKey) -> SharedSecret {
        SharedSecret{
            ec_info: self.ec_info.clone(),
            shared_point: self.ec_info.ecurve.proj_point_mul(&pub_key.shared_point, &self.k),
        }
    }

    pub(crate) fn get_key(&self) -> BigUint{
        self.k.clone()
    }
}

impl From<&EphemeralSecret> for PublicKey{
    fn from(value: &EphemeralSecret) -> Self {
        PublicKey{
            ec_info: value.ec_info.clone(),
            shared_point: value.ec_info.ecurve.proj_point_mul(&value.ec_info.bp, &value.k),
        }
    }
}
impl PublicKey {
    pub fn get_point_proj(&self) -> EcPointP{
        self.shared_point.clone()
    }
}

impl SharedSecret{
    pub fn get_point_proj(&self) -> EcPointP{
        self.shared_point.clone()
    }
}

// # use rand_core::OsRng;
// # use x25519_dalek::{EphemeralSecret, PublicKey};
// # let alice_secret = EphemeralSecret::new(OsRng);
// # let alice_public = PublicKey::from(&alice_secret);
// # let bob_secret = EphemeralSecret::new(OsRng);
// # let bob_public = PublicKey::from(&bob_secret);
// # let alice_shared_secret = alice_secret.diffie_hellman(&bob_public);
// # let bob_shared_secret = bob_secret.diffie_hellman(&alice_public);
// assert_eq!(alice_shared_secret.as_bytes(), bob_shared_secret.as_bytes());
