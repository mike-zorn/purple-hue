pub mod aqi;

use failure::Error;
use log::info;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PurpleairResponse {
    results: Vec<PurpleAirResult>,
}

#[derive(Debug, Deserialize)]
struct PurpleAirResult {
    #[serde(alias = "PM2_5Value")]
    pm25_value: String,
}

pub fn for_sensor(sensor_id: u32) -> Result<Box<dyn aqi::Aqi>, Error> {
    let url = format!("https://www.purpleair.com/json?show={}", sensor_id);
    let resp = reqwest::blocking::get(&url)?.json::<PurpleairResponse>()?;
    info!("{:#?}", resp);
    Ok(Box::new(resp))
}

pub fn for_local_sensor(sensor_ip: String) -> Result<Box<dyn aqi::Aqi>, Error> {
    let url =format!("http://{}/json", sensor_ip);
    let resp = reqwest::blocking::get(&url)?.json::<LocalPurpleair>()?;
    info!("{:#?}", resp);
    Ok(Box::new(resp))
}

impl PurpleAirResult {
    fn pm25(&self) -> Result<f64, std::num::ParseFloatError> {
        self.pm25_value.parse::<f64>()
    }
}

impl aqi::Aqi for PurpleairResponse {
    fn pm25(&self) -> f64 {
        self.results.iter().fold(0.0, |acc, r| match r.pm25() {
            Ok(pm25) => acc + pm25,
            Err(_) => acc,
        }) / (self.results.len() as f64)
    }
}

#[derive(Debug, Deserialize)]
struct LocalPurpleair {
    #[serde(alias = "pm2.5_aqi")]
    pm25a_value: f64,
    #[serde(alias = "pm2.5_aqi_b")]
    pm25b_value: f64,
}

impl aqi::Aqi for LocalPurpleair {
    fn pm25(&self) -> f64 {
        (self.pm25a_value + self.pm25b_value) / 2.0
    }
}

