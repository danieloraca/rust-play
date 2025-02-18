use num_bigint::BigUint;
use num_format::{Locale, ToFormattedString};
use num_traits::{One, Zero};
use std::ops::Shl;

fn is_prime(n: &BigUint) -> bool {
    if *n < BigUint::from(2u32) {
        return false;
    }
    if *n == BigUint::from(2u32) {
        return true;
    }
    if n % BigUint::from(2u32) == BigUint::zero() {
        return false;
    }

    let sqrt_n = n.sqrt();
    let mut i = BigUint::from(3u32);

    while &i <= &sqrt_n {
        if n % &i == BigUint::zero() {
            return false;
        }
        i += BigUint::from(2u32); // Skip even numbers
    }
    true
}

fn generate_perfect_numbers(limit: usize) -> Vec<BigUint> {
    let mut perfect_numbers = Vec::new();
    let mut p = 2;

    while perfect_numbers.len() < limit {
        let mersenne = BigUint::one().shl(p) - BigUint::one(); // 2^p - 1
        if is_prime(&mersenne) {
            let perfect_number = (BigUint::one().shl(p - 1)) * &mersenne; // 2^(p-1) * (2^p - 1)
            perfect_numbers.push(perfect_number);
        }
        p += 1;
    }

    perfect_numbers
}

fn main() {
    let limit = 10;
    let perfect_numbers = generate_perfect_numbers(limit);

    for num in perfect_numbers {
        let num_str = num.to_string(); // Convert to String
        let formatted = num_str
            .parse::<u128>()
            .unwrap_or(0)
            .to_formatted_string(&Locale::en);
        println!("Perfect number: {}", formatted);
    }
}
