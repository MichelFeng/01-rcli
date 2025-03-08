use std::fs;

use csv::Reader;
use serde::{Deserialize, Serialize};

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

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut rdr = Reader::from_path(input)?;
    // let records = rdr
    //     .deserialize()
    //     .map(|record| record.unwrap())
    //     .collect::<Vec<Player>>();

    let mut ret: Vec<Player> = Vec::with_capacity(128);
    for result in rdr.deserialize() {
        let record: Player = result?;
        println!("{:?}", record);
        ret.push(record);
    }

    let json: String = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
}
