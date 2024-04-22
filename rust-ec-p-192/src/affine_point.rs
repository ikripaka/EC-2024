use crate::helpers::{affine_to_projective, projective_to_affine};
use crate::projective_point::EcPointP;
use crate::EcCurve;
use num_bigint::BigUint;

mod addition;
mod helpers;
mod multiplication;

#[derive(Debug)]
pub struct EcPointA {
    pub x: BigUint,
    pub y: BigUint,
}

impl EcPointA {
    pub fn new(x: &BigUint, y: &BigUint) -> Self {
        EcPointA {
            x: x.clone(),
            y: y.clone(),
        }
    }

    pub fn from_projective(a: &EcPointP, ec_curve: &EcCurve) -> Self {
        projective_to_affine(ec_curve, a)
    }

    pub fn to_projective(&self, ec_curve: &EcCurve) -> EcPointP {
        affine_to_projective(ec_curve, self)
    }
}
