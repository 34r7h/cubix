//! Implements NIST-like API wrappers for MAYO cryptographic operations.
use wasm_bindgen::prelude::*;

use crate::types::{CompactSecretKey, CompactPublicKey, Message, Signature, ExpandedSecretKey, ExpandedPublicKey};
use crate::params::MayoParams; // MayoVariantParams is accessed via MayoParams.variant()
use crate::keygen::{compact_key_gen, expand_sk, expand_pk};
use crate::sign::sign_message;
use crate::verify::verify_signature;

/// Generates a compact key pair (secret key, public key) for the specified MAYO variant.
/// This wraps `MAYO.CompactKeyGen`.
#[wasm_bindgen]
pub fn keypair(params_enum: &MayoParams) -> Result<(CompactSecretKey, CompactPublicKey), String> {
    compact_key_gen(params_enum).map_err(|e| e.to_string())
}

/// Signs a message using a compact secret key.
/// This involves expanding the secret key and then calling `MAYO.Sign`.
/// The returned signature does not include the message.
#[wasm_bindgen]
pub fn sign(csk: &CompactSecretKey, message: &Message, params_enum: &MayoParams) -> Result<Signature, String> {
    // Note: The problem description mentions ExpandedSecretKey is not used by sign.
    // However, the provided function signature for sign_message in sign.rs *does* take ExpandedSecretKey.
    // Algorithm 8 (MAYO.Sign) takes esk as input.
    // Algorithm 3 (NIST API Sign) takes sk (csk) as input, implying internal expansion.
    // So, expanding sk to esk here is correct.
    let esk: ExpandedSecretKey = expand_sk(csk, params_enum).map_err(|e| e.to_string())?;
    sign_message(&esk, message, params_enum).map_err(|e| e.to_string())
}

/// Verifies a signature on a "signed message" and recovers the original message if valid.
/// This corresponds to `sign_open` in some APIs.
/// Assumes `signed_message` is `signature_bytes || original_message_bytes`.
#[wasm_bindgen]
pub fn open(cpk: &CompactPublicKey, signed_message: &[u8], params_enum: &MayoParams) -> Result<Option<Message>, String> {
    let params = params_enum.variant();
    
    // Determine signature length: s_bytes_len (n elements) + salt_bytes
    let s_bytes_len = MayoParams::bytes_for_gf16_elements(params.n);
    let expected_sig_len = s_bytes_len + params.salt_bytes;

    if signed_message.len() < expected_sig_len {
        return Err("Signed message is too short to contain a signature".to_string());
    }

    let sig_bytes = &signed_message[0..expected_sig_len];
    let message_bytes = &signed_message[expected_sig_len..];

    let signature = Signature(sig_bytes.to_vec());
    let original_message = Message(message_bytes.to_vec());

    // Note: The problem description mentions ExpandedPublicKey is not used by verify.
    // However, the provided function signature for verify_signature in verify.rs *does* take ExpandedPublicKey.
    // Algorithm 9 (MAYO.Verify) takes epk as input.
    // Algorithm 4 (NIST API Verify/Open) takes pk (cpk) as input, implying internal expansion.
    // So, expanding pk to epk here is correct.
    let epk: ExpandedPublicKey = expand_pk(cpk, params_enum).map_err(|e| e.to_string())?;
    
    match verify_signature(&epk, &original_message, &signature, params_enum) {
        Ok(true) => Ok(Some(original_message)), // Valid signature, return message
        Ok(false) => Ok(None),                  // Invalid signature
        Err(e) => Err(e.to_string()),                       // Error during verification
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::MayoParams; // This is MayoParams enum type itself
    // Message and Signature types are already imported if needed.

    #[test]
    fn test_keypair_api() {
        // Test for MAYO1
        let params_mayo1 = MayoParams::mayo1();
        let res1 = keypair(&params_mayo1);
        assert!(res1.is_ok());
        let (csk1, cpk1) = res1.unwrap();
        assert_eq!(csk1.0.len(), params_mayo1.sk_seed_bytes());
        assert_eq!(cpk1.0.len(), params_mayo1.pk_seed_bytes() + params_mayo1.p3_bytes());

        // Test for MAYO2
        let params_mayo2 = MayoParams::mayo2();
        let res2 = keypair(&params_mayo2);
        assert!(res2.is_ok());
        let (csk2, cpk2) = res2.unwrap();
        assert_eq!(csk2.0.len(), params_mayo2.sk_seed_bytes());
        assert_eq!(cpk2.0.len(), params_mayo2.pk_seed_bytes() + params_mayo2.p3_bytes());
    }

    #[test]
    fn test_sign_api_flow_with_placeholder() {
        let params_enum = MayoParams::mayo1();
        let (csk, _cpk) = keypair(&params_enum).expect("keypair generation failed");
        let message = Message(b"test message for sign api".to_vec());

        let sign_result = sign(&csk, &message, &params_enum);
        
        // Since sign_message -> compute_Y_A_yprime_and_s_components is a placeholder returning Err,
        // we expect that specific error from sign_message.
        match sign_result {
            Err(e) => assert_eq!(e, "MAYO.Sign math core (compute_Y_A_yprime_and_s_components) not implemented".to_string()),
            Ok(_) => panic!("API sign should fail due to placeholder in sign_message"),
        }
    }

    #[test]
    fn test_open_api_flow_with_placeholder() {
        let params_enum = MayoParams::mayo1();
        let (_csk, cpk) = keypair(&params_enum).expect("keypair generation failed");
        
        // Create a dummy "signed message"
        // Signature part: s_bytes (n elements) + salt_bytes
        let s_bytes_len = MayoParams::bytes_for_gf16_elements(params_enum.variant().n); // Corrected call
        let expected_sig_len = s_bytes_len + params_enum.salt_bytes();
        
        let dummy_sig_bytes = vec![0u8; expected_sig_len];
        let original_message_text = b"test message for open api";
        
        let mut signed_message_bytes = Vec::new();
        signed_message_bytes.extend_from_slice(&dummy_sig_bytes);
        signed_message_bytes.extend_from_slice(original_message_text);

        let open_result = open(&cpk, &signed_message_bytes, &params_enum);

        // Since verify_signature -> compute_p_star_s is a placeholder returning Err,
        // we expect that specific error from verify_signature.
        match open_result {
            Err(e) => assert_eq!(e, "MAYO.Verify math core (compute_p_star_s) not implemented".to_string()),
            Ok(_) => panic!("API open should fail due to placeholder in verify_signature"),
        }
    }

    #[test]
    fn test_open_api_message_too_short() {
        let params_enum = MayoParams::mayo1();
        let (_csk, cpk) = keypair(&params_enum).expect("keypair generation failed");
        
        let s_bytes_len = MayoParams::bytes_for_gf16_elements(params_enum.variant().n); // Corrected call
        let expected_sig_len = s_bytes_len + params_enum.salt_bytes();
        
        let short_signed_message = vec![0u8; expected_sig_len - 1];
        
        let open_result = open(&cpk, &short_signed_message, &params_enum);
        assert_eq!(open_result, Err("Signed message is too short to contain a signature".to_string()));
    }
    
    // Conceptual test for open with tampered data (depends on functional sign & verify)
    // #[test]
    // fn test_open_tampered_flow_conceptual() {
    //     let params_enum = MayoParams::mayo1();
    //     let (csk, cpk) = keypair(&params_enum).unwrap();
    //     let message_text = b"original message";
    //     let original_message = Message(message_text.to_vec());

    //     // This part requires sign to be functional
    //     // let signature = sign(&csk, &original_message, &params_enum).expect("sign failed conceptually");
    //     // let mut signed_message_bytes = Vec::new();
    //     // signed_message_bytes.extend_from_slice(&signature.0);
    //     // signed_message_bytes.extend_from_slice(message_text);
        
    //     // // Tamper the signature part (e.g., flip a bit)
    //     // if !signed_message_bytes.is_empty() {
    //     //     signed_message_bytes[0] ^= 0x01; 
    //     // }
        
    //     // // This part requires verify_signature to be functional beyond placeholder
    //     // let open_result = open(&cpk, &signed_message_bytes, &params_enum);
    //     // match open_result {
    //     //     Ok(None) => { /* Correct for tampered signature */ },
    //     //     Ok(Some(_)) => panic!("Open succeeded with tampered signature"),
    //     //     Err(e) if e == "MAYO.Verify math core (compute_p_star_s) not implemented" => { /* Expected current state */ }
    //     //     Err(e) => panic!("Open failed unexpectedly: {}", e),
    //     // }
    // }

    // TODO: Implement Known Answer Tests (KATs) for the full keypair, sign, and open API lifecycle
    // once the core cryptographic math (compute_Y_A_yprime_and_s_components and compute_p_star_s)
    // is fully implemented. These tests will use official MAYO test vectors to verify
    // end-to-end correctness of the API functions.
}
