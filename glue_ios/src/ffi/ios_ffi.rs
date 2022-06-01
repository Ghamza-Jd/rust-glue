use glue_core::gcd;

#[no_mangle]
pub extern "C" fn rust_gcd(n: u64, m: u64) -> u64 {
    return gcd(n, m);
}
