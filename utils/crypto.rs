use std::ops::{Deref, DerefMut};

use elliptic_curve::{Curve, Secp256k1};
use elliptic_curve::{KeyPair, PublicKey, SecretKey};
use elliptic_curve::{Signature, Verifier};
use hex::{FromHex, ToHex};
use rand_core::{CryptoRng, RngCore};

pub trait KeyPairTrait {
    fn generate<R>(rng: &mut R) -> Self
    where
        R: CryptoRng + RngCore;
    fn public_key(&self) -> &PublicKey;
    fn secret_key(&self) -> &SecretKey;
    fn sign(&self, message: &[u8]) -> Signature;
    fn verify(&self, message: &[u8], signature: &Signature) -> bool;
}

pub struct KeyPair {
    key_pair: KeyPair<Secp256k1>,
}

impl KeyPairTrait for KeyPair {
    fn generate<R>(rng: &mut R) -> Self
    where
        R: CryptoRng + RngCore,
    {
        let key_pair = KeyPair::generate(rng);
        KeyPair { key_pair }
    }

    fn public_key(&self) -> &PublicKey {
        &self.key_pair.public_key
    }

    fn secret_key(&self) -> &SecretKey {
        &self.key_pair.secret_key
    }

    fn sign(&self, message: &[u8]) -> Signature {
        self.key_pair.sign(message)
    }

    fn verify(&self, message: &[u8], signature: &Signature) -> bool {
        self.key_pair.verify(message, signature)
    }
}

impl Deref for KeyPair {
    type Target = KeyPair<Secp256k1>;

    fn deref(&self) -> &Self::Target {
        &self.key_pair
    }
}

impl DerefMut for KeyPair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.key_pair
    }
}

impl FromHex for KeyPair {
    fn from_hex<T>(hex: T) -> Result<Self, hex::FromHexError>
    where
        T: AsRef<[u8]>,
    {
        let bytes = hex::decode(hex)?;
        let key_pair = KeyPair::from_bytes(&bytes)?;
        Ok(KeyPair { key_pair })
    }
}

impl ToHex for KeyPair {
    fn encode_hex<T>(&self) -> T
    where
        T: FromIterator<char>,
    {
        let bytes = self.to_bytes();
        hex::encode(bytes).into_iter().collect()
    }
}

impl KeyPair {
    fn from_bytes(bytes: &[u8]) -> Result<KeyPair<Secp256k1>, elliptic_curve::Error> {
        KeyPair::from_bytes(bytes)
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.key_pair.to_bytes()
    }
        }
