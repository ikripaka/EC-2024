#[cfg(test)]
mod tests {
    use num_bigint::BigInt;
    use num_traits::One;
    use rust_ec_p_192::affine_point::EcPointA;
    use rust_ec_p_192::projective_point::EcPointP;
    use rust_ec_p_192::{EcCurve, Params};

    #[test]
    fn testing_ec_creation() {
        assert!(EcCurve::new(Params {
            a: BigInt::from(1_u8),
            b: BigInt::from(1_u8),
            q: BigInt::from(7_u8),
        })
        .is_ok());

        //todo: come up with example equal to zero
        assert!(EcCurve::new(Params {
            a: BigInt::from(1_u8),
            b: BigInt::from(1_u8),
            q: BigInt::from(7_u8),
        })
        .is_ok());

        assert!(EcCurve::new(Params {
            a: BigInt::from(3_u8),
            b: BigInt::from(7_u8),
            q: BigInt::from(949_u64),
        })
        .is_ok());

        assert!(EcCurve::new(Params {
            a: BigInt::from(5_u8),
            b: BigInt::from(3_u8),
            q: BigInt::from(31_u8),
        })
        .is_ok()); // = -1

        assert!(EcCurve::new(Params {
            a: BigInt::from(8_u8),
            b: BigInt::from(1_u8),
            q: BigInt::from(11_u8),
        })
        .is_ok());

        assert!(EcCurve::new(Params {
            a: BigInt::from(1_u8),
            b: BigInt::from(1_u8),
            q: BigInt::from(31_u8),
        })
        .is_err());
    }

    #[test]
    fn negative() {
        let affine_point = EcPointA::new(&BigInt::from(11_u8), &BigInt::from(22_u8));
        assert_eq!(
            affine_point.negative(),
            EcPointA::new(&BigInt::from(11_u8), &BigInt::from(-22_i8))
        );

        let projective_point = EcPointP::new(
            &BigInt::from(11_u8),
            &BigInt::from(22_u8),
            &BigInt::from(33_u8),
        );
        assert_eq!(
            projective_point.negative(),
            EcPointP::new(
                &BigInt::from(11_u8),
                &BigInt::from(-22_i8),
                &BigInt::from(33_i8)
            )
        );
    }

    #[test]
    fn ec_add() {
        let curve = EcCurve::new(Params {
            a: BigInt::from(11_u8),
            b: BigInt::from(7_u8),
            q: BigInt::from(13_u8),
        })
        .unwrap();

        let p1 = EcPointP::new(&BigInt::from(6), &BigInt::from(4), &BigInt::from(1));
        let p2 = curve.projective_point_add(&p1, &p1);
        // EcPointP::new(&BigInt::from(10),&BigInt::from(8),&BigInt::from(1));
        assert_eq!(
            curve
                .transform_proj_point(&curve.projective_point_add(&p1, &p2))
                .unwrap(),
            EcPointP::new(&BigInt::from(11), &BigInt::from(4), &BigInt::from(1))
        );
        assert_eq!(
            curve
                .transform_proj_point(&curve.projective_point_add(&p2, &p2))
                .unwrap(),
            EcPointP::new(&BigInt::from(9), &BigInt::from(9), &BigInt::from(1))
        );

        let x_p3 = curve.projective_point_add(&p1, &p2);
        let x_p4 = curve.projective_point_add(&p2, &p2);
        let x_p5 = curve.projective_point_add(&x_p4, &p1);
        let x_p6 = curve.projective_point_add(&x_p5, &p1);
        let x_p7 = curve.projective_point_add(&x_p6, &p1);
        let x_p8 = curve.projective_point_add(&x_p7, &p1);
        let x_p9 = curve.projective_point_add(&x_p8, &p1);
        let x_p10 = curve.projective_point_add(&x_p9, &p1);
        let x_p11 = curve.projective_point_add(&x_p10, &p1);
        let x_p12 = curve.projective_point_add(&x_p11, &p1);
        // assert!(p12.is_inf(), "{p12:?}");

        let p3 = curve.projective_point_add(&p1, &p2);
        assert_eq!(
            curve.transform_proj_point(&x_p3).unwrap(),
            curve.transform_proj_point(&p3).unwrap()
        );
        let p4 = curve.projective_point_add(&p2, &p2);
        assert_eq!(
            curve.transform_proj_point(&x_p4).unwrap(),
            curve.transform_proj_point(&p4).unwrap()
        );
        let p5 = curve.projective_point_add(&p3, &p2);
        assert_eq!(
            curve.transform_proj_point(&x_p5).unwrap(),
            curve.transform_proj_point(&p5).unwrap()
        );
        let p6 = curve.projective_point_add(&p5, &p1);
        assert_eq!(
            curve.transform_proj_point(&x_p6).unwrap(),
            curve.transform_proj_point(&p6).unwrap()
        );
        let p7 = curve.projective_point_add(&p6, &p1);
        assert_eq!(
            curve.transform_proj_point(&x_p7).unwrap(),
            curve.transform_proj_point(&p7).unwrap()
        );
        let p8 = curve.projective_point_add(&p4, &p4);
        assert_eq!(
            curve.transform_proj_point(&x_p8).unwrap(),
            curve.transform_proj_point(&p8).unwrap()
        );
        let p9 = curve.projective_point_add(&p5, &p4);
        assert_eq!(
            curve.transform_proj_point(&x_p9).unwrap(),
            curve.transform_proj_point(&p9).unwrap()
        );
        let p10 = curve.projective_point_add(&p8, &p2);
        assert_eq!(
            curve.transform_proj_point(&x_p10).unwrap(),
            curve.transform_proj_point(&p10).unwrap()
        );
        let p11 = curve.projective_point_add(&p8, &p3);
        assert_eq!(&x_p11, &p11);
        let p12 = curve.projective_point_add(&p8, &p4);
        assert_eq!(
            curve.transform_proj_point(&x_p12).unwrap(),
            curve.transform_proj_point(&p12).unwrap()
        );

        assert!(
            p11.is_inf() && x_p11.is_inf(),
            "p11: {p11:?}, x_p11: {x_p11:?}"
        );

        let p12 = curve.projective_point_add(&p1, &p11);
        let p13 = curve.projective_point_add(&p12, &p1);
        assert_eq!(
            curve.transform_proj_point(&p12).unwrap(),
            curve.transform_proj_point(&p1).unwrap()
        );
        assert_eq!(
            curve.transform_proj_point(&p13).unwrap(),
            curve.transform_proj_point(&p2).unwrap()
        );

        // doubling
        let p12_d = curve.projective_point_add(&p12, &p12);
        assert_eq!(
            curve.transform_proj_point(&p12_d).unwrap(),
            curve.transform_proj_point(&p2).unwrap()
        );
        let p10_d = curve.projective_point_add(&p10, &p10);
        assert_eq!(
            curve.transform_proj_point(&p10_d).unwrap(),
            curve.transform_proj_point(&p9).unwrap()
        );
        let p6_d = curve.projective_point_add(&p6, &p6);
        assert_eq!(
            curve.transform_proj_point(&p6_d).unwrap(),
            curve.transform_proj_point(&p12).unwrap()
        );
        let p8_d = curve.projective_point_add(&p8, &p8);
        assert_eq!(
            curve.transform_proj_point(&p8_d).unwrap(),
            curve.transform_proj_point(&p5).unwrap()
        );

        // adding negative value
        let p12_n = curve.projective_point_add(&p12, &p12.negative());
        assert!(p12_n.is_inf(), "p12_n: {p12:?}");
        let p10_n = curve.projective_point_add(&p10, &p10.negative());
        assert!(p10_n.is_inf(), "p10_n: {p10_n:?}");
        let p6_n = curve.projective_point_add(&p6, &p6.negative());
        assert!(p6_n.is_inf(), "p6_n: {p6_n:?}");
        let p8_n = curve.projective_point_add(&p8, &p8.negative());
        assert!(p8_n.is_inf(), "p8_n: {p8_n:?}");
    }

    #[test]
    fn ec_mul() {
        let curve = EcCurve::new(Params {
            a: BigInt::from(8_u8),
            b: BigInt::from(1_u8),
            q: BigInt::from(11_u8),
        })
        .is_ok();
    }
}
