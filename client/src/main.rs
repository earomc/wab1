mod num_util;
mod queries;
mod measure;
mod write_results;

use std::{env, error::Error, time::Duration};
use queries::{get_graphql_queries, get_rest_queries};
pub const HOST_NAME: &str = "http://localhost:8080";
pub const GRAPHQL_ENDPOINT: &str = "http://localhost:8080/graphql";

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_BACKTRACE", "1");
    
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let iterations = 1000;
    
    let queries_gql = get_graphql_queries();
    let queries_rest = get_rest_queries();

    let mut results_gql = Vec::new();
    let mut results_rest = Vec::new();
    
    write_results::write_bulk_results(&results_gql, "graphql")?;
    write_results::write_bulk_results(&results_rest, "rest")?;

    write_results::write_results(&results_gql.get(0).unwrap().single_results, "graphql_allcustomers")?;
    write_results::write_results(&results_rest.get(0).unwrap().single_results, "rest_allcustomers")?;

    println!("GraphQL Result {:?}", results_gql.get(0).unwrap());
    println!("Rest Result {:?}", results_rest.get(0).unwrap());

    Ok(())
}