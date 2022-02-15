extern crate spider;

use spider::website::Website;
use scraper::{ Html, Selector };
use serde::{Deserialize};
use serde_json::{Result, Value};

fn main() {
  let mut website: Website = Website::new("https://www.lttstore.com/");
  // website.on_link_find_callback = |s| { println!("link target: {}", s); s }; // Callback to run on each link find
  
  website.crawl();
  
  for page in website.get_pages() {
    let html = page.get_html();
    let product: Option<Value> = find_products(&html);
    if product.is_some() {
      println!("{}", product.unwrap()["name"]);
    }
  }
}

fn find_products(html: &Html) -> Option<Value> {
  let jsonld_selector = Selector::parse(r#"script[type="application/ld+json"]"#).unwrap();
  let jsonld_nodes = html.select(&jsonld_selector).into_iter();
  let mut products: Vec<Value> = jsonld_nodes
    .map(|node| node.inner_html())
    .map(|inner_html| serde_json::from_str(&inner_html) as Result<Value>)
    .filter_map(|jsonld| {
        match jsonld {
          Ok(x) => if x["@type"] == "Product" { Some(x) } else { None },
          Err(e) => None
        }
    })
    .collect::<Vec<Value>>();
  return products.pop();
}