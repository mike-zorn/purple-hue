use hueclient::bridge::Bridge;
use hueclient::HueError;

fn main() {
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
