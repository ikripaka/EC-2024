#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use num_traits::One;
    use rust_ec_p_192::affine_point::EcPointA;
    use rust_ec_p_192::{EcCurve, ParamsAffine};

    #[test]
    fn testing_ec_creation() {
        let curve = EcCurve::new(ParamsAffine {
            a: BigUint::from(1_u8),
            b: BigUint::from(1_u8),
            q: BigUint::from(7_u8),
            bp: EcPointA::new(&BigUint::one(), &BigUint::one()),
        })
        .unwrap();
    }
}
