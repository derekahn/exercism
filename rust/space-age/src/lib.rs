#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s as f64)
    }
}

pub trait Planet {
    const ORBIT: f64;

    fn years_during(d: &Duration) -> f64 {
        let earth_year_in_seconds: f64 = 31_557_600.0;
        (d.0 / earth_year_in_seconds) / Self::ORBIT
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const ORBIT: f64 = 0.2408467;
}
impl Planet for Venus {
    const ORBIT: f64 = 0.61519726;
}
impl Planet for Earth {
    const ORBIT: f64 = 1.0;
}
impl Planet for Mars {
    const ORBIT: f64 = 1.8808158;
}
impl Planet for Jupiter {
    const ORBIT: f64 = 11.862615;
}
impl Planet for Saturn {
    const ORBIT: f64 = 29.447498;
}
impl Planet for Uranus {
    const ORBIT: f64 = 84.016846;
}
impl Planet for Neptune {
    const ORBIT: f64 = 164.79132;
}
