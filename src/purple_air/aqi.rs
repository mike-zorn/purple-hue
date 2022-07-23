use tint::Color;

pub trait Aqi {
    fn pm25(&self) -> f64;
    
    // lrapa conversion from http://lar.wsu.edu/nw-airquest/docs/20200610_meeting/NWAQ_20200611_1030_Hadley.pdf
    fn lrapa_pm25(&self) -> Option<f64> {
        match 0.5 * self.pm25() - 0.66 {
            adj_pm25 if adj_pm25 >= 0.0 => Some(adj_pm25),
            adj_pm25 if adj_pm25 >= -0.66 => Some(0.0),
            _ => return None, // more negative than the adjustment could cause. This is invalid, so None.
        }
    }

    // aqi is based on the computations listed on https://docs.google.com/document/d/15ijz94dXJ-YAZLi9iZ_RaBwrZ4KtYeCy08goGBwnbCU/edit
    fn aqi(&self) -> Option<f64> {
        let (pm, aqi_upperbound, aqi_lowerbound, pm25_upperbound, pm25_lowerbound) =
            match self.lrapa_pm25() {
                Some(pm) if pm > 350.5 => (pm, 500.0, 401.0, 500.0, 350.5),
                Some(pm) if pm > 250.5 => (pm, 400.0, 301.0, 350.4, 250.5),
                Some(pm) if pm > 150.5 => (pm, 300.0, 201.0, 250.4, 150.5),
                Some(pm) if pm > 55.5 => (pm, 200.0, 151.0, 150.4, 55.5),
                Some(pm) if pm > 35.5 => (pm, 150.0, 101.0, 55.4, 35.5),
                Some(pm) if pm > 12.1 => (pm, 100.0, 51.0, 35.4, 12.1),
                Some(pm) if pm >= 0.0 => (pm, 50.0, 0.0, 12.0, 0.0),
                _ => return None,
            };
        // The idea here is to figure out which band of AQI we're in, linerally interpolate that
        // band and then figure out where our current pm25 reading lands on that interpolation.
        let m = (aqi_upperbound - aqi_lowerbound) / (pm25_upperbound - pm25_lowerbound);
        let x = pm - pm25_lowerbound;
        return Some(m * x + aqi_lowerbound);
    }

    fn hue(&self) -> Option<Color> {
        if let Some(aqi) = self.aqi() {
            let c = match aqi {
                aqi if aqi > 300.0 => Color::from("maroon"),
                aqi if aqi > 200.0 => Color::from("purple"),
                aqi if aqi > 150.0 => Color::from("red"),
                aqi if aqi > 100.0 => Color::from("orange"),
                aqi if aqi > 50.0 => Color::from("yellow"),
                _ => Color::from("green"),
            };
            return Some(c);
        }
        None
    }
}
