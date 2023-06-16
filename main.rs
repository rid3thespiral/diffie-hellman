use num_bigint::BigUint;
use num_traits::{One, Zero};
use rand::Rng;
use num_integer::Integer;

fn main() {
    // Key generation
    let prime = generate_prime(512);
    let generator = BigUint::from(2u32); 

    let alice_private_key = generate_private_key(&prime);
    let alice_public_key = compute_public_key(&generator, &prime, &alice_private_key);

    let bob_private_key = generate_private_key(&prime);
    let bob_public_key = compute_public_key(&generator, &prime, &bob_private_key);

    // Secret key computation
    let alice_shared_secret = compute_shared_secret(&bob_public_key, &prime, &alice_private_key);
    let bob_shared_secret = compute_shared_secret(&alice_public_key, &prime, &bob_private_key);

    // Verify keys
    assert_eq!(alice_shared_secret, bob_shared_secret);

    println!("Shared Secret: {:?}", alice_shared_secret);
}

// Random Prime number generation
fn generate_prime(bit_size: usize) -> BigUint {
    let mut rng = rand::thread_rng();
    loop {
        let prime_candidate = generate_random(bit_size);
        if is_prime(&prime_candidate) {
            return prime_candidate;
        }
    }
}

// Random number generation dimension bit_size bit
fn generate_random(bit_size: usize) -> BigUint {
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..(bit_size + 7) / 8)
        .map(|_| rng.gen())
        .collect();
    BigUint::from_bytes_be(&random_bytes)
}

// Verify prime number with Miller-Rabin test
fn is_prime(n: &BigUint) -> bool {
    if n == &BigUint::zero() || n == &BigUint::one() {
        return false;
    }
    let mut rng = rand::thread_rng();
    let num_trials = 10;
    for _ in 0..num_trials {
        let a = BigUint::from(2u32 + rng.gen_range(0..100));
        if !miller_rabin(n, &a) {
            return false;
        }
    }
    true
}

// Miller-Rabin test implementation prime number
fn miller_rabin(n: &BigUint, a: &BigUint) -> bool {
    let one = BigUint::one();
    let d = n - &one;
    let mut s = 0;
    let mut d = d.clone();
    while d.is_even() {
        d >>= 1;
        s += 1;
    }

    let mut x = a.modpow(&d, n);
    if x == one || x == n - &one {
        return true;
    }

    for _ in 0..s - 1 {
        x = x.modpow(&2u32.into(), n);
        if x == one {
            return false;
        }
        if x == n - &one {
            return true;
        }
    }

    false
}

// Private key random generation
fn generate_private_key(prime: &BigUint) -> BigUint {
    let mut rng = rand::thread_rng();
    loop {
          let private_key = generate_random(prime.bits().try_into().unwrap());
          if private_key > BigUint::zero() && private_key < *prime {
            return private_key;
        }
    }
}

// Public key computation from private key
fn compute_public_key(generator: &BigUint, prime: &BigUint, private_key: &BigUint) -> BigUint {
    generator.modpow(private_key, prime)
}

// Secret shared key generation from public and private key
fn compute_shared_secret(public_key: &BigUint, prime: &BigUint, private_key: &BigUint) -> BigUint {
    public_key.modpow(private_key, prime)
}
