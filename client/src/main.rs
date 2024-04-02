mod num_util;
mod queries;
mod measure;

use std::{error::Error, time::Duration};
use queries::{get_graphql_queries, get_rest_queries};
use measure::{measure_graphql_request_bulk, measure_rest_request_bulk};

pub const HOST_NAME: &str = "http://localhost:8080";
pub const GRAPHQL_ENDPOINT: &str = "http://localhost:8080/graphql";

fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let iterations = 10000;

    let graphql_queries = get_graphql_queries();
    let rest_queries = get_rest_queries();

    let bulk_result_gql = measure_graphql_request_bulk(&client, graphql_queries.get(0).unwrap(), iterations)?;
    
    let rest_query0 = rest_queries.get(0).unwrap();
    let bulk_result_rest = measure_rest_request_bulk(&client, &rest_query0.0, &rest_query0.1, iterations)?;

    println!("GraphQL Result {:?}", bulk_result_gql);
    println!("Rest Result {:?}", bulk_result_rest);

    Ok(())
}