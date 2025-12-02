//! Unit conversion utilities for metric to imperial conversions
//!
//! Data is stored in metric (scientific standard) but displayed in both systems.
//! Using zero-sized types as namespaces for related conversion functions.

/// Temperature conversions
pub struct Temperature;

impl Temperature {
    pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
        (celsius * 9.0 / 5.0) + 32.0
    }
}

/// Distance conversions
pub struct Distance;

impl Distance {
    pub fn mm_to_inches(mm: f64) -> f64 {
        mm * 0.0393701
    }

    pub fn meters_to_feet(meters: f64) -> f64 {
        meters * 3.28084
    }
}

/// Speed conversions
pub struct Speed;

impl Speed {
    pub fn kmh_to_mph(kmh: f64) -> f64 {
        kmh * 0.621371
    }
}

/// Pressure conversions
pub struct Pressure;

impl Pressure {
    pub fn hpa_to_inhg(hpa: f64) -> f64 {
        hpa * 0.02953
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(Temperature::celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(Temperature::celsius_to_fahrenheit(100.0), 212.0);
    }

    #[test]
    fn test_kmh_to_mph() {
        let result = Speed::kmh_to_mph(100.0);
        assert!((result - 62.1371).abs() < 0.0001);
    }

    #[test]
    fn test_mm_to_inches() {
        let result = Distance::mm_to_inches(25.4);
        assert!((result - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_meters_to_feet() {
        let result = Distance::meters_to_feet(1.0);
        assert!((result - 3.28084).abs() < 0.0001);
    }

    #[test]
    fn test_hpa_to_inhg() {
        let result = Pressure::hpa_to_inhg(1013.25);
        assert!((result - 29.92).abs() < 0.01);
    }
}
