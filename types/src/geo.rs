use num_traits::Float;
use crate::datum::{Datum, WGS84};

pub trait GeoPoint<T: Float> {
    fn datum() -> &'static Datum;

    fn lat(&self) -> T;
    fn lon(&self) -> T;

    fn lon_f64(&self) -> f64 {
        self.lon().to_f64().unwrap()
    }

    fn lat_f64(&self) -> f64 {
        self.lat().to_f64().unwrap()
    }

    fn distance(&self, other: &Self) -> T {
        Self::datum().distance(self, other)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wgs84Point<T: Float> {
    lat: T,
    lon: T,
}

impl<T: Float> Wgs84Point<T> {
    pub fn latlon(lat: T, lon: T) -> Self {
        Self {lat, lon}
    }

    pub fn lonlat(lon: T, lat: T) -> Self {
        Self {lat, lon}
    }
}

impl<T: Float> GeoPoint<T> for Wgs84Point<T> {
    fn datum() -> &'static Datum {
        &WGS84
    }

    fn lat(&self) -> T {
        self.lat
    }

    fn lon(&self) -> T {
        self.lon
    }
}

// impl<T: Float> Point<T> for Wgs84Point<T> {
//     fn distance_square(&self, other: &Self) -> T {
//         let distance = self.distance(other);
//         distance * distance
//     }
//
//     fn distance(&self, other: &Self) -> T {
//         GeoPoint::distance(self, other)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "no-std")]
    use alloc::format;

    #[test]
    fn distance() {
        let p1 = Wgs84Point::latlon(35.6544, 139.74477);
        let p2 = Wgs84Point::latlon(21.4225, 39.8261);

        let distance = p1.distance(&p2);
        let expected = 9496707.639;
        assert!((distance - expected).abs() < 0.01, "Distance {} is not close to expected {}", distance, expected);
    }

    #[test]
    fn simple_point_constructors() {
        let p1 = Wgs84Point::latlon(35.6544, 139.74477);
        let p2 = Wgs84Point::lonlat(139.74477, 35.6544);

        assert_eq!(p1, p2);
    }

    #[test]
    fn simple_point_clone() {
        let p1 = Wgs84Point::latlon(35.6544, 139.74477);
        let p2 = p1.clone();

        assert_eq!(p1, p2);
    }

    #[test]
    fn simple_point_debug() {
        let p1 = Wgs84Point::latlon(35.6544, 139.74477);
        assert!(format!("{:?}", p1).contains("35.65"));
    }
}