use num_bigint::BigUint;
use num_format::{Locale, ToFormattedString};
use num_traits::One;
use rayon::prelude::*;
use rug::Integer;
use std::ops::Shl;

fn is_prime(n: &BigUint) -> bool {
    let int_n = Integer::from_str_radix(&n.to_string(), 10).unwrap();
    int_n.is_probably_prime(30) != rug::integer::IsPrime::No // Probabilistic primality test
}

fn generate_perfect_numbers(limit: usize) -> Vec<BigUint> {
    let primes: Vec<u32> = (2..200)
        .filter(|&p| Integer::from(p).is_probably_prime(10) != rug::integer::IsPrime::No) // Faster prime check
        .collect();

    let mut perfect_numbers: Vec<BigUint> = primes
        .into_par_iter()
        .filter_map(|p| {
            let mersenne = BigUint::one().shl(p) - BigUint::one();
            if is_prime(&mersenne) {
                Some((BigUint::one().shl(p - 1)) * &mersenne)
            } else {
                None
            }
        })
        .collect();

    perfect_numbers.truncate(limit);
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
