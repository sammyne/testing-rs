#![no_std]

#[macro_use]
extern crate sgx_tstd as std;

use std::string::String;
use std::vec::Vec;

#[macro_export]
macro_rules! run_tests {
    (
        $($f : ident),*
    ) => {{
        test_start();
        let mut ntestcases : u64 = 0u64;
        let mut failurecases : Vec<String> = Vec::new();
        $(run_test(&mut ntestcases, &mut failurecases, &$f,stringify!($f));)*

        let failures = failurecases.len();
        test_end(ntestcases, failurecases);

        failures
    }}
}

#[macro_export]
macro_rules! should_panic {
    ($fmt:expr) => {{
        if std::panic::catch_unwind(|| $fmt).is_ok() {
            let (file, line, col): (&'static str, u32, u32) = (file!(), line!(), column!());
            panic!("missing panic at {}:{}:{}", file, line, col);
        }
    }};
    ($fmt:expr, $expected:literal) => {{
        match std::panic::catch_unwind(|| $fmt) {
            Ok(_) => {
                let (file, line, col): (&'static str, u32, u32) = (file!(), line!(), column!());
                panic!("missing panic at {}:{}:{}", file, line, col);
            }
            Err(err) => {
                let expected: &str = $expected;

                let done = match err.downcast_ref::<&str>() {
                    Some(got) if got.contains(expected) => true,
                    Some(got) => panic!("should_panic fail: expect '{}', got '{}'", expected, got),
                    None => false,
                };

                // panic!("xxx {}", arg) would match this downcast
                let done = done
                    || match err.downcast_ref::<String>() {
                        Some(got) if got.contains(expected) => true,
                        Some(got) => {
                            panic!("should_panic fail: expect '{}', got '{}'", expected, got)
                        }
                        None => false,
                    };

                let _ = done || panic!("unsupported panic info type");
            }
        }
    }};
}

/// Perform one test case at a time.
///
/// This is the core function of testing. It runs one test case at a
/// time and saves the result. On test passes, it increases the passed counter
/// and on test fails, it records the failed test.
/// Required test function must be `Fn()`, taking nothing as input and returns
/// nothing.
pub fn run_test<F, R>(ncases: &mut u64, failurecases: &mut Vec<String>, f: F, name: &str)
where
    F: FnOnce() -> R + std::panic::UnwindSafe,
{
    *ncases = *ncases + 1;
    match std::panic::catch_unwind(|| {
        f();
    })
    .is_ok()
    {
        true => {
            println!("{} {} ... {}!", "testing", name, "\x1B[1;32mok\x1B[0m");
        }
        false => {
            println!("{} {} ... {}!", "testing", name, "\x1B[1;31mfailed\x1B[0m");
            failurecases.push(String::from(name));
        }
    }
}

pub fn test_start() {
    println!("\nstart running tests");
}

/// An epilogue function for Rust SGX unit testing.
///
/// `test_end` prints the statistics on test result, including
/// a list of failed tests and the statistics.
pub fn test_end(ntestcases: u64, failurecases: Vec<String>) {
    let ntotal = ntestcases as usize;
    let nsucc = ntestcases as usize - failurecases.len();

    if failurecases.len() != 0 {
        let vfailures = failurecases;
        print!("\nfailures: ");
        println!(
            "    {}",
            vfailures
                .iter()
                .fold(String::new(), |s, per| s + "\n    " + per)
        );
    }

    if ntotal == nsucc {
        print!("\ntest result \x1B[1;32mok\x1B[0m. ");
    } else {
        print!("\ntest result \x1B[1;31mFAILED\x1B[0m. ");
    }

    println!(
        "{} tested, {} passed, {} failed\n",
        ntotal,
        nsucc,
        ntotal - nsucc
    );
}

#[cfg(feature = "testing")]
pub mod tests;
