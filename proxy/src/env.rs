/// Convenience function for setting env vars only if they are unset.
pub fn set_if_unset (key: &str, val: &str) -> Result<String, std::env::VarError> {
    use std::env::{var,set_var};
    var(key).or({ set_var(key, val); var(key) })
}

#[test]
fn test_set_if_unset() {
    std::env::var("DUMMY_VALUE").unwrap_err(); // should fail because the value should be unset
    let val = set_if_unset("DUMMY_VALUE", "hello world").expect("Value should be set.");
    assert_eq!(val, "hello world");
}
