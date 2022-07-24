# purple-hue
Change a hue light's color based on air quality data from purple air.

![Rust](https://github.com/mike-zorn/purple-hue/workflows/Rust/badge.svg)

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

## Release
The following instructions are for deploying updates to this repository to crates.io and to the Arch User Repository.

1. Identify the new tag version you'll use for the Release
2. Update the version in Cargo.toml
3. Regenerate lockfile `cargo generate-lockfile`
3. Commit & create a new git tag and push it to the origin repository.
4. `cargo publish`
5. Update the pkgver in the PKGBUILD
6. Download the tar.gz file that github generated for the tag and record the
   sha 512 sum in the PKGBUILD
7. Copy the updated PKGBUILD to the git repository for the AUR package
8. `makepkg --printsrcinfo > .SRCINFO` in the AUR package repository
9. commit the updated PKGBUILD to the AUR repository
