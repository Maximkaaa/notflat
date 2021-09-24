use crate::common::{Polyline};
use num_traits::{Float, FloatConst};
use core::iter::FromIterator;
use crate::cartesian::CartesianPoint2;
use crate::geo::GeoPoint;

use core::fmt::Display;
use crate::error::NotFlatError;

pub trait Projection<'a, T: Float, PFrom: 'a, PTo: 'a>
{
    type IntermediatePoint;

    fn project(&self, point: &PFrom, constructor: &dyn Fn(Self::IntermediatePoint) -> PTo) -> Result<PTo, NotFlatError>;
    fn project_line<SourcePoly, ResultPoly>(&'a self, polyline: &'a SourcePoly, constructor: &dyn Fn(Self::IntermediatePoint) -> PTo) -> Result<ResultPoly, NotFlatError>
        where SourcePoly: Polyline<'a, T, PFrom>,
              ResultPoly: Polyline<'a, T, PTo> + FromIterator<PTo>
    {
         polyline.points().map(|p| self.project(p, constructor)).collect()
    }
}

/// Web mercator direct projection.
///
/// # Example
///
/// ```
/// use types::projection::{WebMercator, Projection};
/// use types::geo::Wgs84Point;
/// use assert_float_eq::*;
///
/// let projection = WebMercator {};
/// let p = Wgs84Point::latlon(55.0, 37.0);
///
/// let (x, y) = projection.project(&p, &|(x, y)| (x, y)).unwrap();
/// assert_float_absolute_eq!(4118821.1593511, x, 0.000001);
/// assert_float_absolute_eq!(7361866.11305118, y, 0.000001);
/// ```
pub struct WebMercator {}

impl<'a, T: FloatConst + Float + Display, PFrom: 'a + GeoPoint<T>, PTo: 'a + CartesianPoint2<T>> Projection<'a, T, PFrom, PTo> for WebMercator {
    type IntermediatePoint = (T, T);

    fn project(&self, point: &PFrom, constructor: &dyn Fn(Self::IntermediatePoint) -> PTo) -> Result<PTo, NotFlatError> {
        let lon = point.lon().to_radians();
        let lat = point.lat().to_radians();
        let a: T = PFrom::datum().a();
        let x = a * lon;
        let y: T = a * (T::FRAC_PI_4()  + lat / T::from(2.0).unwrap()).tan().ln();

        if y.is_finite() {
            Ok(constructor((x, y)))
        } else {
            Err(NotFlatError::InvalidLatitude(point.lat_f64()))
        }
    }
}