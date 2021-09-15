use core::ops::Index;
use core::slice::Iter;

#[cfg(feature = "no-std")]
use alloc::vec::Vec;

use super::Point;
use crate::segment::Segments;
use num_traits::Float;

pub trait Polyline<'a, T: Float, P: 'a + Point<T>, PIter: Iterator<Item = &'a P>>: Index<usize> {
    fn points_count(&self) -> usize;
    fn points(&'a self) -> PIter;

    fn segments(&'a self) -> Segments<'a, T, P, PIter> {
        Segments::new(self.points())
    }

    fn length(&'a self) -> T {
        let mut length = T::zero();
        for segment in self.segments() {
            length = length + segment.length();
        }

        length
    }
}

impl<'a, T: Float, P: Point<T>> Polyline<'a, T, P, core::slice::Iter<'a, P>> for Vec<P> {
    fn points_count(&self) -> usize {
        Vec::len(self)
    }

    fn points(&'a self) -> core::slice::Iter<'a, P> {
        self.iter()
    }
}

impl<'a, T: Float, P: Point<T>> Polyline<'a, T, P, core::slice::Iter<'a, P>> for [P] {
    fn points_count(&self) -> usize {
        Self::len(self)
    }

    fn points(&'a self) -> Iter<'a, P> {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::CartesianPoint2;
    use test_case::test_case;

    #[cfg(feature = "no-std")]
    use alloc::vec;

    #[test_case(2)]
    #[test_case(3)]
    #[test_case(10)]
    fn iter_segments(points_count: usize) {
        let mut points = vec![];
        for i in 0..points_count {
            points.push((i as f32, i as f32));
        }

        let polyline = points;
        assert_eq!(points_count - 1, polyline.segments().count());
        for segment in polyline.segments() {
            assert_eq!(1.0, segment.end().x() - segment.start().x());
            assert_eq!(1.0, segment.end().y() - segment.start().y());
        }
    }

    #[test_case(0)]
    #[test_case(1)]
    fn iter_segments_zero_length(points_count: usize) {
        let polyline = vec![(0.0, 0.0); points_count];
        assert_eq!(0, polyline.segments().count());
    }

    #[test]
    fn zero_length() {
        let polyline = [(0.0, 0.0); 0];
        assert_eq!(0.0, polyline.length());

        let polyline = [(1.0, 1.0)];
        assert_eq!(0.0, polyline.length());

        let polyline = [(1.0, 1.0), (1.0, 1.0)];
        assert_eq!(0.0, polyline.length());

        let polyline = [(1.0, 1.0); 10];
        assert_eq!(0.0, polyline.length());
    }

    #[test]
    fn length() {
        let polyline = [(0.0, 0.0), (1.0, 0.0)];
        assert_eq!(1.0, polyline.length());

        let polyline = [(0.0, 0.0), (1.0, 0.0), (2.0, 1.0)];
        assert_eq!(1.0 + 2.0f32.sqrt(), polyline.length());
    }

    #[test_case(0)]
    #[test_case(1)]
    #[test_case(2)]
    #[test_case(10)]
    fn points_count(count: usize) {
        let polyline = vec![(0.0, 0.0); count];
        assert_eq!(count, polyline.points_count());
        assert_eq!(count, polyline[..].points_count());
    }

    #[test]
    fn iterating() {
        let polyline = [(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)];
        for (i, p) in polyline.points().enumerate() {
            assert_eq!(i as f32, p.x());
        }
    }
}