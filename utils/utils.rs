use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

pub trait Hashable {
    fn hash(&self) -> u64;
}

impl<T> Hashable for T
where
    T: Hash,
{
    fn hash(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish() as u64
    }
}

pub fn hex_encode(data: &[u8]) -> String {
    hex::encode(data)
}

pub fn hex_decode(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex)
}

pub fn base58_encode(data: &[u8]) -> String {
    bs58::encode(data).into_string()
}

pub fn base58_decode(base58: &str) -> Result<Vec<u8>, bs58::decode::Error> {
    bs58::decode(base58).into_vec()
}

pub fn sha256(data: &[u8]) -> Vec<u8> {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

pub fn ripemd160(data: &[u8]) -> Vec<u8> {
    use ripemd::{Digest, Ripemd160};
    let mut hasher = Ripemd160::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

pub fn to_hex<T>(value: &T) -> String
where
    T: AsRef<[u8]>,
{
    hex::encode(value.as_ref())
}

pub fn from_hex<T>(hex: &str) -> Result<T, hex::FromHexError>
where
    T: FromVec,
{
    let bytes = hex::decode(hex)?;
    T::from_vec(bytes)
}

pub trait FromVec {
    fn from_vec(vec: Vec<u8>) -> Self;
}

impl<T> FromVec for T
where
    T: Default + serde::de::DeserializeOwned,
{
    fn from_vec(vec: Vec<u8>) -> Self {
        serde_json::from_slice(&vec).unwrap_or_default()
    }
}

pub fn set_difference<T>(set1: &HashSet<T>, set2: &HashSet<T>) -> HashSet<T>
where
    T: Eq + Hash + Clone,
{
    set1.iter().cloned().filter(|x| !set2.contains(x)).collect()
}

pub fn set_intersection<T>(set1: &HashSet<T>, set2: &HashSet<T>) -> HashSet<T>
where
    T: Eq + Hash + Clone,
{
    set1.iter().cloned().filter(|x| set2.contains(x)).collect()
}

pub fn set_union<T>(set1: &HashSet<T>, set2: &HashSet<T>) -> HashSet<T>
where
    T: Eq + Hash + Clone,
{
    set1.iter().cloned().chain(set2.iter().cloned()).collect()
}
