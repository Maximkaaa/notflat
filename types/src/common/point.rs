use num_traits::Float;

pub trait Point<T: Float> {
    fn distance_square(&self, other: &Self) -> T;
    fn distance(&self, other: &Self) -> T {
        self.distance_square(other).sqrt()
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
    #[test_case((0.0, 0.0), (1.0, 1.0), 2.0f32.sqrt())]
    #[test_case((0.0, 0.0), (0.0, -1.0), 1.0)]
    #[test_case((-100.0, 0.0), (100.5, 0.0), 200.5)]
    fn distance(p1: (f32, f32), p2: (f32, f32), distance: f32) {
        assert_eq!(distance, p1.distance(&p2));
        assert_eq!(distance, to_slice(&p1).distance(&to_slice(&p2)));
    }
}