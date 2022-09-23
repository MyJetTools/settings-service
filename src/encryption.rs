pub struct AesKey {
    pub key: [u8; 32],
    pub iv: [u8; 16],
}

impl AesKey {
    pub fn new(key: &[u8]) -> AesKey {
        if key.len() != 48 {
            panic!("AesKey: key must be 48 bytes");
        }

        let mut aes_key = AesKey {
            key: [0; 32],
            iv: [0; 16],
        };
        aes_key.key.copy_from_slice(&key[..32]);
        aes_key.iv.copy_from_slice(&key[32..]);
        aes_key
    }

    pub fn get_cipher(&self) -> libaes::Cipher {
        libaes::Cipher::new_256(&self.key)
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let cipher = self.get_cipher();
        cipher.cbc_encrypt(&self.iv, data)
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let cipher = self.get_cipher();

        let result = std::panic::catch_unwind(|| cipher.cbc_decrypt(&self.iv, data));
        match result {
            Ok(result) => Ok(result),
            Err(err) => Err(format!("AesKey: decryption failed: {:?}", err)),
        }
    }
}
