use core::marker::PhantomData;
use num_traits::Float;
use crate::cartesian::CartesianPoint2;

#[derive(Debug, Clone)]
pub struct Segment<'a, T: Float, P> {
    start: &'a P,
    end: &'a P,
    phantom: PhantomData<T>,
}

impl<'a, T: Float, P> Segment<'a, T, P> {
    pub fn start(&self) -> &P {
        self.start
    }

    pub fn end(&self) -> &P {
        self.end
    }
}

impl<'a, T: Float, P: CartesianPoint2<T>> Segment<'a, T, P> {
    pub fn length(&self) -> T {
        self.start.distance(&self.end)
    }

    pub fn determinant(&self) -> T {
        self.start.x() * self.end.y() - self.start.y() * self.end.x()
    }

    pub fn determinant_shifted(&self, dx: T, dy: T) -> T {
        let x1 = self.start.x() + dx;
        let y1 = self.start.y() + dy;
        let x2 = self.end.x() + dx;
        let y2 = self.end.y() + dy;

        x1 * y2 - x2 * y1
    }
}

pub struct Segments<'a, T: Float, P, PIter: Iterator<Item = &'a P>> {
    prev_point: Option<&'a P>,
    points_iter: PIter,
    phantom: PhantomData<T>,
}

impl<'a, T: Float, P, PIter: Iterator<Item = &'a P>> Segments<'a, T, P, PIter> {
    pub fn new(mut points_iter: PIter) -> Self {
        let first_point = points_iter.next();
        Self {
            prev_point: first_point,
            points_iter,
            phantom: PhantomData::default(),
        }
    }
}

impl<'a, T: Float, P, PIter: Iterator<Item = &'a P>> Iterator for Segments<'a, T, P, PIter> {
    type Item = Segment<'a, T, P>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(val) = self.prev_point {
            if let Some(next_val) = self.points_iter.next() {
                let segment = Segment {start: val, end: next_val, phantom: PhantomData::default()};
                self.prev_point = Some(next_val);
                Some(segment)
            } else {
                None
            }
        } else {
            None
        }
    }
}
