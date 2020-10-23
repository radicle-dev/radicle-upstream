#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    api::main().await
}
