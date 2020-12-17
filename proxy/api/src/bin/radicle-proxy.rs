#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    api::env::set_if_unset("RUST_BACKTRACE", "full");
    api::env::set_if_unset("RUST_LOG", "info,quinn=warn");
    pretty_env_logger::init();

    let mut args = pico_args::Arguments::from_env();
    let args = api::Args {
        test: args.contains("--test"),
        /// HTTP API binds to 127.0.0.1:17246 by default.
        http_listen: args
            .opt_value_from_str("--http-listen")?
            .unwrap_or("127.0.0.1:17246".parse()?),
        /// Radicle peer binds to all local interfaces and any available port.
        peer_listen: args
            .opt_value_from_str("--peer-listen")?
            .unwrap_or("0.0.0.0:0".parse()?),
    };

    api::run(args).await
}
