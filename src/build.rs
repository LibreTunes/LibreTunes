// Set the target config variable to the target triple of the current build
// This is used so we can determine the target triple at compile time
// See https://stackoverflow.com/a/51311222
fn main() {
    println!(
        "cargo:rustc-cfg=target=\"{}\"",
        std::env::var("TARGET").unwrap()
    );
}
