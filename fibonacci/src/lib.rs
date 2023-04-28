use wasm_bindgen::prelude::*;
use js_sys::Array;
use bit_vec::BitVec;

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
    This implementation worked splendidly with the last input, which is a copy of this:
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