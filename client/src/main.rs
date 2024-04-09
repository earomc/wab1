mod num_util;
mod queries;
mod measure;
mod write_results;

use std::{env, error::Error, fmt::{write, Display, Pointer}, time::Duration};
use queries::{get_graphql_queries, get_rest_queries};

use crate::measure::{measure_graphql_query_bulk, measure_rest_query_bulk};
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

    let mut bulk_results_gql = Vec::new();
    let mut bulk_results_rest = Vec::new();
    
    for query_gql in &queries_gql {
        bulk_results_gql.push(measure_graphql_query_bulk(&client, query_gql, iterations)?);
    }
    for query_rest in &queries_rest {
        bulk_results_rest.push(measure_rest_query_bulk(&client, query_rest, iterations)?)
    }

    write_results::write_bulk_results(&bulk_results_gql, ApiKind::GraphQl)?;
    write_results::write_bulk_results(&bulk_results_rest, ApiKind::Rest)?;

    for bulk_result_gql in bulk_results_gql {
        println!("GraphQL Result {:?}", bulk_result_gql);
    }
    for bulk_result_rest in bulk_results_rest {
        println!("Rest Result {:?}", bulk_result_rest);
    }
    Ok(())
}

#[derive(Debug)]
pub enum ApiKind {
    GraphQl,
    Rest
}

impl Display for ApiKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}