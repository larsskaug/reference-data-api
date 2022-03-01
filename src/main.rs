use std::fs;
use scraper::Html;
use scraper::Selector;
use std::collections::HashMap;



fn main() {

    let url = "https://www.nrcs.usda.gov/wps/portal/nrcs/detail/?cid=nrcs143_013696";

let mut page = reqwest::blocking::get(url).unwrap()
            .text().unwrap();

page.retain(|c| !c.is_whitespace());    

let document = Html::parse_document(&page);

let selector = &Selector::parse("#detail td")
    .expect("Error during the parsing of table");

let stateName:Vec<_> = document
    .select(selector)
    .flat_map(|el| el.text())
    .collect();

println!("{:?}",stateName);   
} 

