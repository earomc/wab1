use std::{
    error::Error, fs, path::PathBuf
};

use chrono::Utc;
use csv::Writer;

use crate::measure::{BulkMeasureResult, MeasureResult};

fn create_path(is_bulk: bool, name: &str) -> PathBuf {
    let mut path = PathBuf::new();
    path.push("results");
    fs::create_dir_all(&path).expect("couldn't create dirs");
    path.push(format!(
        "{}-{}{}-results.csv",
        Utc::now().format("%d-%m-%Y_%H-%M"),
        name.to_string(),
        if is_bulk { "-bulk" } else { "" }
    ));
    path
}


pub fn write_results(results: &Vec<MeasureResult>, name: &str) -> Result<(), Box<dyn Error>> {
    let path = create_path(false, name);

    let path_exists = path.try_exists().unwrap_or(false);
    let mut wtr = Writer::from_path(&path)?;
    if !path_exists {
        wtr.write_record(&["duration_us"])?;
    }
    for result in results {
        wtr.write_record(&[result.duration.as_micros().to_string()])?;
    }

    wtr.flush()?;
    Ok(())
}

pub fn write_bulk_measure_results<'a>(
    results: impl IntoIterator<Item = &'a BulkMeasureResult<'a>>,
    name: &str,
) -> Result<(), Box<dyn Error>> {
    let path = create_path(true, name);

    let path_exists = path.try_exists().unwrap_or(false);
    let mut wtr = Writer::from_path(path)?;
    if !path_exists {
        println!("Path didnt exist! Writing header.");
        wtr.write_record(&["name", "average_us", "median_us"])?;
    } else {
        println!("Path existed");
    }
    for result in results {
        wtr.write_record(&[
            result.name.to_string(),
            result.average_duration.as_micros().to_string(),
            result.median_duration.as_micros().to_string(),
        ])?;
    }
    Ok(())
}