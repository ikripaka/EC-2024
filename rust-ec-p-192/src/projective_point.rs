use crate::affine_point::EcPointA;
use crate::helpers::{affine_to_projective, projective_to_affine};
use crate::EcCurve;
use num_bigint::BigUint;

mod addition;
mod helpers;
mod multiplication;

// O_e = (0, 1, 0)
#[derive(Debug, Clone)]
pub struct EcPointP {
    pub x: BigUint,
    pub y: BigUint,
    pub z: BigUint,
}

impl EcPointP {
    pub fn new(x: &BigUint, y: &BigUint, z: &BigUint) -> Self {
        EcPointP {
            x: x.clone(),
            y: y.clone(),
            z: z.clone(),
        }
    }

    pub fn from_affine(a: &EcPointA, ec_curve: &EcCurve) -> Self {
        affine_to_projective(ec_curve, a)
    }

    pub fn to_affine(&self, ec_curve: &EcCurve) -> EcPointA {
        projective_to_affine(ec_curve, self)
    }
}
