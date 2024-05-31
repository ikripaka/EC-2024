use crate::diffie_hellman::{EphemeralSecret, PublicKey, SharedSecret};
use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{AeadCore, Aes256Gcm, Key, KeyInit, Nonce};
use rust_ec::affine_point;

const BYTE_KEY_LEN: usize = 32;
const NONCE_LEN: usize = 12;

pub struct EncryptedMessage {
    ct_m: Vec<u8>,
    ct_k: Vec<u8>,
}

pub struct Encryptor {}

impl Encryptor {
    pub fn encrypt(msg: &[u8], shared_secret: &SharedSecret) -> EncryptedMessage {
        let key_bytes: [u8; BYTE_KEY_LEN] = rand::random();
        let ct_m = enc(&key_bytes, msg);

        let (proj_shared_point, ec_info) =
            (shared_secret.get_point_proj(), shared_secret.get_ec_info());
        let affine_shared_point = proj_shared_point
            .to_affine(&ec_info.ecurve)
            .expect("Failed to extract affine point.");
        let ct_k = enc(
            &{
                let mut key = affine_shared_point.get_x().to_bytes_be().1;
                if key.len() < BYTE_KEY_LEN {
                    key.extend_from_slice(&vec![0; BYTE_KEY_LEN - key.len()])
                }
                key
            },
            &key_bytes,
        );

        EncryptedMessage { ct_m, ct_k }
    }

    pub fn decrypt(enc_msg: &EncryptedMessage, shared_secret: &SharedSecret) -> Vec<u8> {
        let (proj_shared_point, ec_info) =
            (shared_secret.get_point_proj(), shared_secret.get_ec_info());
        let affine_shared_point = proj_shared_point
            .to_affine(&ec_info.ecurve)
            .expect("Failed to extract affine point.");

        let pt_k = dec(
            &{
                let mut key = affine_shared_point.get_x().to_bytes_be().1;
                if key.len() < BYTE_KEY_LEN {
                    key.extend_from_slice(&vec![0; BYTE_KEY_LEN - key.len()])
                }
                key
            },
            &enc_msg.ct_k,
        );
        let pt_m = dec(&pt_k, &enc_msg.ct_m);

        pt_m
    }
}

fn enc(key: &[u8], msg: &[u8]) -> Vec<u8> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher = Aes256Gcm::new(key);
    let ciphertext = cipher.encrypt(&nonce, msg).expect("Failed to encrypt");

    // combining nonce and encrypted data together
    let mut encrypted_data: Vec<u8> = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphertext);

    encrypted_data
}

fn dec(key: &[u8], ct: &[u8]) -> Vec<u8> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let (nonce_arr, ciphered_data) = ct.split_at(NONCE_LEN);
    let nonce = Nonce::from_slice(nonce_arr);
    let cipher = Aes256Gcm::new(key);

    cipher
        .decrypt(nonce, ciphered_data)
        .expect("failed to decrypt data")
}

#[cfg(test)]
mod tests {
    use crate::directed_encryption::{dec, enc, BYTE_KEY_LEN};

    #[test]
    fn enc_test() {
        let key_bytes: [u8; BYTE_KEY_LEN] = rand::random();
        let msg = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F,
        ];
        let ct = enc(&key_bytes, &msg);
        let pt = dec(&key_bytes, &ct);
        assert_eq!(msg.as_slice(), pt.as_slice())
    }
}
