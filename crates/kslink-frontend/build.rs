use chrono::Utc;

const ENV: &str = "KSLINK_BASE_URL";

fn main() {
    println!("cargo::rerun-if-env-changed=BUILD_TIME");

    match std::env::var(ENV) {
        Ok(env) => println!("cargo::rustc-env={ENV}={env}"),
        Err(_) => println!("cargo::rustc-env={ENV}=http://127.0.0.1:8000"),
    }

    println!("cargo::rustc-env=BUILD_TIME={}", Utc::now());
}
