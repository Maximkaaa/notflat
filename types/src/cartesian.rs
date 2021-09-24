use num_traits::Float;

pub trait CartesianPoint<T: Float> {
    fn taxicab_distance(&self, other: &Self) -> T;
}

pub trait CartesianPoint2<T: Float> {
    fn x(&self) -> T;
    fn y(&self) -> T;

    fn distance_square(&self, other: &Self) -> T {
        let dx = self.x() - other.x();
        let dy = self.y() - other.y();
        dx * dx + dy * dy
    }

    fn distance(&self, other: &Self) -> T {
        self.distance_square(other).sqrt()
    }
}

impl<T: Float, P: CartesianPoint2<T>> CartesianPoint<T> for P {
    fn taxicab_distance(&self, other: &Self) -> T {
        (self.x() - other.x()).abs() + (self.y() - other.y()).abs()
    }
}

impl<T: Float> CartesianPoint2<T> for [T; 2] {
    fn x(&self) -> T {
        self[0]
    }

    fn y(&self) -> T {
        self[1]
    }
}

impl<T: Float> CartesianPoint2<T> for (T, T) {
    fn x(&self) -> T {
        self.0
    }

    fn y(&self) -> T {
        self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn to_slice(p: &(f32, f32)) -> [f32; 2] {
        [p.0, p.1]
    }

    #[test_case((0.0, 0.0), (0.0, 0.0), 0.0)]
    #[test_case((0.0, 0.0), (1.0, 0.0), 1.0)]
    #[test_case((0.0, 0.0), (1.0, 1.0), 2.0)]
    #[test_case((0.0, 0.0), (0.0, -1.0), 1.0)]
    #[test_case((-100.0, 0.0), (100.5, 0.0), 200.5 * 200.5)]
    fn distance_square(p1: (f32, f32), p2: (f32, f32), distance: f32) {
        assert_eq!(distance, p1.distance_square(&p2));
        assert_eq!(distance, to_slice(&p1).distance_square(&to_slice(&p2)));
    }

    #[test_case((0.0, 0.0), (0.0, 0.0), 0.0)]
    #[test_case((0.0, 0.0), (1.0, 0.0), 1.0)]
    #[test_case((0.0, 0.0), (1.0, 2.0), 3.0)]
    #[test_case((0.0, 0.0), (1.0, -1.0), 2.0)]
    #[test_case((-100.0, 0.0), (100.5, 10.0), 210.5)]
    fn taxicab_distance(p1: (f32, f32), p2: (f32, f32), distance: f32) {
        assert_eq!(distance, p1.taxicab_distance(&p2));
        assert_eq!(distance, to_slice(&p1).taxicab_distance(&to_slice(&p2)));
    }
}
