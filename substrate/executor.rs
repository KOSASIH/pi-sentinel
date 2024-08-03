use std::{
    sync::{Arc, Mutex},
    thread,
};

use crypto::{KeyPair, PublicKey, SecretKey};
use math::{gcd, is_prime, lcm, next_prime, random_prime};
use storage::{Database, Storage};

pub struct Executor {
    db: Arc<Mutex<Database>>,
    key_pair: KeyPair,
}

impl Executor {
    pub fn new(db: Arc<Mutex<Database>>, key_pair: KeyPair) -> Self {
        Executor { db, key_pair }
    }

    pub fn execute(&self, task: Task) -> Result<(), String> {
        match task {
            Task::GenerateKeyPair => {
                let mut rng = OsRng;
                let key_pair = KeyPair::generate(&mut rng);
                self.db.lock().unwrap().insert_key_pair(key_pair);
                Ok(())
            }
            Task::VerifySignature(message, signature) => {
                let public_key = self.key_pair.public_key();
                if public_key.verify(message, &signature) {
                    Ok(())
                } else {
                    Err("Invalid signature".to_string())
                }
            }
            Task::ComputeGcd(a, b) => {
                let result = gcd(&a, &b);
                self.db.lock().unwrap().insert_gcd(result);
                Ok(())
            }
            Task::ComputeLcm(a, b) => {
                let result = lcm(&a, &b);
                self.db.lock().unwrap().insert_lcm(result);
                Ok(())
            }
            Task::IsPrime(n) => {
                let result = is_prime(&n);
                self.db.lock().unwrap().insert_prime(result);
                Ok(())
            }
            Task::NextPrime(n) => {
                let result = next_prime(&n);
                self.db.lock().unwrap().insert_next_prime(result);
                Ok(())
            }
            Task::RandomPrime(bits) => {
                let mut rng = OsRng;
                let result = random_prime(&mut rng, bits);
                self.db.lock().unwrap().insert_random_prime(result);
                Ok(())
            }
        }
    }

    pub fn run(&self) {
        loop {
            let task = self.db.lock().unwrap().pop_task();
            if let Some(task) = task {
                match self.execute(task) {
                    Ok(_) => {}
                    Err(err) => eprintln!("Error executing task: {}", err),
                }
            } else {
                thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
}

pub enum Task {
    GenerateKeyPair,
    VerifySignature(Vec<u8>, Vec<u8>),
    ComputeGcd(BigInt, BigInt),
    ComputeLcm(BigInt, BigInt),
    IsPrime(BigInt),
    NextPrime(BigInt),
    RandomPrime(usize),
            }
