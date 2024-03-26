mod num_util;
mod queries;

use std::{
    error::Error,
    fmt::Debug,
    io::Read,
    time::{Duration, Instant},
};

use num_util::{average, median};
use queries::*;
use reqwest::blocking::{Client, Request, Response};

pub const GRAPHQL_ENDPOINT: &str = "http://localhost:8080/graphql";

fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let graphql_queries = get_graphql_queries();

    let bulk_result =
        measure_graphql_request_bulk(&client, graphql_queries.get(0).unwrap(), 10000).unwrap();
    
    println!(
        "Average {:?}, Median: {:?}, ",
        bulk_result.average_duration, bulk_result.median_duration
    );
    Ok(())
}

struct MeasureResult {
    duration: Duration,
    response: Response,
    response_string: String,
}

impl Debug for MeasureResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Duration: {:?}, Response: {:?}, Response as String: {}",
            self.duration, self.response, self.response_string
        ))
    }
}

fn measure_request(client: &Client, request: Request) -> Result<MeasureResult, Box<dyn Error>> {
    let start_time = Instant::now();
    let mut response = client.execute(request)?;
    let elapsed_time = Instant::now().duration_since(start_time);

    let mut response_string = String::new();
    response.read_to_string(&mut response_string).unwrap();

    Ok(MeasureResult {
        duration: elapsed_time,
        response,
        response_string,
    })
}

struct BulkMeasureResult {
    average_duration: Duration,
    median_duration: Duration,
    single_results: Vec<MeasureResult>,
}

impl Debug for BulkMeasureResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Average duration: {:?}, Median duration: {:?}, Individual Results {:?}",
            self.average_duration, self.median_duration, self.single_results
        ))
    }
}

impl BulkMeasureResult {
    pub fn from_single_results(single_results: Vec<MeasureResult>) -> BulkMeasureResult {
        let mut durations_as_nanos: Vec<u128> = single_results
            .iter()
            .map(|res| res.duration.as_nanos())
            .collect();

        let average_nanos = average(&durations_as_nanos);
        let average_duration = Duration::from_nanos(average_nanos as u64);

        let median_nanos = median(&mut durations_as_nanos);
        let median_duration = Duration::from_nanos(median_nanos as u64);

        Self { single_results, average_duration, median_duration }
    }
}

fn measure_graphql_request_bulk(
    client: &Client,
    query: &str,
    iterations: usize,
) -> Result<BulkMeasureResult, Box<dyn Error>> {
    let mut single_results = Vec::new();
    for _ in 0..iterations {
        let result = measure_request(client, build_graphql_request(&client, query))?;
        single_results.push(result);
    }

    Ok(BulkMeasureResult::from_single_results(single_results))
}