
//extern crate rand;

#[no_mangle]
pub fn simple() -> i32 {
    123
}

/// Using floating point arithmetic
#[no_mangle]
fn float() -> f64 {
    let a = 1.5;
    let b = 0.999999;
    a + b
}

/// Macros `println!` with writing ti 'stdout'
#[no_mangle]
fn println() {
    println!("{:?}", "hello world");
}

/// Writing bytes to `stdout`
#[no_mangle]
fn stdout() {
    use std::io::{self, Write};
    io::stdout().write(b"hello world").unwrap();
}

/// Creating thread
#[no_mangle]
fn thread() {
    use std::thread;
    thread::spawn(|| { "hello world" }).join().unwrap();
}

/// Using random generators
#[no_mangle]
fn random() -> usize {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen()
}
