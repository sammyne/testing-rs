use std::prelude::v1::*;

use crate::*;

fn ok() {}

fn oks() -> usize {
    crate::run_tests!(ok, ok)
}

fn oks_and_failures() -> usize {
    crate::run_tests!(ok, fail, ok)
}

fn fail() {
    panic!("failed");
}

fn failures() -> usize {
    crate::run_tests!(fail, fail)
}

pub fn do_rsgx_tests() -> usize {
    oks() + oks_and_failures() + failures()
}
