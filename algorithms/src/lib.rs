use wasm_bindgen::prelude::*;
use rand::Rng;
use rand::distributions::{Distribution, Uniform};

#[wasm_bindgen]
pub fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let c = a + 0;
        a = b;
        b = c;
    }
    b
}

/*
    After many attempts and failures, 
    I've started to look elsewhere due to my insufficient knowledge of Rust.

    This implementation by Roy Adams worked splendidly with the last input:
    https://github.com/rgadams/wasm-primes/blob/main/src/lib.rs
*/

#[no_mangle]
#[wasm_bindgen(js_name = sieveOfEratosthenes)]
pub fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut primes: Vec<bool> = (0..n + 1).map(|num| num == 2 || num & 1 != 0).collect();
    let mut num = 3usize;
    while num * num <= n {        
        let mut j = num * num;
        while j <= n {
            primes[j] = false;
            j += num;
        }
        num += 2;
    }
    primes.into_iter().enumerate().skip(2).filter_map(|(i, p)| if p {Some(i)} else {None}).collect::<Vec<usize>>()
}

#[wasm_bindgen(js_name = monteCarloPi)]
pub fn monte_carlo_pi(times: i32) -> f64 {
    let between = Uniform::from(0.0..1.0);

    let mut inside = 0;
    let mut rng = rand::thread_rng();
    
    for _ in 0..times {
        let x = between.sample(&mut rng);
        let y = between.sample(&mut rng);
        
        if x * x + y * y <= 1.0 {
            inside += 1;
        }
    }
    
    4.0 * inside as f64 / times as f64
}

#[wasm_bindgen(js_name = djb2Hash)]
pub fn djb2_hash(s: &str) -> u32 {
    let mut hash: u32 = 5381;
    for c in s.chars() {
        hash = hash.wrapping_shl(5).overflowing_add(hash).0 + c as u32;
    }
    hash
}
