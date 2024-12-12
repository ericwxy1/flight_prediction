use serde::Deserialize;
use csv::ReaderBuilder;
use std::error::Error;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct FlightRecord {
    #[serde(rename = "data_dte")]
    pub date_str: String,

    #[serde(rename = "Year")]
    pub year: u16,

    #[serde(rename = "Month")]
    pub month: u8,

    #[serde(rename = "usg_apt_id")]
    pub usg_apt_id: u32,

    #[serde(rename = "usg_apt")]
    pub usg_apt: String,

    #[serde(rename = "usg_wac")]
    pub usg_wac: u16,

    #[serde(rename = "fg_apt_id")]
    pub fg_apt_id: u32,

    #[serde(rename = "fg_apt")]
    pub fg_apt: String,

    #[serde(rename = "fg_wac")]
    pub fg_wac: u16,

    #[serde(rename = "airlineid")]
    pub airline_id: u32,

    #[serde(rename = "carrier")]
    pub carrier: String,

    #[serde(rename = "carriergroup")]
    pub carriergroup: u8,

    #[serde(rename = "type")]
    pub flight_type: String,

    #[serde(rename = "Scheduled")]
    pub scheduled: u32,

    #[serde(rename = "Charter")]
    pub charter: u32,

    #[serde(rename = "Total")]
    pub total: u32,
}

pub fn load_data<P: AsRef<Path>>(path: P) -> Result<Vec<FlightRecord>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(path)?;
    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: FlightRecord = result?;
        records.push(record);
    }
    Ok(records)
}

pub fn clean_data(mut records: Vec<FlightRecord>) -> Vec<FlightRecord> {
    records.retain(|r| r.scheduled + r.charter == r.total);
    records
}
