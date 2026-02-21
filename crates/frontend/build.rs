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

    // Print environment configuration during build
    // This helps developers understand what URLs are baked into the WASM binary

    let api_url = option_env!("LOCUS_API_URL")
        .unwrap_or("https://api.locusmath.org/api (hardcoded fallback)");

    let frontend_url =
        option_env!("LOCUS_FRONTEND_URL").unwrap_or("https://locusmath.org (hardcoded fallback)");

    println!("cargo:warning=━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("cargo:warning=Frontend Build Configuration");
    println!("cargo:warning=━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("cargo:warning=API Base URL:      {}", api_url);
    println!("cargo:warning=Frontend Base URL: {}", frontend_url);
    println!("cargo:warning=━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Remind about environment variables if using defaults
    if option_env!("LOCUS_API_URL").is_none() {
        println!("cargo:warning=WARNING: LOCUS_API_URL not set - using production fallback");
        println!("cargo:warning=  Set LOCUS_API_URL=http://localhost:3000/api for dev");
    }

    if option_env!("LOCUS_FRONTEND_URL").is_none() {
        println!("cargo:warning=WARNING: LOCUS_FRONTEND_URL not set - using production fallback");
        println!("cargo:warning=  Set LOCUS_FRONTEND_URL=http://localhost:8080 for dev");
    }
}
