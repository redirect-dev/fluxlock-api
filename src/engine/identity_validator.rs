use pqcrypto_dilithium::dilithium2;
use pqcrypto_traits::sign::{PublicKey, SecretKey, DetachedSignature};

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

    let (_old_pk, old_sk) = store.get(&id).unwrap().clone();

    // 🔥 FIXED FUNCTION NAME
    let signature = dilithium2::detached_sign(new_message, &old_sk);

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
    let pk = match dilithium2::PublicKey::from_bytes(old_pk_bytes) {
        Ok(pk) => pk,
        Err(_) => return false,
    };

    let sig = match dilithium2::DetachedSignature::from_bytes(sig_bytes) {
        Ok(sig) => sig,
        Err(_) => return false,
    };

    dilithium2::verify_detached_signature(&sig, new_message, &pk).is_ok()
}

// =========================
// 🧠 VALIDATION LOGIC (REQUIRED BY API)
// =========================
pub struct ValidationResult {
    pub valid: bool,
    pub reason: String,
}

pub fn validate_identity_logic(
    trust: f64,
    drift: f64,
    epoch_age: u64,
    epoch_valid: bool,
    compromised: bool,
    network_accepted: bool,
) -> ValidationResult {

    // 🔴 HARD FAILS
    if compromised {
        return ValidationResult {
            valid: false,
            reason: "identity compromised".into(),
        };
    }

    if !epoch_valid {
        return ValidationResult {
            valid: false,
            reason: "invalid identity chain".into(),
        };
    }

    // 🔥 MATURITY GATE
    if epoch_age < 120 {
        return ValidationResult {
            valid: false,
            reason: "identity too new (maturing)".into(),
        };
    }

    // 🔥 NETWORK CONSENSUS REQUIRED
    if !network_accepted {
        return ValidationResult {
            valid: false,
            reason: "network has not accepted identity".into(),
        };
    }

    // 🔴 INSTABILITY
    if drift > 80.0 {
        return ValidationResult {
            valid: false,
            reason: "unstable identity (high drift)".into(),
        };
    }

    // 🟡 RECOVERY
    if trust < 60.0 {
        return ValidationResult {
            valid: true,
            reason: "recovering identity".into(),
        };
    }

    // 🟢 HEALTHY
    ValidationResult {
        valid: true,
        reason: "identity stable and verified".into(),
    }
}