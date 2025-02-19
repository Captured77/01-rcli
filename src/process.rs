use csv::Reader;
use std::fs;
use serde_json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="PascalCase")]
struct Player  {
    // #[serde(rename = "Name")]
    name: String,
    // #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    // #[serde(rename = "Nationality")]
    nationality:  String,
    #[serde(rename = "Kit Number")]
    kit: u8
}

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let player: Player = result?;
        // println!("{:?}", player);
        ret.push(player) ;
    }
    let json: String = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
} 