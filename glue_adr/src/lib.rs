use glue_core::gcd;

struct AndroidGlue {}

impl AndroidGlue {
    pub fn android_gcd(n: u64, m: u64) -> u64 {
        gcd(n, m)
    }
}
