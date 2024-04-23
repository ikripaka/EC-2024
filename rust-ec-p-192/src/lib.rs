pub mod affine_point;
pub mod helpers;
pub mod projective_point;

use crate::affine_point::EcPointA;
use crate::helpers::{check_discriminant, projective_add, projective_mul};
use crate::projective_point::EcPointP;
use num_bigint::BigUint;
use num_traits::Zero;
use std::convert::Into;
use std::fmt::{Display, Formatter};

/// **ECCurve** -- represents elliptic curve in Weierstrass form
/// points satisfy the following equation
/// y^2 = x^3 + ax + b or in projective coordinates Y^{2}Z = X^{3} + aXZ^{2} + bZ^3
/// and EC discriminant has to be not equal to zero, i.e. 4a^3 + 27b^2 mod q != 0
pub struct EcCurve {
    a: BigUint,
    b: BigUint,
    q: BigUint,
}

#[derive(Debug)]
pub struct Params {
    pub a: BigUint,
    pub b: BigUint,
    pub q: BigUint,
}

#[derive(Debug)]
pub enum EcError {
    IncorrectParameters(String),
    NonZeroDiscriminant(BigUint),
}

pub type Result<T> = core::result::Result<T, EcError>;

impl Display for EcError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EcError::IncorrectParameters(str) => format!("Can't parse EC parameters: {str}"),
                EcError::NonZeroDiscriminant(x) =>
                    format!("Discriminant doesn't equal to zero [{x} != 0]"),
            }
        )
    }
}

impl Params {
    fn check_discriminant(&self) -> Result<()> {
        check_discriminant(&self.a, &self.b, &self.q)
    }
}

impl EcCurve {
    pub fn new(params: Params) -> Result<Self> {
        if let Err(e) = params.check_discriminant() {
            return Err(e);
        }
        let ec = EcCurve {
            a: params.a,
            b: params.b,
            q: params.q,
        };
        Ok(ec)
    }

    // y^2 = x^3 + ax + b
    pub fn check_affine_point(&self, p: &EcPointA) -> bool {
        p.y.modpow(&BigUint::from(2_u8), &self.q)
            == p.x.modpow(&BigUint::from(3_u8), &self.q) + &self.a * &p.x + &self.b
    }

    // Y^{2}Z = X^{3} + aXZ^{2} + bZ^3,
    pub fn check_projective_point(&self, p: &EcPointP) -> bool {
        (p.y.modpow(&BigUint::from(2_u8), &self.q) * &p.z) % &self.q
            == p.x.modpow(&BigUint::from(3_u8), &self.q)
                + &self.a * &p.x * p.z.modpow(&BigUint::from(2_u8), &self.q)
                + &self.b * p.z.modpow(&BigUint::from(3_u8), &self.q)
    }

    pub fn affine_point_add(&self, a: &EcPointA, b: &EcPointA) -> EcPointA {
        self.projective_point_add(&a.to_projective(self), &b.to_projective(self))
            .to_affine(self)
    }
    pub fn affine_point_mul(&self, a: &EcPointA, b: &EcPointA) -> EcPointA {
        self.projective_point_mul(&a.to_projective(self), &b.to_projective(self))
            .to_affine(self)
    }
    pub fn projective_point_add(&self, a: &EcPointP, b: &EcPointP) -> EcPointP {
        projective_add(self, a, b)
    }
    pub fn projective_point_mul(&self, a: &EcPointP, b: &EcPointP) -> EcPointP {
        projective_mul(self, a, b)
    }
}
