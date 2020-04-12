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
