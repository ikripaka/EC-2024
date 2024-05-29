use num_bigint::BigUint;
use rust_ec::ECurve;
use rust_ec::projective_point::EcPointP;

mod diffie_hellman;
mod digital_signature;
mod directed_encryption;

#[derive(Clone, Debug)]
pub struct EcInfo{
    pub bp: EcPointP,
    /// **n** -- order of EC
    pub n: BigUint,
    pub ecurve: ECurve,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
