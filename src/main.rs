#![deny(warnings)]

extern crate futures;
extern crate reqwest;
extern crate tokio;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

// use futures::Future;
use serde_json::Value;
use reqwest::{Client, Response};
use std::str::FromStr;
use std::collections::LinkedList;

// use std::collections::HashMap;
#[derive(Deserialize, Debug)]
struct Slideshow {
    title: String,
    author: String,
}

#[derive(Deserialize, Debug)]
struct SlideshowContainer {
    slideshow: Slideshow,
}


fn fetch() -> std::result::Result<std::collections::LinkedList<f32>, reqwest::Error> {
    let client = Client::new();

    let json = |mut res : Response | {
        res.json::<Value>()
    };
    let request1 =
        client
            .get("https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol=GOOGL&interval=1min&apikey=IVO96IWUXGF22KP9")
            .send()
            .and_then(json);
         request1.map(|res1|{
            let obj = res1.as_object().unwrap();
            let meta = obj["Time Series (1min)"].as_object().unwrap();
            let mut stocks = LinkedList::new();
            for x in meta {
                let cur_stock = x.1.as_object().unwrap();
                let open = cur_stock["1. open"].as_str().unwrap();
                let x = f32::from_str(open).unwrap();
                println!("{:?}", x);
                stocks.push_back(x);
            }
            println!("{:?}", stocks.len());
            return stocks;
        })
        .map_err(|err| {
            println!("stdout error: {}", err);
            return err;
        })
}

fn main() {
 let x = fetch();
match x {
    Ok(v) => println!("{:?}", v.len()),
    Err(e) => println!("error parsing header: {:?}", e),
}
}