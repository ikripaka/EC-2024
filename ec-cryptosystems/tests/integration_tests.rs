#[cfg(test)]
mod tests {
    use ec_cryptosystems::diffie_hellman::{EphemeralSecret, PublicKey};
    use ec_cryptosystems::digital_signature::{Signer, Verifier};
    use ec_cryptosystems::directed_encryption::Encryptor;
    use num_bigint::{BigInt, BigUint};
    use num_traits::{Num, One};
    use rust_ec::affine_point::EcPointA;
    use rust_ec::projective_point::EcPointP;
    use rust_ec::{ECurve, EcInfo, Params, PreGeneratedParams};

    #[test]
    fn sign() {
        let ec = EcInfo::from(PreGeneratedParams::P192);

        let alice_secret = EphemeralSecret::random(&ec);
        let alice_pub_key = PublicKey::from(&alice_secret);
        let msg = "Checking message for integrity".as_bytes().to_vec();
        let flawed_msg = "Check1ng message for 1ntegr1ty".as_bytes().to_vec();

        let sign = Signer::sign(&msg, &alice_secret);
        assert!(Verifier::verify(&msg, &sign, &alice_pub_key));
        assert!(!Verifier::verify(&flawed_msg, &sign, &alice_pub_key));
    }

    #[test]
    fn diffie_hellman() {
        let ec = EcInfo::from(PreGeneratedParams::P192);

        let alice_secret = EphemeralSecret::random(&ec);
        let alice_pub_key = PublicKey::from(&alice_secret);

        let bob_secret = EphemeralSecret::random(&ec);
        let bob_pub_key = PublicKey::from(&bob_secret);

        let alice_shared_secret = alice_secret.diffie_hellman(bob_pub_key);
        let bob_shared_secret = bob_secret.diffie_hellman(alice_pub_key);

        assert_eq!(alice_shared_secret, bob_shared_secret)
    }

    #[test]
    fn enc_dec() {
        let ec = EcInfo::from(PreGeneratedParams::P192);
        let msg = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F,
        ];

        let alice_secret = EphemeralSecret::random(&ec);
        let alice_pub_key = PublicKey::from(&alice_secret);

        let bob_secret = EphemeralSecret::random(&ec);
        let bob_pub_key = PublicKey::from(&bob_secret);

        let alice_shared_secret = alice_secret.diffie_hellman(bob_pub_key);
        let bob_shared_secret = bob_secret.diffie_hellman(alice_pub_key);

        let ct = Encryptor::encrypt(&msg, &alice_shared_secret);
        let pt = Encryptor::decrypt(&ct, &bob_shared_secret);

        assert_eq!(msg.as_slice(), pt.as_slice())
    }
}
