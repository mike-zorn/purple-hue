use failure::Error;
use log::info;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PurpleairResponse {
    results: Vec<PurpleAirResult>,
}

#[derive(Debug, Deserialize)]
struct PurpleAirResult {
    #[serde(alias = "Stats")]
    stats: String, // JSON encoded JSON stats...might be useful to get 10 minute average (v1 property)
    #[serde(alias = "PM2_5Value")]
    pm25_value: String,
}

impl PurpleairResponse {
    pub fn for_sensor(sensor_id: u32) -> Result<PurpleairResponse, Error> {
        let url = format!("https://www.purpleair.com/json?show={}", sensor_id);
        let resp = reqwest::blocking::get(&url)?.json::<PurpleairResponse>()?;
        info!("{:#?}", resp);
        Ok(resp)
    }

    fn pm25(&self) -> f64 {
        self.results.iter().fold(0.0, |acc, r| match r.pm25() {
            Ok(pm25) => acc + pm25,
            Err(_) => acc,
        }) / (self.results.len() as f64)
    }

}

impl PurpleAirResult {
    fn pm25(&self) -> Result<f64, std::num::ParseFloatError> {
        self.pm25_value.parse::<f64>()
    }
}
