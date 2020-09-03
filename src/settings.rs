#[derive(Debug, Deserialize)]
struct Settings {
    username: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {

    let mut config = Config::default();
    config.merge(File::with_name("Settings")).unwrap();
    config.try_into()
    }
}
