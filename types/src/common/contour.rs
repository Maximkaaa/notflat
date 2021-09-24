use crate::segment::Segments;
use num_traits::Float;
use crate::cartesian::CartesianPoint2;

#[cfg(feature = "no-std")]
use alloc::vec::Vec;

pub trait Contour<'a, T: Float, P: 'a> {
    type PIter: Iterator<Item = &'a P>;

    /// Returns the number of the vertices in the contour. This count does not add additional
    /// virtual point at the end of the contour to emulate closed contour.
    ///
    /// ```
    /// use types::common::Contour;
    ///
    /// let contour = [[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
    /// assert_eq!(4, contour.points_count());
    /// ```
    fn points_count(&self) -> usize;

    /// Returns iterator over vertices of the contour. The iterator will not duplicate the first
    /// of the contour at the end.
    ///
    /// ```
    /// use types::common::Contour;
    ///
    /// let contour = [[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
    /// for (i, point) in contour.points().enumerate() {
    ///     assert_eq!(point[0], contour[i][0]);
    ///     assert_eq!(point[1], contour[i][1]);
    /// }
    /// ```
    fn points(&'a self) -> Self::PIter;

    /// Returns iterator over vertices of the contour, but unlike [Contour::points] it adds the
    /// first point at the end of the iterator, making the point contour enclosed.
    ///
    /// ```
    /// use types::common::Contour;
    ///
    /// let contour = [[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
    /// assert_eq!(5, contour.points_closed().count());
    ///
    /// let last_point = contour.points_closed().last().unwrap();
    /// assert_eq!(contour[0][0], last_point[0]);
    /// assert_eq!(contour[0][1], last_point[1]);
    /// ```
    fn points_closed(&'a self) -> core::iter::Chain<Self::PIter, core::iter::Take<Self::PIter>> {
        self.points().chain(self.points().take(1))
    }

    /// Returns iterator over segments of the contour. The last segment in the iterator will be
    /// the segment between the last and the first points of the contour.
    ///
    /// ```
    /// use types::common::Contour;
    ///
    /// let contour = [[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
    /// assert_eq!(4, contour.segments().count());
    ///
    /// let first_segment = contour.segments().next().unwrap();
    /// assert_eq!(contour[0][0], first_segment.start()[0]);
    /// assert_eq!(contour[0][1], first_segment.start()[1]);
    ///
    /// let last_segment = contour.segments().last().unwrap();
    /// assert_eq!(contour[0][0], last_segment.end()[0]);
    /// assert_eq!(contour[0][1], last_segment.end()[1]);
    /// ```
    fn segments(&'a self) -> Segments<'a, T, P, core::iter::Chain<Self::PIter, core::iter::Take<Self::PIter>>> {
        Segments::new(self.points_closed())
    }

    /// Calculates perimeter of the contour. The perimeter is calculated in the units, in which
    /// the distance is calculated for the point type.
    ///
    /// ```
    /// use types::common::Contour;
    ///
    /// let contour = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
    /// assert_eq!(40.0, contour.length());
    /// ```
    fn length(&'a self) -> T;
}

pub trait SurfaceContour<'a, T: Float, P: 'a>: Contour<'a, T, P> {
    /// Calculates area of the contour. The area is calculated in the squared units, in which
    /// the distance is calculated for the point type.
    ///
    /// ```
    /// use types::common::SurfaceContour;
    ///
    /// let contour = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
    /// assert_eq!(100.0, contour.area());
    /// ```
    ///
    /// This method assumes that the contour is not self-intersecting. For self-intersecting contour
    /// the actual result of the calculation will be
    /// `area_of_clockwise_parts - area_of_counterclockwise_parts`;
    ///
    /// ```
    /// use types::common::SurfaceContour;
    ///
    /// let contour = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 0.0], [10.0, 10.0]];
    /// assert_eq!(0.0, contour.area());
    /// ```
    fn area(&'a self) -> T;
}

impl<'a, T: Float, P: 'a + CartesianPoint2<T>> Contour<'a, T, P> for Vec<P> {
    type PIter = core::slice::Iter<'a, P>;

    fn points_count(&self) -> usize {
        Vec::len(self)
    }

    fn points(&'a self) -> Self::PIter {
        self.iter()
    }

    fn length(&'a self) -> T {
        self[..].length()
    }
}

impl<'a, T: Float, P: 'a + CartesianPoint2<T>> Contour<'a, T, P> for [P] {
    type PIter = core::slice::Iter<'a, P>;

    fn points_count(&self) -> usize {
        self.len()
    }

    fn points(&'a self) -> Self::PIter {
        self.iter()
    }

    fn length(&'a self) -> T {
        self.segments().map(|s| s.length()).fold(T::zero(), |acc, x| acc + x)
    }
}

impl<'a, T: Float, P: 'a + CartesianPoint2<T>> SurfaceContour<'a, T, P> for [P] {
    fn area(&'a self) -> T {
        if self.points_count() < 3 {
            return T::zero();
        }

        // We need to shift coordinates to prevent numeric errors when multiplying large numbers
        // (rounding, overflow etc).
        let dx = -self[0].x();
        let dy = -self[0].y();
        self.segments()
            .map(|s| s.determinant_shifted(dx, dy)).fold(T::zero(), |acc, x| acc + x).abs() / T::from(2.0).unwrap()
    }
}

impl<'a, T: Float, P: 'a + CartesianPoint2<T>> SurfaceContour<'a, T, P> for Vec<P> {
    fn area(&'a self) -> T {
        self[..].area()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn area_large_numbers() {
        let contour: &[[f32; 2]] = &[
            [10_000_000.0, 10_000_000.0],
            [10_000_000.0, 10_000_100.0],
            [10_000_100.0, 10_000_100.0],
            [10_000_100.0, 10_000_000.0]];
        assert_eq!(10_000.0, contour.area());
    }

    #[test_case(0)]
    #[test_case(1)]
    #[test_case(2)]
    fn area_few_points(num_points: usize) {
        let mut contour = vec![];
        for i in 0..num_points {
            contour.push([i as f32, (i * 10) as f32])
        }

        assert_eq!(0.0, contour.area());
    }
}