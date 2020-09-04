# purple-hue
Change a hue light's color based on air quality data from purple air

## TODO

- [ ] Read settings file for user name
- [ ] Read settings file for light name
- [ ] set light to a color
- [x] Get air quality from purple air via [this](https://github.com/mrsharpoblunto/purple-rain/blob/master/index.js#L103)
- [ ] Convert purple air readings to AQI via [this](https://docs.google.com/document/d/15ijz94dXJ-YAZLi9iZ_RaBwrZ4KtYeCy08goGBwnbCU/edit)

- [ ] create second utility for adding the user name to the settings file
- [ ] Read correction factors from settings file
- [ ] actually set the light based on purple air
- [ ] use [systemd timers](https://medium.com/horrible-hacks/using-systemd-as-a-better-cron-a4023eea996d) to run on an interval
- [ ] create PKGBUILD to build this for arch
- [ ] upload to AUR
