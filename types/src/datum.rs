use geographiclib_rs::{Geodesic, InverseGeodesic};
use crate::geo::GeoPoint;
use num_traits::Float;
use lazy_static::lazy_static;

pub struct Datum {
    geodesic: Geodesic,
}

impl Datum {
    fn new(a: f64, inv_f: f64) -> Self {
        Self {
            geodesic: Geodesic::new(a, inv_f)
        }
    }

    pub fn a<T: Float>(&self) -> T {
        T::from(self.geodesic.a).unwrap()
    }

    pub fn inv_f<T: Float>(&self) -> T {
        T::from(self.geodesic.f).unwrap()
    }

    pub fn distance<T: Float, P: GeoPoint<T> + ?Sized>(&self, p1: &P, p2: &P) -> T {
        let result: f64 = self.geodesic.inverse(p1.lat_f64(), p1.lon_f64(), p2.lat_f64(), p2.lon_f64());
        T::from(result).unwrap()
    }
}

lazy_static! {
    pub static ref WGS84: Datum = Datum::new(6378137.0, 1.0 / 298.257223563);
}
