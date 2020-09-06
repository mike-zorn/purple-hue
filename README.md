# purple-hue
Change a hue light's color based on air quality data from purple air

## TODO

- [x] Read settings file for user name
- [x] Read settings file for light name
- [x] set light to a color
- [x] Get air quality from purple air via [this](https://github.com/mrsharpoblunto/purple-rain/blob/master/index.js#L103)
- [x] Convert purple air readings to AQI via [this](https://docs.google.com/document/d/15ijz94dXJ-YAZLi9iZ_RaBwrZ4KtYeCy08goGBwnbCU/edit)

- [x] ~~Read correction factors from settings file~~
- [x] actually set the light based on purple air
- [ ] Split into multiple files
- [ ] Use some kind of logger utility instead of panicking in main everywhere
- [ ] create second utility for adding the user name to the settings file
- [x] use [systemd timers](https://medium.com/horrible-hacks/using-systemd-as-a-better-cron-a4023eea996d) to run on an interval
- [ ] create PKGBUILD to build this for arch
- [ ] upload to AUR
