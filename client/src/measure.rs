use std::{error::Error, io::Read, time::{Duration, Instant}};

use reqwest::{blocking::{Client, Request, Response}, Method};

use crate::{num_util::{average, median}, queries::{build_graphql_request, build_rest_request}};

use core::fmt::Debug;

pub struct MeasureResult {
    pub duration: Duration,
    pub response: Response,
    pub response_string: String,
}

impl Debug for MeasureResult {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_fmt(format_args!(
            "Duration: {:?}, Response: {:?}, Response as String: {}",
            self.duration, self.response, self.response_string
        ))
    }
}

pub struct BulkMeasureResult<'a> {
    pub name: &'a str,
    pub average_duration: Duration,
    pub median_duration: Duration,
    pub single_results: Vec<MeasureResult>,
}

impl<'a> Debug for BulkMeasureResult<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Average duration: {:?}, Median duration: {:?}",
            self.average_duration, self.median_duration
        ))
    }
}

impl<'a> BulkMeasureResult<'a> {
    pub fn from_single_results(single_results: Vec<MeasureResult>, name: &'a str) -> BulkMeasureResult {
        let mut durations_as_nanos: Vec<u128> = single_results
            .iter()
            .map(|res| res.duration.as_nanos())
            .collect();

        let average_nanos = average(&durations_as_nanos);
        let average_duration = Duration::from_nanos(average_nanos as u64);

        let median_nanos = median(&mut durations_as_nanos);
        let median_duration = Duration::from_nanos(median_nanos as u64);

        Self { single_results, average_duration, median_duration, name}
    }
}


pub fn measure_request(client: &Client, request: Request) -> Result<MeasureResult, Box<dyn Error>> {
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

pub fn measure_rest_request_bulk<'a>(client: &Client, method: &Method, query: &str, iterations: usize, name: &'a str) -> Result<BulkMeasureResult<'a>, Box<dyn Error>> {
    let mut single_results = Vec::new();
    for _ in 0..iterations {
        let result = measure_request(client, build_rest_request(client, method, query))?;
        single_results.push(result);
    }
    Ok(BulkMeasureResult::from_single_results(single_results, name))
}

pub fn measure_graphql_request_bulk<'a>(
    client: &Client,
    query: &str,
    iterations: usize,
    name: &'a str
) -> Result<BulkMeasureResult<'a>, Box<dyn Error>> {
    let mut single_results = Vec::new();
    for _ in 0..iterations {
        let result = measure_request(client, build_graphql_request(&client, query))?;
        single_results.push(result);
    }
    Ok(BulkMeasureResult::from_single_results(single_results, name))
}