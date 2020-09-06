use failure::Error;
use log::info;
use serde::Deserialize;
use tint::Color;

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
    pub fn for_sensor(sensor_id: u16) -> Result<PurpleairResponse, Error> {
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

    // lrapa conversion from http://lar.wsu.edu/nw-airquest/docs/20200610_meeting/NWAQ_20200611_1030_Hadley.pdf
    fn lrapa_pm25(&self) -> f64 {
        0.5 * self.pm25() - 0.66
    }

    // aqi is based on the computations listed on https://docs.google.com/document/d/15ijz94dXJ-YAZLi9iZ_RaBwrZ4KtYeCy08goGBwnbCU/edit
    pub fn aqi(&self) -> Option<f64> {
        let (pm, aqi_upperbound, aqi_lowerbound, pm25_upperbound, pm25_lowerbound) =
            match self.lrapa_pm25() {
                pm if pm > 350.5 => (pm, 500.0, 401.0, 500.0, 350.5),
                pm if pm > 250.5 => (pm, 400.0, 301.0, 350.4, 250.5),
                pm if pm > 150.5 => (pm, 300.0, 201.0, 250.4, 150.5),
                pm if pm > 55.5 => (pm, 200.0, 151.0, 150.4, 55.5),
                pm if pm > 35.5 => (pm, 150.0, 101.0, 55.4, 35.5),
                pm if pm > 12.1 => (pm, 100.0, 51.0, 35.4, 12.1),
                pm if pm >= 0.0 => (pm, 50.0, 0.0, 12.0, 0.0),
                _ => return None,
            };
        // The idea here is to figure out which band of AQI we're in, linerally interpolate that
        // band and then figure out where our current pm25 reading lands on that interpolation.
        let m = (aqi_upperbound - aqi_lowerbound) / (pm25_upperbound - pm25_lowerbound);
        let x = pm - pm25_lowerbound;
        return Some(m * x + aqi_lowerbound);
    }

    pub fn hue(&self) -> Option<Color> {
        if let Some(aqi) = self.aqi() {
            let c = match aqi {
                aqi if aqi > 300.0 => Color::from("maroon"),
                aqi if aqi > 200.0 => Color::from("purple"),
                aqi if aqi > 150.0 => Color::from("red"),
                aqi if aqi > 100.0 => Color::from("orange"),
                aqi if aqi > 50.0 => Color::from("yellow"),
                _ => Color::from("green"),
            };
            return Some(c);
        }
        None
    }
}

impl PurpleAirResult {
    fn pm25(&self) -> Result<f64, std::num::ParseFloatError> {
        self.pm25_value.parse::<f64>()
    }
}
