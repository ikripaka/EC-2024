pub mod affine_point;
pub mod helpers;
pub mod projective_point;

use crate::affine_point::EcPointA;
use crate::helpers::{check_discriminant, inverse, projective_add, projective_mul, take_by_module};
use crate::projective_point::EcPointP;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::fmt::{Display, Formatter};

/// **ECCurve** -- represents elliptic curve in Weierstrass form
/// points satisfy the following equation
/// y^2 = x^3 + ax + b or in projective coordinates Y^{2}Z = X^{3} + aXZ^{2} + bZ^3
/// and EC discriminant has to be not equal to zero, i.e. 4a^3 + 27b^2 mod q != 0
pub struct EcCurve {
    a: BigInt,
    b: BigInt,
    q: BigInt,
}

#[derive(Debug)]
pub struct Params {
    pub a: BigInt,
    pub b: BigInt,
    pub q: BigInt,
}

#[derive(Debug)]
pub enum EcError {
    IncorrectParameters(String),
    NonZeroDiscriminant(BigInt),
    ImpossibleToFindInverse(String),
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
                EcError::ImpossibleToFindInverse(msg) =>
                    format!("Impossible to find inverse for {msg}"),
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
        params.check_discriminant()?;
        let ec = EcCurve {
            a: params.a,
            b: params.b,
            q: params.q,
        };
        Ok(ec)
    }

    // y^2 = x^3 + ax + b
    pub fn check_affine_point(&self, p: &EcPointA) -> bool {
        p.y.modpow(&BigInt::from(2_u8), &self.q)
            == p.x.modpow(&BigInt::from(3_u8), &self.q) + &self.a * &p.x + &self.b
    }

    // Y^{2}Z = X^{3} + aXZ^{2} + bZ^3,
    pub fn check_projective_point(&self, p: &EcPointP) -> bool {
        (p.y.modpow(&BigInt::from(2_u8), &self.q) * &p.z) % &self.q
            == p.x.modpow(&BigInt::from(3_u8), &self.q)
                + &self.a * &p.x * p.z.modpow(&BigInt::from(2_u8), &self.q)
                + &self.b * p.z.modpow(&BigInt::from(3_u8), &self.q)
    }

    pub fn affine_point_add(&self, a: &EcPointA, b: &EcPointA) -> Result<EcPointA> {
        self.projective_point_add(&a.to_projective(), &b.to_projective())
            .to_affine(self)
    }
    pub fn affine_point_mul(&self, a: &EcPointA, b: &EcPointA) -> Result<EcPointA> {
        self.projective_point_mul(&a.to_projective(), &b.to_projective())
            .to_affine(self)
    }
    pub fn projective_point_add(&self, a: &EcPointP, b: &EcPointP) -> EcPointP {
        projective_add(self, a, b)
    }
    pub fn projective_point_mul(&self, a: &EcPointP, b: &EcPointP) -> EcPointP {
        projective_mul(self, a, b)
    }

    /// **transform_proj_point** -- transforms projective point Z coordinate into 1
    pub fn transform_proj_point(&self, p: &EcPointP) -> crate::Result<EcPointP> {
        let mut p = p.clone();
        let inv = inverse(&take_by_module(&p.z, &self.q), &self.q)?;
        p.x = take_by_module(&((&p.x * &inv) % &self.q), &self.q);
        p.y = take_by_module(&((&p.y * &inv) % &self.q), &self.q);
        p.z = BigInt::one();
        Ok(p)
    }
}
