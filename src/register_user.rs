use hueclient::bridge::Bridge;
use hueclient::HueError;

pub fn register_user(bridge: &mut Bridge) -> Result<String, HueError> {
    loop {
        let r = bridge.register_user("purple-hue");
        match r {
            Err(HueError::BridgeError{ code, .. }) if code == 101 => {
                println!("Push the bridge button");
                std::thread::sleep(::std::time::Duration::from_secs(5));
                continue;
            }
            _ => return r,
        }
    }
}
