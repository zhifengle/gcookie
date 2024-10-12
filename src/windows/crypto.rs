use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use windows::Win32::Foundation::{LocalFree, HLOCAL};
use windows::Win32::Security::Cryptography::{CryptUnprotectData, CRYPT_INTEGER_BLOB};

pub fn crypt_unprotect_data(crypted_bytes: &[u8]) -> windows::core::Result<Vec<u8>> {
    let len = crypted_bytes.len();
    let mut bytes = Vec::from(crypted_bytes);
    let pb = bytes.as_mut_ptr();
    let mut blob = CRYPT_INTEGER_BLOB {
        pbData: pb,
        cbData: len as u32,
    };
    let mut out = Vec::with_capacity(len);
    let mut blob_out = CRYPT_INTEGER_BLOB {
        pbData: out.as_mut_ptr(),
        cbData: out.len() as u32,
    };
    unsafe {
        CryptUnprotectData(&mut blob, None, None, None, None, 0, &mut blob_out).ok();

        let slice = std::slice::from_raw_parts(blob_out.pbData, blob_out.cbData as usize);
        // LocalFree(blob.pbData as isize);
        LocalFree(HLOCAL(blob_out.pbData.cast()));
        Ok(slice.to_vec())
    }
}

pub fn aes_gcm_decrypt(value: &[u8], key: &[u8], nonce: &[u8]) -> Vec<u8> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    let nonce = Nonce::from_slice(nonce);

    let plaintext = cipher
        .decrypt(nonce, value)
        .expect("decryption aes_gcm value failure!");
    plaintext
}
