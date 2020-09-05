use config::{Config, ConfigError, File};
use hueclient::bridge::Bridge;
use hueclient::HueError;
use serde::Deserialize;
use tint::Color;

fn main() {
    let mut bridge = Bridge::discover_required();
    println!("bridge {:#?}", bridge);

    let Settings {
        user_id,
        sensor_id,
        light_id,
    } = match Settings::new() {
        Ok(settings) => settings,
        Err(why) => panic!("{:?}", why),
    };

    //let user = match register_user(&bridge) {
    //    Err(why) => panic!("{:?}", why),
    //    Ok(user) => user,
    //};
    //println!("username {}", user);
    bridge = bridge.with_user(user_id.into());
    let lights = match bridge.get_all_lights() {
        Err(why) => panic!("{:?}", why),
        Ok(lights) => lights,
    };
    println!("lights {:?}", lights);
    for light in lights.iter() {
        if light.id == light_id {
            if !light.light.state.on {
                panic!("light is off");
            }
            break;
        }
    }

    let purpleair_response = match get_purple_air_for_sensor(sensor_id) {
        Err(why) => panic!("{:?}", why),
        Ok(purpleair_response) => purpleair_response,
    };
    println!("aqi {}", purpleair_response.aqi().unwrap());

    let color = purpleair_response.hue().unwrap();
    println!("color {:?}", color);

    let (hue, sat, bri) = color.to_hsv();
    println!("hue {} sat {} bri {}", hue, sat, bri);
    let command = hueclient::bridge::CommandLight::default()
        .with_hue((hue * 65535.0 / 360.0) as u16)
        .with_sat((sat * 255.0) as u8)
        .with_bri((bri * 255.0) as u8);
    if let Err(why) = bridge.set_light_state(light_id, &command) {
        panic!("{:?}", why);
    };
}

fn get_purple_air_for_sensor(
    sensor_id: u16,
) -> Result<PurpleairResponse, Box<dyn std::error::Error>> {
    let url = format!("https://www.purpleair.com/json?show={}", sensor_id);
    let resp = reqwest::blocking::get(&url)?.json::<PurpleairResponse>()?;
    println!("{:#?}", resp);
    Ok(resp)
}

fn register_user(bridge: &Bridge) -> Result<String, HueError> {
    loop {
        let r = bridge.register_user("purple-hue");
        match r {
            Err(HueError(hueclient::HueErrorKind::BridgeError(code, _), _)) if code == 101 => {
                println!("Push the bridge button");
                std::thread::sleep(::std::time::Duration::from_secs(5));
                continue;
            }
            _ => return r,
        }
    }
}

#[derive(Debug, Deserialize)]
struct PurpleairResponse {
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
    fn aqi(&self) -> Option<f64> {
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

    fn hue(&self) -> Option<Color> {
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

#[derive(Debug, Deserialize)]
struct Settings {
    light_id: usize,
    sensor_id: u16,
    user_id: String,
}

impl Settings {
    fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("settings"))?;
        s.try_into()
    }
}
