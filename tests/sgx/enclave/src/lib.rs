#![no_std]

extern crate sgx_tstd as std;

#[no_mangle]
pub extern "C" fn ecall_do_rsgx_test() {
    wheel::tests::do_rsgx_test();
}
