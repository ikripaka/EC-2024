use crate::diffie_hellman::{EphemeralSecret, PublicKey};

struct Signature{
    r: Vec<u8>,
    s: Vec<u8>,
}

struct Signer{}
impl Signer{
    pub fn sign(priv_key: &EphemeralSecret) -> (Signature, PublicKey){
        todo!()
    }
}

struct Verifier{}

impl Verifier{
    pub fn verify( m: &[u8], sign: &Signature, pub_key: &PublicKey,) -> bool{
        todo!()
    }
}