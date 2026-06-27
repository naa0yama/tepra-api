//! tepra-api binary entry point.

#[allow(clippy::print_stdout)]
fn main() {
    if std::env::args().any(|a| a == "--help" || a == "-h") {
        println!("tepra-api {}", tepra_web::app_version());
    }
}
