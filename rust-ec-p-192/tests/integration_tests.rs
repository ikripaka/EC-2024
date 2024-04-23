#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use num_traits::One;
    use rust_ec_p_192::affine_point::EcPointA;
    use rust_ec_p_192::projective_point::EcPointP;
    use rust_ec_p_192::{EcCurve, Params};

    #[test]
    fn testing_ec_creation() {
        assert!(EcCurve::new(Params {
            a: BigUint::from(1_u8),
            b: BigUint::from(1_u8),
            q: BigUint::from(7_u8),
        })
        .is_ok());

        //todo: come up with example equal to zero
        assert!(EcCurve::new(Params {
            a: BigUint::from(1_u8),
            b: BigUint::from(1_u8),
            q: BigUint::from(7_u8),
        })
        .is_ok());

        assert!(EcCurve::new(Params {
            a: BigUint::from(3_u8),
            b: BigUint::from(7_u8),
            q: BigUint::from(949_u64),
        })
        .is_ok());

        assert!(EcCurve::new(Params {
            a: BigUint::from(5_u8),
            b: BigUint::from(3_u8),
            q: BigUint::from(31_u8),
        })
        .is_ok()); // = -1

        assert!(EcCurve::new(Params {
            a: BigUint::from(8_u8),
            b: BigUint::from(1_u8),
            q: BigUint::from(11_u8),
        })
        .is_ok());

        assert!(EcCurve::new(Params {
            a: BigUint::from(1_u8),
            b: BigUint::from(1_u8),
            q: BigUint::from(31_u8),
        })
        .is_err());
    }

    #[test]
    fn ec_add() {
        let curve = EcCurve::new(Params {
            a: BigUint::from(8_u8),
            b: BigUint::from(1_u8),
            q: BigUint::from(11_u8),
        })
        .is_ok();
    }
}
