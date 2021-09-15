use crate::traits::Point;
use core::marker::PhantomData;
use num_traits::Float;

pub struct Segment<'a, T: Float, P: Point<T>> {
    start: &'a P,
    end: &'a P,
    phantom: PhantomData<T>,
}

impl<'a, T: Float, P: Point<T>> Segment<'a, T, P> {
    pub fn length(&self) -> T {
        self.start.distance(self.end)
    }

    pub fn start(&self) -> &P {
        self.start
    }

    pub fn end(&self) -> &P {
        self.end
    }
}

pub struct Segments<'a, T: Float, P: Point<T>, PIter: Iterator<Item = &'a P>> {
    prev_point: Option<&'a P>,
    points_iter: PIter,
    phantom: PhantomData<T>,
}

impl<'a, T: Float, P: Point<T>, PIter: Iterator<Item = &'a P>> Segments<'a, T, P, PIter> {
    pub fn new(mut points_iter: PIter) -> Self {
        let first_point = points_iter.next();
        Self {
            prev_point: first_point,
            points_iter,
            phantom: PhantomData::default(),
        }
    }
}

impl<'a, T: Float, P: Point<T>, PIter: Iterator<Item = &'a P>> Iterator for Segments<'a, T, P, PIter> {
    type Item = Segment<'a, T, P>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(val) = self.prev_point {
            if let Some(next_val) = self.points_iter.next() {
                let segment = Segment { start: val, end: next_val, phantom: PhantomData::default() };
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
