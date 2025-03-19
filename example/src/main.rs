#![no_main]

risc0_zkvm::guest::entry!(main);

fn fibonacci(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let sum = (a + b) % 7919; // Mod to avoid overflow
        a = b;
        b = sum;
    }
    b
}

pub fn main() {
    let n: u32 = risc0_zkvm::guest::env::read();
    let result = fibonacci(n);
    println!("result: {}", result);
    risc0_zkvm::guest::env::commit(&result);
}