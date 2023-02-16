use wasm_bindgen::prelude::*;
use js_sys::Array;

#[wasm_bindgen]
pub fn bubble_sort(data: Vec<i32>)-> Array {
    let mut copy = data.to_vec();   
    let mut temp: i32;
    for i in 0.. copy.len() {
        for j in 0.. copy.len(){
            if copy[j] > copy[i] {
                temp = copy[j];
                copy[j] = copy[i];
                copy[i] = temp;
            }
        }
    }
    let js_array : Array = Array::new();
    for i in 0..copy.len(){
        js_array.push(&wasm_bindgen::JsValue::from(copy[i]));
    }

    return js_array;
}

