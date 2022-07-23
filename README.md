# purple-hue
Change a hue light's color based on air quality data from purple air.

![Rust](https://github.com/apechimp/purple-hue/workflows/Rust/badge.svg)

## Installation

### Cargo
```
cargo install purple-hue
```

### ArchLinux

[`purple-hue`](https://aur.archlinux.org/packages/purple-hue/) is in the [AUR](https://wiki.archlinux.org/index.php/Arch_User_Repository#Installing_and_upgrading_packages).

## Configuration
In order to run purple-hue, you need to configure a sensor, light and register a user.

You can find the sensor id from the [purple air map](https://www.purpleair.com/map). The light id is available through [these instructions](https://developers.meethue.com/develop/get-started-2/#turning-a-light-on-and-off). Finally, you can register a user via `purple-hue register-user`. Save all of these in a `purple-hue.toml` file either at `/etc/purple-hue.toml`, `./purple-hue.toml` or `$XDG_CONFIG/purple-hue.toml`. An example configuration follows.
```
light_id = 1
sensor_id = 1
user_id = "user-id"
```

Instead of providing a `sensor_id`, you can also set a `sensor_ip` if you are able to access the http api of a given sensor.

### Systemd
This repository has a systemd unit and timer that can be used to run this as a systemd service every minute.
