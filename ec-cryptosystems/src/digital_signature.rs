use crate::diffie_hellman::{EphemeralSecret, PublicKey};
use crate::{gen_random_biguint, hash};
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::{ToBytes, Zero};
use rust_ec::affine_point::EcPointA;
use rust_ec::EcInfo;
use rust_ec::helpers::{inverse, take_by_bigint_module, take_by_biguint_module};
use sha3::{Digest, Sha3_256};

pub struct Signature {
    r: Vec<u8>,
    s: Vec<u8>,
}

pub struct Signer {}

impl Signer {
    pub fn sign(m: &[u8], priv_key: &EphemeralSecret) -> Signature {
        let digest_num = BigInt::from_bytes_be(Sign::Plus, &hash(m));
        let (_one_time_priv_key, _pub_key, r, k) = {
            let (mut r, mut k, mut one_time_priv_key, mut pub_key) =
                (BigInt::zero(), BigUint::zero(), None, None);
            while r == BigInt::zero() {
                let _ = one_time_priv_key.insert(EphemeralSecret::random(&priv_key.ec_info));
                let _ = pub_key.insert(PublicKey::from(one_time_priv_key.as_ref().unwrap()));
                let k_p: EcPointA = pub_key
                    .as_ref()
                    .unwrap()
                    .get_point_proj()
                    .to_affine(&one_time_priv_key.as_ref().unwrap().ec_info.ecurve)
                    .expect("Unable to generate inverse in signing!");
                k = one_time_priv_key.as_ref().unwrap().get_key();
                r = take_by_biguint_module(
                    &k_p.get_x(),
                    &one_time_priv_key.as_ref().unwrap().ec_info.n,
                );
            }
            (one_time_priv_key.unwrap(), pub_key.unwrap(), r, k)
        };
        let k_inv = inverse(&BigInt::from(k), &BigInt::from(priv_key.ec_info.n.clone()))
            .expect("Failed to generate inverse in creating sign.");
        let d_a = BigInt::from(priv_key.get_key());
        let s = take_by_biguint_module(
            &(k_inv * (digest_num + d_a * r.clone())),
            &priv_key.ec_info.n,
        );

        Signature {
            r: r.to_bytes_be().1.to_vec(),
            s: s.to_bytes_be().1.to_vec(),
        }
    }
}

pub struct Verifier {}

impl Verifier {
    pub fn verify(m: &[u8], sign: &Signature, pub_key: &PublicKey) -> bool {
        let digest = BigInt::from_bytes_be(Sign::Plus, &hash(m));
        let n = BigInt::from(pub_key.ec_info.n.clone());
        let ( r, s) = (
            BigInt::from_bytes_be(Sign::Plus, &sign.r),
            BigInt::from_bytes_be(Sign::Plus, &sign.s),
        );
        let s_inv = inverse(&s, &n).expect("Unable to generate inverse in sign check!");
        let (u1, u2): (BigInt, BigInt) = (
            take_by_bigint_module(&(s_inv.clone() * digest), &n),
            take_by_bigint_module(&(s_inv.clone() * &r), &n),
        );

        let (part1, part2) = (
            pub_key
                .ec_info
                .ecurve
                .proj_point_mul(&pub_key.ec_info.bp, &u1.to_biguint().unwrap()),
            pub_key
                .ec_info
                .ecurve
                .proj_point_mul(&pub_key.get_point_proj(), &u2.to_biguint().unwrap()),
        );

        let lhs = pub_key.ec_info
            .ecurve.proj_point_add(&part1, &part2);

        take_by_biguint_module(&lhs.to_affine(&pub_key.ec_info
            .ecurve).unwrap().get_x(), &pub_key.ec_info.n) == r
    }
}
