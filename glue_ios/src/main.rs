use glue_core::{gcd, record_custom_event};

fn main() {
    gcd(24, 6);
    record_custom_event("idkman");
    println!("gcd(24, 6) = {}", gcd(24, 6));
}
