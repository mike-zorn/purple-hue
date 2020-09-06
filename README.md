# purple-hue
Change a hue light's color based on air quality data from purple air.

## Installation

### Cargo
```
cargo install purple-hue
```

## Configuration
In order to run purple-hue, you need to configure a sensor, light and register a user.

You can find the sensor id from the [purple air map](https://www.purpleair.com/map). The light id is available through [these instructions](https://developers.meethue.com/develop/get-started-2/#turning-a-light-on-and-off). Finally, you can register a user via `purple-hue register-user`. Save all of these in a `purple-hue.toml` file either at `/etc/purple-hue.toml`, `./purple-hue.toml` or `$XDG_CONFIG/purple-hue.toml`. An example configuration follows.
```
light_id = 1
sensor_id = 1
user_id = "user-id"
```

### Systemd
This repository has a systemd unit and timer that can be used to run this as a systemd service every minute.
