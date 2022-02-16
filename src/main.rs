extern crate spider;

use spider::website::Website;
use scraper::{ Html, Selector };
use serde::Serialize;
use serde_json::Value;
use std::io::Write;
use clap::Parser;
use url::Url;
use regex::Regex;

/// Search for json-ld entities on a website and collect them.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
  /// The domain name to search
  #[clap(short, long)]
  url: Url,
    
  /// Object @type to search for (e.g. Product)
  #[clap(short = 'T', long)]
  object_type: Option<String>,
    
  #[clap(short, long, default_value = "entities.ndjson")]
  output: std::path::PathBuf,
}

fn main() {
  let args = CliArgs::parse();
  let url = args.url.as_str().trim_end_matches('/');
  let mut website: Website = Website::new(url);
  website.configuration.verbose = true;
  let file_writer = std::fs::File::create(&args.output).expect("Could not create output file");
  website.crawl();
  for page in website.get_pages() {
    let html = page.get_html();
    let entities: Vec<Value> = find_linked_data_objects(&html, &args.object_type);

    for entity in entities {
      match serialize_to(&file_writer, &entity) {
        Err(e) => println!("Error writing to file: {}", e),
        Ok(_) => (),
      }
    };
  }
}


// Naive linked data scanner, only supports jsonld not microdata or rdfa
fn find_linked_data_objects(html: &Html, object_type: &Option<String>) -> Vec<Value> {
  let cdata_replace = Regex::new(r"(?ms)^<!\[CDATA\[(.*)\]\]>$").unwrap(); // TODO: surely scraper has something in it that does this bit for me?
  let jsonld_selector = Selector::parse(r#"script[type="application/ld+json"]"#).unwrap();
  let jsonld_nodes = html.select(&jsonld_selector).into_iter();
  jsonld_nodes
    .map(|node| node.text().collect::<String>())
    .map(|inner_html| {
      let clean_script = cdata_replace.replace_all(inner_html.trim(), "$1");
      serde_json::from_str(&clean_script)
    })
    .filter_map(|jsonld| matching_objects(jsonld, object_type))
    .collect::<Vec<Value>>()
}

fn matching_objects(jsonld: serde_json::Result<Value>, object_type: &Option<String>) -> Option<Value> {
  match jsonld {
    // we have an object
    Ok(x) => match object_type {
      // if we are matching on object type then check it and return None if it doesn't match
      Some(target_type) => if x["@type"].as_str() == Some(target_type.as_str()) {
        Some(x)
      } else {
        None
      },
      // if we are NOT matching on object type then return the object
      None => Some(x)
    },
    // we have nothing, so print an error and return nothing.
    Err(e) => { println!("Error reading jsonld: {}", e); None }
  }
}

fn serialize_to<W: Write, T: ?Sized + Serialize>(mut writer: W, value: &T) -> std::io::Result<()> {
  serde_json::to_writer(&mut writer, value)?;
  writer.write_all("\n".as_bytes())
}