mod queries;

use std::{
    error::Error,
    io::{stdin, Read},
    time::{Duration, Instant},
};

use queries::*;
use reqwest::blocking::{Client, Request, Response};
use serde_json::{json, Value};

const GRAPHQL_ENDPOINT: &str = "http://localhost:8080/graphql";

fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let graphql_query = r#"
        query {
            products(priceFilter: { min: 10, max: 100 }) {
                id
                name
                price
            }
        }
    "#;

    let request = build_graphql_request(&client, graphql_query);
    let mut result = measure_request(&client, request)?;
    println!("{:?}", result.0);
    let mut string = String::new();
    result.1.read_to_string(&mut string)?;
    println!("{}", string);
    Ok(())
}

struct MeasureResult(Duration, Response);

fn build_graphql_request(client: &Client, query: &str) -> Request {
    let body = json!({"query": query});

    client
        .post(GRAPHQL_ENDPOINT)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .build()
        .unwrap()
}

fn measure_request(client: &Client, request: Request) -> Result<MeasureResult, Box<dyn Error>> {
    let start_time = Instant::now();
    let response = client.execute(request)?;
    let elapsed_time = Instant::now().duration_since(start_time);
    Ok(MeasureResult(elapsed_time, response))
}
