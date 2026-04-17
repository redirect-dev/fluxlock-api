use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

use pqcrypto_dilithium::dilithium2;

pub static KEY_STORE: Lazy<
    Mutex<HashMap<u32, (dilithium2::PublicKey, dilithium2::SecretKey)>>
> = Lazy::new(|| Mutex::new(HashMap::new()));