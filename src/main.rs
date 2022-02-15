extern crate spider;

use spider::website::Website;
use scraper::{ Html, Selector };
use serde::Serialize;
use serde_json::Value;
use std::io::Write;
use std::io::Result;
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
  // The domain name to search
  #[clap(short, long)]
  url: String,
    
  #[clap(short, long, default_value = "products.ndjson")]
  output: std::path::PathBuf,
}

fn main() {
  let args = CliArgs::parse();
  let mut website: Website = Website::new(&args.url);
  let file_writer = std::fs::File::create(&args.output).expect("Could Not Create File");
  website.crawl();
  for page in website.get_pages() {
    let html = page.get_html();
    let products: Vec<Value> = find_products(&html);

    for product in products {
      match serialize_to(&file_writer, &product) {
        Err(e) => println!("Error: {}", e),
        Ok(_) => (),
      }
    };
  }
}


// Naive linked data scanner, only supports jsonld not microdata or rdfa
fn find_products(html: &Html) -> Vec<Value> {
  let jsonld_selector = Selector::parse(r#"script[type="application/ld+json"]"#).unwrap();
  let jsonld_nodes = html.select(&jsonld_selector).into_iter();
  jsonld_nodes
    .map(|node| node.inner_html())
    .map(|inner_html| serde_json::from_str(&inner_html) as serde_json::Result<Value>)
    .filter_map(|jsonld| {
        match jsonld {
          Ok(x) => if x["@type"] == "Product" { Some(x) } else { None },
          Err(e) => { println!("Error: {}", e); None }
        }
    })
    .collect::<Vec<Value>>()
}

fn serialize_to<W: Write, T: ?Sized + Serialize>(mut writer: W, value: &T) -> Result<()> {
  serde_json::to_writer(&mut writer, value)?;
  writer.write_all("\n".as_bytes())
}