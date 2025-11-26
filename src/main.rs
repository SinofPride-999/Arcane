use std::process;
use html_parser::{Dom, Node};
use anyhow::Result;
use dotenvy::dotenv;
use std::env;

async fn crawl_url(url: &str) -> Result<Vec<String>> {
    let html = reqwest::get(url)
        .await?
        .text()
        .await?;

    let dom = Dom::parse(&html)?;

    for child in dom.children {
        match child {
            Node::Text(text) => {
                log::info!("Node found: {}", text);
            },
            Node::Element(elem)=> {
                log::info!("Element found: {}", elem.name);
            },
            Node::Comment(comment) => {
                log::info!("Comment found: {}", comment);
            }
        }
    }

    let res = Vec::new();
    Ok(res)
}


async fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let url = env::var("TARGET_URL").expect("TARGET_URL in .env isn't set.");

    let _ = crawl_url(&url).await?;   // Chaley I don't care about the result so _
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    match try_main().await {
        Ok(_) => {
            log::info!("Finished")
        }
        Err(e) => {
            log::info!("Error: {:?}", e);
            process::exit(-1);
        }
    }

    Ok(())
}

