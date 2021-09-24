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
    /// let contour = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
    /// assert_eq!(4, contour.points_count());
    /// ```
    fn points_count(&self) -> usize;

    /// Returns iterator over vertices of the contour. The iterator will not duplicate the first
    /// of the contour at the end.
    ///
    /// ```
    /// use types::common::Contour;
    ///
    /// let contour = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
    /// for (i, point) in contour.points().enumerate() {
    ///     assert_eq!(point[0], contour[i][0]);
    ///     assert_eq!(point[1], contour[i][1]);
    /// }
    /// ```
    fn points(&'a self) -> Self::PIter;

    /// Returns iterator over segments of the contour. The last segment in the iterator will be
    /// the segment between the last and the first points of the contour.
    ///
    /// ```
    /// use types::common::Contour;
    ///
    /// let contour = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
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
    fn segments(&'a self) -> Segments<'a, T, P, std::iter::Chain<Self::PIter, std::iter::Take<Self::PIter>>> {
        Segments::new(self.points().chain(self.points().take(1)))
    }

    /// Returns perimeter of the contour. The perimeter is calculated in the units, it which
    /// the distance is calculated for point type.
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
