use num_traits::Float;
use crate::datum::{Datum, WGS84};

pub trait GeoPoint<T: Float> {
    fn lon(&self) -> T;
    fn lat(&self) -> T;

    fn lon_f64(&self) -> f64 {
        self.lon().to_f64().unwrap()
    }

    fn lat_f64(&self) -> f64 {
        self.lat().to_f64().unwrap()
    }

    fn geo_distance(&self, other: &Self, datum: &Datum) -> T {
        datum.distance(self, other)
    }
}

pub trait PointWithDatum<'a, T: Float>: GeoPoint<T> {
    fn datum(&self) -> &'a Datum;

    fn distance(&self, other: &Self) -> T {
        self.geo_distance(other, self.datum())
    }
}

pub trait Wgs84Point<T: Float>: GeoPoint<T> {
}

impl<T: Float, P: Wgs84Point<T>> PointWithDatum<'static, T> for P {
    fn datum(&self) -> &'static Datum {
        &WGS84
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleGeoPoint<T: Float> {
    lat: T,
    lon: T,
}

impl<T: Float> SimpleGeoPoint<T> {
    pub fn from_lonlat(lon: T, lat: T) -> Self {
        Self {lat, lon}
    }

    pub fn from_latlon(lat: T, lon: T) -> Self {
        Self {lat, lon}
    }
}

impl<T: Float> GeoPoint<T> for SimpleGeoPoint<T> {
    fn lon(&self) -> T {
        self.lon
    }

    fn lat(&self) -> T {
        self.lat
    }
}

impl<T: Float> Wgs84Point<T> for SimpleGeoPoint<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "no-std")]
    use alloc::format;

    #[test]
    fn distance() {
        let p1 = SimpleGeoPoint::from_latlon(35.6544, 139.74477);
        let p2 = SimpleGeoPoint::from_latlon(21.4225, 39.8261);

        let distance = p1.distance(&p2);
        let expected = 9496707.639;
        assert!((distance - expected).abs() < 0.01, "Distance {} is not close to expected {}", distance, expected);
    }

    #[test]
    fn simple_point_constructors() {
        let p1 = SimpleGeoPoint::from_latlon(35.6544, 139.74477);
        let p2 = SimpleGeoPoint::from_lonlat(139.74477, 35.6544);

        assert_eq!(p1, p2);
    }

    #[test]
    fn simple_point_clone() {
        let p1 = SimpleGeoPoint::from_latlon(35.6544, 139.74477);
        let p2 = p1.clone();

        assert_eq!(p1, p2);
    }

    #[test]
    fn simple_point_debug() {
        let p1 = SimpleGeoPoint::from_latlon(35.6544, 139.74477);
        assert!(format!("{:?}", p1).contains("35.65"));
    }
}