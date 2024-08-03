use num::{BigInt, BigUint, ToPrimitive};
use num_traits::{One, Zero};

pub trait ModularArithmetic {
    fn mod_add(&self, other: &Self, modulus: &BigInt) -> BigInt;
    fn mod_sub(&self, other: &Self, modulus: &BigInt) -> BigInt;
    fn mod_mul(&self, other: &Self, modulus: &BigInt) -> BigInt;
    fn mod_pow(&self, exponent: &BigInt, modulus: &BigInt) -> BigInt;
    fn mod_inv(&self, modulus: &BigInt) -> Option<BigInt>;
}

impl ModularArithmetic for BigInt {
    fn mod_add(&self, other: &BigInt, modulus: &BigInt) -> BigInt {
        (self + other) % modulus
    }

    fn mod_sub(&self, other: &BigInt, modulus: &BigInt) -> BigInt {
        (self - other) % modulus
    }

    fn mod_mul(&self, other: &BigInt, modulus: &BigInt) -> BigInt {
        (self * other) % modulus
    }

    fn mod_pow(&self, exponent: &BigInt, modulus: &BigInt) -> BigInt {
        self.mod_pow(exponent, modulus)
    }

    fn mod_inv(&self, modulus: &BigInt) -> Option<BigInt> {
        self.mod_inverse(modulus)
    }
}

pub fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    if b.is_zero() {
        a.clone()
    } else {
        gcd(b, &(a % b))
    }
}

pub fn lcm(a: &BigInt, b: &BigInt) -> BigInt {
    (a * b) / gcd(a, b)
}

pub fn is_prime(n: &BigInt) -> bool {
    if n <= &BigInt::from(1) {
        return false;
    }
    if n <= &BigInt::from(3) {
        return true;
    }
    if n % &BigInt::from(2) == BigInt::from(0) {
        return false;
    }
    let mut i = BigInt::from(3);
    while i * i <= n {
        if n % &i == BigInt::from(0) {
            return false;
        }
        i += BigInt::from(2);
    }
    true
}

pub fn next_prime(n: &BigInt) -> BigInt {
    let mut n = n.clone();
    n += BigInt::from(1);
    while !is_prime(&n) {
        n += BigInt::from(1);
    }
    n
}

pub fn random_prime<R>(rng: &mut R, bits: usize) -> BigInt
where
    R: rand_core::RngCore,
{
    let mut n = BigInt::from(1) << bits;
    while !is_prime(&n) {
        n = BigInt::from(rng.next_u64()) << (bits - 64);
        n += BigInt::from(1);
    }
    n
               }
