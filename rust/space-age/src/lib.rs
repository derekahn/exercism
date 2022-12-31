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

macro_rules! planet {
    ($i:ident, $e:expr) => {
        pub struct $i;
        impl Planet for $i {
            const ORBIT: f64 = $e;
        }
    };
}

planet!(Earth, 1.0);
planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
