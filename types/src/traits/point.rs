pub trait Point {
    fn distance_square(&self, other: &Self) -> f32;
    fn distance(&self, other: &Self) -> f32 {
        #[cfg(feature = "no-std")]
            {
                return libm::sqrtf(self.distance_square(other));
            }

        #[cfg(not(feature = "no-std"))]
            {
                self.distance_square(other).sqrt()
            }
    }
}

pub trait CartesianPoint {
    fn taxicab_distance(&self, other: &Self) -> f32;
}

pub trait CartesianPoint2 {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
}

impl<P: CartesianPoint2> CartesianPoint for P {
    fn taxicab_distance(&self, other: &Self) -> f32 {
        #[cfg(feature = "no-std")]
            {
                return libm::fabsf(self.x() - other.x()) + libm::fabsf(self.y() - other.y());
            }

        #[cfg(not(feature = "no-std"))]
            {
                (self.x() - other.x()).abs() + (self.y() - other.y()).abs()
            }
    }
}

impl<P: CartesianPoint2> Point for P {
    fn distance_square(&self, other: &Self) -> f32 {
        let dx = self.x() - other.x();
        let dy = self.y() - other.y();
        dx * dx + dy * dy
    }
}

impl CartesianPoint2 for [f32; 2] {
    fn x(&self) -> f32 {
        self[0]
    }

    fn y(&self) -> f32 {
        self[1]
    }
}

impl CartesianPoint2 for (f32, f32) {
    fn x(&self) -> f32 {
        self.0
    }

    fn y(&self) -> f32 {
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
    #[test_case((0.0, 0.0), (1.0, 1.0), 2.0f32.sqrt())]
    #[test_case((0.0, 0.0), (0.0, -1.0), 1.0)]
    #[test_case((-100.0, 0.0), (100.5, 0.0), 200.5)]
    fn distance(p1: (f32, f32), p2: (f32, f32), distance: f32) {
        assert_eq!(distance, p1.distance(&p2));
        assert_eq!(distance, to_slice(&p1).distance(&to_slice(&p2)));
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