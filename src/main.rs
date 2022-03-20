mod client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("API_KEY").expect("API_KEY is not defined");
    let builder = client::ClientBuilder::new(api_key);
    let client = builder.build().expect("successful build client");

    eprintln!("{}", client.get_devises().await?);
    Ok(())
}
