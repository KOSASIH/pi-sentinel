use criterion::{BenchmarkGroup, Criterion};
use rand_core::OsRng;

use crypto::{KeyPair, PublicKey, SecretKey};
use math::{gcd, is_prime, lcm, next_prime, random_prime};

pub fn benchmark_key_pair_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("key_pair_generation");
    let mut rng = OsRng;

    group.bench_function("secp256k1", |b| {
        b.iter(|| KeyPair::generate(&mut rng));
    });

    group.finish();
}

pub fn benchmark_signature_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("signature_verification");
    let mut rng = OsRng;
    let key_pair = KeyPair::generate(&mut rng);
    let message = b"Hello, world!";
    let signature = key_pair.sign(message);

    group.bench_function("secp256k1", |b| {
        b.iter(|| key_pair.verify(message, &signature));
    });

    group.finish();
}

pub fn benchmark_gcd(c: &mut Criterion) {
    let mut group = c.benchmark_group("gcd");
    let a = BigInt::from(123456789);
    let b = BigInt::from(987654321);

    group.bench_function("gcd", |b| {
        b.iter(|| gcd(&a, &b));
    });

    group.finish();
}

pub fn benchmark_lcm(c: &mut Criterion) {
    let mut group = c.benchmark_group("lcm");
    let a = BigInt::from(123456789);
    let b = BigInt::from(987654321);

    group.bench_function("lcm", |b| {
        b.iter(|| lcm(&a, &b));
    });

    group.finish();
}

pub fn benchmark_is_prime(c: &mut Criterion) {
    let mut group = c.benchmark_group("is_prime");
    let n = BigInt::from(123456789);

    group.bench_function("is_prime", |b| {
        b.iter(|| is_prime(&n));
    });

    group.finish();
}

pub fn benchmark_next_prime(c: &mut Criterion) {
    let mut group = c.benchmark_group("next_prime");
    let n = BigInt::from(123456789);

    group.bench_function("next_prime", |b| {
        b.iter(|| next_prime(&n));
    });

    group.finish();
}

pub fn benchmark_random_prime(c: &mut Criterion) {
    let mut group = c.benchmark_group("random_prime");
    let mut rng = OsRng;
    let bits = 256;

    group.bench_function("random_prime", |b| {
        b.iter(|| random_prime(&mut rng, bits));
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_key_pair_generation,
    benchmark_signature_verification,
    benchmark_gcd,
    benchmark_lcm,
    benchmark_is_prime,
    benchmark_next_prime,
    benchmark_random_prime,
);

criterion_main!(benches);
