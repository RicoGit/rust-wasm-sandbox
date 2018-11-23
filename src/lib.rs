extern crate rand;

extern {
    #[no_mangle]
    fn rust_wasm_syscall(index: usize, data: *mut u8) -> usize;
}

//
// Simple function (shouldn't fail sanitizer)
//

#[no_mangle]
pub unsafe fn t_simple() -> i32 {
    rust_wasm_syscall(1, 2 as *mut u8);
    123
}

//
// Sys calls (should fail sanitizer)
//

/// Macros `println!` with writing ti 'stdout'
#[no_mangle]
fn t_println() {
    println!("{:?}", "hello world");
}

/// Writing bytes to `stdout`
#[no_mangle]
fn t_stdout() {
    use std::io::{self, Write};
    io::stdout().write(b"hello world").unwrap();
}

/// Creating thread
#[no_mangle]
fn t_thread() {
    use std::thread;
    thread::spawn(|| { "hello world" }).join().unwrap();
}


/// Getting system time
#[no_mangle]
fn t_system_clock() {
    std::time::SystemTime::now();
}

// todo add all system calls

//
// Non-deterministic operations (should fail sanitizer)
//

/// Using floating point arithmetic
#[no_mangle]
fn t_float() -> f64 {
    let a = 1.5;
    let b = 0.999999;
    a + b
}

/// Using random generators
#[no_mangle]
fn t_random() -> usize {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen()
}

// todo add more
