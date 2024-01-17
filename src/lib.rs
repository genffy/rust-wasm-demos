use std::panic;

use base64::engine::general_purpose::URL_SAFE;
use base64::Engine as _;
use rsa::pkcs8::DecodePublicKey;
use rsa::Pkcs1v15Encrypt;
use rsa::RsaPublicKey;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[no_mangle]
pub fn encrypt(data: String) -> String {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let pub_key = "-----BEGIN PUBLIC KEY-----
<REPLACED WITH YOUR OWN `public_key.pem` CONTENT>
-----END PUBLIC KEY-----";

    let mut rng = rand::thread_rng();
    let pub_key = RsaPublicKey::from_public_key_pem(pub_key).expect("failed to decode public key");

    let curr_time = js_sys::Date::now().to_string();

    let new_data = data + "_" + &curr_time;
    let enc_data = pub_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, new_data.as_bytes())
        .expect("failed to encrypt");

    URL_SAFE.encode(enc_data)
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    use crate::encrypt;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn encode_with_empty_string() {
        let enc_str = encrypt("".to_string());
        println!("{:?}", enc_str);
        assert!(!enc_str.is_empty());
    }

    #[wasm_bindgen_test]
    fn encode_with_string() {
        let enc_str = encrypt("hello".to_string());
        println!("{:?}", enc_str);
        assert!(!enc_str.is_empty());
    }
}
