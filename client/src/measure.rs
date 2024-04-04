use std::{error::Error, io::Read, time::{Duration, Instant}};

use reqwest::blocking::{Client, Request, Response};

use crate::{num_util::{average, median}, queries::{build_graphql_request, build_rest_request, GraphQlQuery, RestQuery}};

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
        let durations : Vec<Duration> = self.single_results.iter().map(|r| r.duration).collect();
        f.write_fmt(format_args!(
            "{} - Average duration: {:?}, Median duration: {:?}\n{:?}",
            self.name, self.average_duration, self.median_duration, durations
        ))
    }
}

impl<'a> BulkMeasureResult<'a> {
    pub fn from_single_results(single_results: Vec<MeasureResult>, name: &'a str) -> BulkMeasureResult {
        let mut durations_as_micros: Vec<u128> = single_results
            .iter()
            .map(|res| res.duration.as_micros())
            .collect();

        let average_micros = average(&durations_as_micros);
        let average_duration = Duration::from_micros(average_micros as u64);

        let median_micros = median(&mut durations_as_micros);
        let median_duration = Duration::from_micros(median_micros as u64);

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

pub fn measure_rest_query_bulk<'a>(client: &Client, query: &'a RestQuery, iterations: usize) -> Result<BulkMeasureResult<'a>, Box<dyn Error>> {
    let mut single_results = Vec::new();
    for _ in 0..iterations {
        let result = measure_request(client, build_rest_request(client, query))?;
        single_results.push(result);
    }
    Ok(BulkMeasureResult::from_single_results(single_results, &query.name))
}

pub fn measure_graphql_query_bulk<'a>(
    client: &Client,
    query: &'a GraphQlQuery,
    iterations: usize,
) -> Result<BulkMeasureResult<'a>, Box<dyn Error>> {
    let mut single_results = Vec::new();
    for _ in 0..iterations {
        let result = measure_request(client, build_graphql_request(&client, query))?;
        single_results.push(result);
    }
    Ok(BulkMeasureResult::from_single_results(single_results, &query.name))
}