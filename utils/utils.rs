// Utility functions
pub fn hash(data: Vec<u8>) -> Vec<u8> {
    // Hash data using SHA-256
}

pub fn sign(data: Vec<u8>, private_key: Vec<u8>) -> Vec<u8> {
    // Sign data using ECDSA
}

pub fn verify_signature(data: Vec<u8>, signature: Vec<u8>, public_key: Vec<u8>) -> bool {
    // Verify signature using ECDSA
}
