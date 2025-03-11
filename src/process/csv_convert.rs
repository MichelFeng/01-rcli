use std::fs;

use csv::{Reader, StringRecord};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::cli::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader: Reader<fs::File> = Reader::from_path(input)?;
    // let records = rdr
    //     .deserialize()
    //     .map(|record| record.unwrap())
    //     .collect::<Vec<Player>>();

    let mut ret: Vec<Value> = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record: StringRecord = result?;
        // headers.iter() -> ["name", "position", "DOB", "nationality", "Kit Number"]
        // record.iter() -> ["Lionel Messi", "Forward", "24-06-1987", "Argentina", "10"]
        // zip -> [("name", "Lionel Messi"), ("position", "Forward"), ...]
        // collect::<Value>() -> {"name": "Lionel Messi", "position": "Forward", ...}
        let value = headers.iter().zip(record.iter()).collect::<Value>();

        // let iter = headers.iter().zip(record.iter());
        // let value = match format {
        //     OutputFormat::Json => iter.collect::<serde_json::Value>(),
        //     OutputFormat::Yaml => iter.collect::<serde_yaml::Value>(),
        //     OutputFormat::Toml => iter.collect::<toml::Value>(),
        // };
        ret.push(value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        OutputFormat::Toml => toml::to_string(&ret)?,
    };

    fs::write(output, content)?;
    Ok(())
}
