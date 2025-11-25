use anyhow::Result;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let res = reqwest::get("https://youtube.com")
        .await?
        .text()
        .await?;

    println!("{}", res);

    log::info!("Hello World!");
    Ok(())
}
