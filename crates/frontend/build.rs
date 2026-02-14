use std::process::Command;

fn main() {
    // Set build timestamp for dev builds
    let timestamp = Command::new("date")
        .arg("+%Y-%m-%d %H:%M:%S UTC")
        .env("TZ", "UTC")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);
}
