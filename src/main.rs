mod purple_air;
mod register_user;
mod settings;

use failure::SyncFailure;
use hueclient::bridge::Bridge;
use log::info;
use quicli::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    verbosity: Verbosity,

    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "register_user")]
    /// Register a user for your hue bridge. Add the output to your config file
    RegisterUser,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    args.verbosity.setup_env_logger("purple-hue")?;

    let mut bridge = match Bridge::discover() {
        Some(bridge) => bridge,
        None => return Err(err_msg("unable to discover a bridge").into()),
    };
    info!("bridge {:#?}", bridge);

    if let Some(Command::RegisterUser) = args.cmd {
        let user = register_user::register_user(&bridge).map_err(SyncFailure::new)?;
        println!("user_id = \"{}\"", user);
        return Ok(());
    }

    let settings::Settings {
        user_id,
        sensor_id,
        sensor_ip,
        light_id,
    } = settings::Settings::new()?;

    info!("fetching lights");
    bridge = bridge.with_user(user_id.into());
    let lights = bridge.get_all_lights().map_err(SyncFailure::new)?;
    info!("lights {:?}", lights);
    for light in lights.iter() {
        if light.id == light_id {
            if !light.light.state.on {
                return Err(format_err!("light, {}, is off", light_id).into());
            }
            break;
        }
    }

    let aqi = match (sensor_id, sensor_ip) {
        (Some(sensor_id),None) => {
            info!("fetching aqi from purple air sensor, {}", sensor_id);
            purple_air::for_sensor(sensor_id).unwrap()
        },
        (None, Some(sensor_ip)) => {
            info!("fetching aqi from local purple air sensor with ip, {}", sensor_ip);
            purple_air::for_local_sensor(sensor_ip).unwrap()
        },
        (Some(_),Some(_)) => return Err(err_msg("both sensor ip and id specified: pick one").into()),
        (None,None) => return Err(err_msg("no sensor id or sensor ip specified").into()),
    };

    info!("aqi {}", aqi.aqi().unwrap());

    let color = aqi.hue().unwrap();
    info!("color {:?}", color);

    let (hue, sat, bri) = color.to_hsv();
    info!("hue {} sat {} bri {}", hue, sat, bri);

    let command = hueclient::bridge::CommandLight::default()
        .with_hue((hue * 65535.0 / 360.0) as u16)
        .with_sat((sat * 255.0) as u8)
        .with_bri((bri * 255.0) as u8);
    bridge
        .set_light_state(light_id, &command)
        .map_err(SyncFailure::new)?;
    Ok(())
}
