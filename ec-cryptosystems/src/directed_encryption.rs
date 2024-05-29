use crate::diffie_hellman::{PublicKey, SharedSecret};

struct EncryptedMessage{
    ct: Vec<u8>,
    encapsulated_key: Vec<u8>,
    pub_key: PublicKey,
}
struct Encryptor{}

impl Encryptor{
    pub fn encrypt(shared_secret: &SharedSecret, msg: &[u8]) -> EncryptedMessage{
        todo!()
    }

    pub fn decrypt(shared_secret: &SharedSecret, enc_msg: &EncryptedMessage) -> Option<Vec<u8>>{
        todo!()
    }
}