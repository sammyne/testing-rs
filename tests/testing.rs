fn panic_with_str(msg: &'static str) {
    panic!(msg);
}

fn panic_with_string(msg: String) {
    panic!(msg);
}

#[test]
fn basic() {
    testing::should_panic!(panic!("aha"));
}

#[test]
#[should_panic]
fn without_panic() {
    testing::should_panic!(|| {});
}

#[test]
fn with_expected() {
    testing::should_panic!(panic_with_str("aha"), "aha");
    testing::should_panic!(panic_with_str("aha!"), "aha");
    testing::should_panic!(panic_with_string("aha".to_string()), "aha");
    testing::should_panic!(panic_with_string("aha!".to_string()), "aha");

    let panic_with_variadic = || {
        panic!("aha {}", 123);
    };
    testing::should_panic!(panic_with_variadic(), "aha");
}
