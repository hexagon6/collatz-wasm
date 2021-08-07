use wasm_bindgen::prelude::*;

fn _collatz(num: i32, list: &mut Vec<i32>) {
    let exists: bool = list.contains(&num);
    match exists {
        true => {}
        false => {
            list.push(num);
            let even: bool = num % 2 == 0;
            match even {
                true => _collatz(num / 2, list),
                false => _collatz(3 * num + 1, list),
            }
        }
    }
}

// i32 returns number
#[wasm_bindgen]
pub fn collatz(x: i32) -> Vec<i32> {
    let mut list: Vec<i32> = Vec::new();
    _collatz(x, &mut list);
    list
}
