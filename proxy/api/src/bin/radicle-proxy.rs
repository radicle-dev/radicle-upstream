#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    api::env::set_if_unset("RUST_BACKTRACE", "full");
    api::env::set_if_unset("RUST_LOG", "info,quinn=warn");
    pretty_env_logger::init();

    api::run(argh::from_env()).await
}
