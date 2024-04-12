use std::{
    error::Error, fs, path::PathBuf
};

use chrono::Utc;
use csv::{ByteRecord, Writer};

use crate::{measure::BulkMeasureResult, ApiKind};

fn create_path(is_bulk: bool, name: &String) -> PathBuf {
    let mut path = PathBuf::new();
    path.push("results");
    fs::create_dir_all(&path).expect("couldn't create dirs");
    path.push(format!(
        "{}_{}results_{}.csv",
        Utc::now().format("%d-%m-%Y_%H-%M-%S"),
        if is_bulk { "bulk-" } else { "" },
        name,
    ));
    path
}

fn create_path_one_file(api_kind: &ApiKind) -> PathBuf {
    let mut path = PathBuf::new();
    path.push("results");
    fs::create_dir_all(&path).expect("couldn't create dirs");
    path.push(format!(
        "{}_results_{}.csv",
        Utc::now().format("%d-%m-%Y_%H-%M-%S"),
        api_kind.to_string().to_lowercase()
    ));
    path
}

/// writes results in one file
pub fn write_bulk_results_file(bulk_results: &Vec<BulkMeasureResult>, api_kind: &ApiKind) -> Result<(), Box<dyn Error>> {
    let path = create_path_one_file(api_kind);

    let path_exists = path.try_exists().unwrap_or(false);
    let mut wtr = Writer::from_path(path)?;
    
    if !path_exists {
        wtr.write_record(bulk_results.iter().map(|r| r.name))?;
    }
    for record in get_records(bulk_results) {
        wtr.write_byte_record(&record)?;
    }
    wtr.flush()?;
    Ok(())
}

/// Converts a collection of BulkMeasureResults into a collection of records or rows.
/// 
/// Assumes all BulkMeasureResults to have the same amount of single results.
fn get_records(bulk_results: &Vec<BulkMeasureResult>) -> Vec<ByteRecord> {
    let mut records = Vec::new();

    let rows = bulk_results[0].single_results.len(); // rows = iterations for all results
    for row in 0..rows {
        let mut record = ByteRecord::new();
        for bulk_result in bulk_results {
            let duration_us = bulk_result.single_results[row].duration.as_micros();
            let duration_string = duration_us.to_string();
            let field: &[u8] = duration_string.as_bytes();
            record.push_field(field);
        }
        records.push(record);
    }
    records
}

pub fn write_bulk_results(
    bulk_results: &Vec<BulkMeasureResult>,
    api_kind: ApiKind,
) -> Result<(), Box<dyn Error>> {
    write_bulk_results_file(bulk_results, &api_kind)?;

    let path = create_path(true, &api_kind.to_string().to_lowercase());

    let path_exists = path.try_exists().unwrap_or(false);
    let mut wtr = Writer::from_path(path)?;
    if !path_exists {
        println!("Path didnt exist! Writing header.");
        wtr.write_record(["name", "average_us", "median_us"])?;
    } else {
        println!("Path existed");
    }
    for bulk_result in bulk_results {
        wtr.write_record(&[
            bulk_result.name.to_string(),
            bulk_result.average_duration.as_micros().to_string(),
            bulk_result.median_duration.as_micros().to_string(),
        ])?;
    }
    Ok(())
}