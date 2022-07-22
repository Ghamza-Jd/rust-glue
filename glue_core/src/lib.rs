use std::collections::HashMap;
use std::thread;

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

pub fn record_custom_event(event_type: &str) {
    println!("event: {}", format!("{}", event_type));
    let client = reqwest::blocking::Client::new();
    let mut map = HashMap::new();
    map.insert("event", format!("{}", event_type));
    let _res = client
        .post("http://localhost:3000/events")
        .json(&map)
        .send();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(14, 15), 1);
        assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
    }
}
