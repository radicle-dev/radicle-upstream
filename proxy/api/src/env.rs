//! Utilities to manipulate the process environment.

use std::env::{set_var, var};

/// Convenience function for setting env vars only if they are unset.
pub fn set_if_unset(key: &str, val: &str) {
    var(key).unwrap_or_else(|_| {
        set_var(key, val);
        var(key).expect("unable to get ENV var")
    });
}

#[test]
fn test_set_if_unset() {
    const KEY: &str = "DUMMY_VALUE";
    const VALUE: &str = "hello world";

    assert!(var(KEY).is_err(), "DUMMY_VALUE should be unset");
    set_if_unset(KEY, VALUE);
    assert_eq!(var(KEY), Ok(VALUE.to_string()));
}
