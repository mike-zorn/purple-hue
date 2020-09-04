use hueclient::bridge::Bridge;
use hueclient::HueError;
use serde::Deserialize;


#[tokio::main]
async fn main() {
    let purpleair_response = match get_purple_air_for_sensor(39051).await {
        Err(why) => panic!("{:?}",why),
        Ok(purpleair_response) => purpleair_response,
    };
    println!("aqi {}", purpleair_response.lrapa_pm25());

    let mut bridge = Bridge::discover_required();
    println!("bridge {:#?}", bridge);
    let user = match register_user(&bridge) {
        Err(why) => panic!("{:?}", why),
        Ok(user) => user,
    };
    println!("username {}",user);
    bridge = bridge.with_user(user);
    let lights = match bridge.get_all_lights() {
        Err(why) => panic!("{:?}", why),
        Ok(lights) => lights,
    };
    println!("lights {:?}", lights);
}

async fn get_purple_air_for_sensor(sensor_id: u16) -> Result<PurpleairResponse, Box<dyn std::error::Error>> {
    let url = format!("https://www.purpleair.com/json?show={}", sensor_id);
    let resp = reqwest::get(&url)
        .await?
        .json::<PurpleairResponse>()
        .await?;
    println!("{:#?}", resp);
    Ok(resp)
}

fn register_user(bridge: &Bridge) -> Result<String, HueError> {
    loop {
        let r = bridge.register_user("purple-hue");
        match r {
            Err(HueError(hueclient::HueErrorKind::BridgeError(code,_),_)) if code == 101 => {
                println!("Push the bridge button");
                std::thread::sleep(::std::time::Duration::from_secs(5));
                continue;
            },
            _ => return r,
        }
    }
}

#[derive(Debug, Deserialize)]
struct PurpleairResponse {
    results: Vec<PurpleAirResult>,
}

#[derive(Debug,Deserialize)]
struct PurpleAirResult {
    #[serde(alias = "Stats")]
    stats: String, // JSON encoded JSON stats...might be useful to get 10 minute average (v1 property)
    #[serde(alias = "PM2_5Value")]
    pm25_value: String,
}

impl PurpleairResponse {
    fn pm25(self) -> f64 {
        self.results.iter().fold(0.0, |acc, r| match r.pm25() {
            Ok(pm25) => acc+pm25,
            Err(_) => acc,
        }) / (self.results.len() as f64)
    }

    fn lrapa_pm25(self) -> f64 {
        0.5*self.pm25()-0.66
    }
}

impl PurpleAirResult {
    fn pm25(&self) -> Result<f64, std::num::ParseFloatError> {
        self.pm25_value.parse::<f64>()
    }
}
