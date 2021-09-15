use crate::traits::Point;

pub struct Segment<'a, P: Point> {
    start: &'a P,
    end: &'a P,
}

impl<'a, P: Point> Segment<'a, P> {
    pub fn length(&self) -> f32 {
        self.start.distance(self.end)
    }

    pub fn start(&self) -> &P {
        self.start
    }

    pub fn end(&self) -> &P {
        self.end
    }
}

pub struct Segments<'a, P: Point, PIter: Iterator<Item = &'a P>> {
    prev_point: Option<&'a P>,
    points_iter: PIter,
}

impl<'a, P: Point, PIter: Iterator<Item = &'a P>> Segments<'a, P, PIter> {
    pub fn new(mut points_iter: PIter) -> Self {
        let first_point = points_iter.next();
        Self {
            prev_point: first_point,
            points_iter,
        }
    }
}

impl<'a, P: Point, PIter: Iterator<Item = &'a P>> Iterator for Segments<'a, P, PIter> {
    type Item = Segment<'a, P>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(val) = self.prev_point {
            if let Some(next_val) = self.points_iter.next() {
                let segment = Segment { start: val, end: next_val };
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
