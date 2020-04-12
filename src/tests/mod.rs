use std::prelude::v1::*;

fn panic_with_str(msg: &'static str) {
    panic!(msg);
}

fn panic_with_string(msg: String) {
    panic!(&msg);
}

//#[test]
fn basic() {
    super::should_panic!(panic!("aha"));
}

////#[test]
//#[should_panic]
//fn without_panic() {
//    super::should_panic!(|| {});
//}

//#[test]
fn with_expected() {
    super::should_panic!(panic_with_str("aha"), "aha");
    super::should_panic!(panic_with_str("aha!"), "aha");
    super::should_panic!(panic_with_string("aha".to_string()), "aha");
    super::should_panic!(panic_with_string("aha!".to_string()), "aha");

    let panic_with_variadic = || {
        panic!("aha {}", 123);
    };
    super::should_panic!(panic_with_variadic(), "aha");
}

pub fn do_rsgx_test() {
    basic();
    with_expected();
}
