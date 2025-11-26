use std::{process};
use html_parser::{Dom, Element, Node};
use anyhow::Result;
use dotenvy::dotenv;
use std::env;

fn is_node(node: &Node) -> bool {
    match node {
       Node::Element(..) => true,
       _ => false
    }
}

fn crawl_element(elem: Element) -> Result<Vec<String>> {

    let mut links: Vec<String> = Vec::new();

    // figure out whether we have a link on this node!
    if elem.name == "a" {
        let href_attrib = elem
            .attributes
            .iter()
            .filter(|(name, _)| name.as_str() == "href")
            .last()
            .ok_or_else(|| anyhow::anyhow!("No href found in a"));

        match href_attrib {
            Ok((_key, Some(val))) => {
                log::info!("Found link: {}", val);
                links.push(val.into());
            },
            _ => {
                log::error!("No link found for element {}", elem.name);
            }
        }
    }

    for node in elem
        .children
        .iter()
        .filter(|c| is_node(c)) {
            match node {
                Node::Element(elem) => {
                    // add whatever links from this elem to our vector
                    let mut children_links = crawl_element(elem.clone())?;
                    links.append(&mut children_links);
                },
                _ => {},
            }
        }

    Ok(links)
}

async fn crawl_url(url: &str) -> Result<Vec<String>> {
    // parsing html into a DOM obj
    let html = reqwest::get(url)
        .await?
        .text()
        .await?;

    let dom = Dom::parse(&html)?;

    // crawls all the nodes in the main html
    for child in dom.children {
        match child {
            Node::Element(elem) => {
                log::info!("Links found for elem {}: {:?}", elem.name.clone(), crawl_element(elem));
            },
            _ => {}
        }
    }

    // TODO: change this to the sum of all the links
    let res = Vec::new();
    Ok(res)
}


async fn try_main() -> Result<()> {
    dotenv().ok();

    let url = env::var("TARGET_URL").expect("TARGET_URL in .env isn't set.");

    let urls = crawl_url(&url).await?;   // Chaley I don't care about the result so_

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    match try_main().await {
        Ok(_) => {
            log::info!("Finished");
        }
        Err(e) => {
            log::info!("Error: {:?}", e);
            process::exit(-1);
        }
    }

    Ok(())
}

