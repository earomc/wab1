use std::{error::Error, path::Path};

use csv::Writer;

use crate::measure::{BulkMeasureResult, MeasureResult};

pub fn write_results(results: &Vec<MeasureResult>, name: &str) -> Result<(), Box<dyn Error>> {
    let path = name.to_string() + "-results.csv";
    let path = Path::new(&path);

    let mut wtr = Writer::from_path(path)?;
    if !path.exists() {
        wtr.write_record(&["duration_us", ])?;
    }
    for result in results {
        wtr.write_record(&[result.duration.as_micros().to_string(), ])?;
    }

    wtr.flush()?;
    Ok(())
}

pub fn write_bulk_measure_results(results: &Vec<BulkMeasureResult>, name: &str) -> Result<(), Box<dyn Error>> {
    let path = name.to_string() + "-bulk-results.csv";
    let path = Path::new(&path);
    
    let exists = path.try_exists().unwrap();
    let mut wtr = Writer::from_path(path)?;
    if !exists {
        println!("Path didnt exist! Writing header.");
        wtr.write_record(&["name", "average_us", "median_us"])?;
    } else {
        println!("Path existed");
    }
    for result in results {
        wtr.write_record(&[result.name.to_string(), result.average_duration.as_micros().to_string(), result.median_duration.as_micros().to_string()])?;
    }
    Ok(())
}