//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
// fn main() {
//     // In tests7, set up an environment variable called `TEST_FOO`.
//     let timestamp = std::time::SystemTime::now()
//         .duration_since(std::time::UNIX_EPOCH)
//         .unwrap()
//         .as_secs();
   
//     // Use std::env::set_var to set the environment variable.
//     std::env::set_var("TEST_FOO", format!("Your command here with {}, please checkout exercises/tests/build.rs", timestamp).to_string());
    
//     // In tests8, enable "pass" feature to make the testcase return early.
//     std::env::set_var("CARGO_FEATURE_PASS", "1");
// }
// fn main() {
//     // Set the "UNIX_EPOCH" environment variable to a value representing the current time.
//     // You can use the `SystemTime::now()` function to get the current time and then convert it to seconds.
//     // For example, set it to the current UNIX timestamp:
//     let timestamp = std::time::SystemTime::now()
//         .duration_since(std::time::UNIX_EPOCH)
//         .unwrap()
//         .as_secs();


//     println!("cargo:rustc-link-lib=foo");



//     // Set the "UNIX_EPOCH" environment variable with the calculated timestamp value.
//     std::env::set_var("UNIX_EPOCH", timestamp.to_string());

//     // Optionally, you can print the value for debugging purposes.
//     println!("cargo:rustc-env=UNIX_EPOCH={}", timestamp);
//  // Set the "CARGO_FEATURE_PASS" environment variable to enable the "pass" feature in tests8.rs.
//  std::env::set_var("CARGO_FEATURE_PASS", "1");

//  // Optionally, you can print the value for debugging purposes.
//  println!("cargo:rustc-env=CARGO_FEATURE_PASS=1");
//     // You can also check if the "CARGO_FEATURE_PASS" environment variable is set
//     // and enable the "pass" feature if needed.
//     if let Ok(_) = std::env::var("CARGO_FEATURE_PASS") {
//         println!("cargo:rustc-cfg=feature=\"pass\"");
//     }
// }
fn main() {
    // Set the "UNIX_EPOCH" environment variable to a value representing the current time.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Set the "UNIX_EPOCH" environment variable with the calculated timestamp value.
    std::env::set_var("UNIX_EPOCH", timestamp.to_string());

    // Print the value for debugging purposes.
    println!("cargo:rustc-env=UNIX_EPOCH={}", timestamp);

    // Set the "CARGO_FEATURE_PASS" environment variable to enable the "pass" feature in tests8.rs.
    std::env::set_var("CARGO_FEATURE_PASS", "1");

    // Print the value for debugging purposes.
    println!("cargo:rustc-env=CARGO_FEATURE_PASS=1");
}