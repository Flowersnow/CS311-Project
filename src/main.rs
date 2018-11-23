#![deny(warnings)]

extern crate futures;
extern crate reqwest;
extern crate tokio;
extern crate serde;
// #[macro_use] extern crate serde_derive;
extern crate serde_json;

// use futures::Future;
use serde_json::Value;
use reqwest::{Client, Response};
use std::str::FromStr;
use std::env;

// use std::collections::LinkedList;
// use std::vec::Vec;
use serde::export::Vec;

fn get_most_recent_price(prices: &mut Vec<f32>) -> f32 {
    let length = prices.len();
    return prices[length - 1]; // get earliest price stock 
}
// use std::collections::HashMap
fn fetch(equity: &str) -> std::result::Result<Vec<f32>, reqwest::Error> {
    let client = Client::new();
    let mut own_string: String = "https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol=".to_owned();
    let bor_string: &str = equity;
    own_string.push_str(bor_string);
    let bor_string2: &str = "&interval=1min&apikey=IVO96IWUXGF22KP9";
    own_string.push_str(bor_string2); 
    println!("{}", own_string);
    let json = |mut res : Response | {
        res.json::<Value>()
    };
    let request1 =
        client
            .get(&own_string)
            .send()
            .and_then(json);
         request1.map(|res1|{
            let obj = res1.as_object().unwrap();
            let meta = obj["Time Series (1min)"].as_object().unwrap();
            let mut stocks: Vec<f32> = Vec::new();
            let start = meta.len() -90;
            let mut index = 0;
            for x in meta {
                if index >= start {
                let cur_stock = x.1.as_object().unwrap();
                let open = cur_stock["1. open"].as_str().unwrap();
                let x = f32::from_str(open).unwrap();
               println!("{:?}", x);
                stocks.push(x);
                index = index + 1;
                }
                index = index + 1;
            }
            println!("{:?}", get_most_recent_price(&mut stocks));
            println!("{:?}", stocks.len());
            return stocks;
        })
        .map_err(|err| {
            println!("stdout error: {}", err);
            return err;
        })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide the equity of your choosing. Need 1 arg");
        return;
    }
    let query = &args[1];
    println!("{:?}", query);
 let x = fetch(query);
match x {
    Ok(v) => println!("{:?}", v.len()),
    Err(e) => println!("error parsing header: {:?}", e),
}
}