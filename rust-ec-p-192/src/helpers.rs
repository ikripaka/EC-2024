use crate::affine_point::EcPointA;
use crate::projective_point::EcPointP;
use crate::EcCurve;
use num_bigint::BigUint;
use num_traits::Zero;

pub fn projective_to_affine(ec: &EcCurve, a: &EcPointP) -> EcPointA {
    todo!()
}

// todo: affine -> projective
// https://math.stackexchange.com/questions/1737883/convert-affine-coordinates-to-projective-coordinates
pub fn affine_to_projective(ec: &EcCurve, a: &EcPointA) -> EcPointP {
    todo!()
}

pub fn check_discriminant(a: &BigUint, b: &BigUint, q: &BigUint) -> (BigUint, bool) {
    let d = (BigUint::from(4_u8) * a.modpow(&BigUint::from(3_u8), q)
        + BigUint::from(27_u8) * b.modpow(&BigUint::from(2_u8), q))
        % q;
    (d.clone(), d != BigUint::zero())
}

pub(crate) fn projective_add(ec_curve: &EcCurve, a: &EcPointP, b: &EcPointP) -> EcPointP {
    todo!()
}
pub(crate) fn projective_mul(ec_curve: &EcCurve, a: &EcPointP, b: &EcPointP) -> EcPointP {
    todo!()
}
