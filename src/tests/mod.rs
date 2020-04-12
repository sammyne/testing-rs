mod should_panic;
mod tests;

pub fn do_rsgx_tests() -> usize {
    should_panic::do_rsgx_tests();

    tests::do_rsgx_tests()
}
