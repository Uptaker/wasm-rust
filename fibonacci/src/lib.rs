use wasm_bindgen::prelude::*;

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
        let c = a + b;
        a = b;
        b = c;
    }
    b
}
