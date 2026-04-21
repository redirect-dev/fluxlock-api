use pqcrypto_dilithium::dilithium2;
use pqcrypto_traits::sign::{PublicKey, SecretKey, SignedMessage, DetachedSignature};

use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

// =========================
// 🔐 GLOBAL KEY STORE
// =========================
pub static KEY_STORE: Lazy<
    Mutex<HashMap<u32, (dilithium2::PublicKey, dilithium2::SecretKey)>>
> = Lazy::new(|| Mutex::new(HashMap::new()));

// =========================
// 🔑 CREATE IDENTITY
// =========================
pub fn generate_identity(id: u32) -> Vec<u8> {
    let (pk, sk) = dilithium2::keypair();

    let mut store = KEY_STORE.lock().unwrap();
    store.insert(id, (pk.clone(), sk));

    pk.as_bytes().to_vec()
}

// =========================
// 🔁 ROTATE IDENTITY (SIGNED)
// =========================
pub fn rotate_identity(id: u32, new_message: &[u8]) -> Vec<u8> {
    let mut store = KEY_STORE.lock().unwrap();

    let (old_pk, old_sk) = store.get(&id).unwrap().clone();

    // sign new identity with old key
    let signature = dilithium2::sign_detached(new_message, &old_sk);

    // generate new keypair
    let (new_pk, new_sk) = dilithium2::keypair();

    store.insert(id, (new_pk.clone(), new_sk));

    signature.as_bytes().to_vec()
}

// =========================
// ✅ VERIFY CHAIN LINK
// =========================
pub fn verify_link(
    old_pk_bytes: &[u8],
    new_message: &[u8],
    sig_bytes: &[u8],
) -> bool {
    let pk = dilithium2::PublicKey::from_bytes(old_pk_bytes).unwrap();
    let sig = dilithium2::DetachedSignature::from_bytes(sig_bytes).unwrap();

    dilithium2::verify_detached_signature(&sig, new_message, &pk).is_ok()
}