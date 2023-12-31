// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.


fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        if let Ok(_) = std::env::var("CARGO_FEATURE_PASS") {
            return; // Return early if the feature is enabled.
        }

        panic!("no cfg set");
    }
}
