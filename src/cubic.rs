#[derive(Clone)]
pub struct CubicBezier {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

impl CubicBezier {
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self { x1, y1, x2, y2 }
    }

    pub fn linear() -> Self {
        Self::new(0.0, 0.0, 1.0, 1.0)
    }

    pub fn ease_in() -> Self {
        Self::new(0.42, 0.0, 1.0, 1.0)
    }

    pub fn ease_out() -> Self {
        Self::new(0.0, 0.0, 0.58, 1.0)
    }

    pub fn ease_in_out() -> Self {
        Self::new(0.42, 0.0, 0.58, 1.0)
    }

    pub fn fluid_ease_out() -> Self {
        Self::new(0.2, 0.0, 0.0, 1.0)
    }

    pub fn fluid_spring() -> Self {
        Self::new(0.5, 1.2, 0.0, 1.0)
    }

    pub fn smooth() -> Self {
        Self::new(0.4, 0.0, 0.2, 1.0)
    }

    pub fn snappy() -> Self {
        Self::new(0.33, 0.66, 0.66, 1.0)
    }

    pub fn bounce() -> Self {
        Self::new(0.68, -0.55, 0.265, 1.55)
    }

    pub fn default() -> Self {
        Self::new(0.25, 0.1, 0.25, 1.0)
    }

    pub fn emphasized() -> Self {
        Self::new(0.4, 0.0, 0.6, 1.0)
    }

    pub fn solve(&self, t: f64) -> f64 {
        if t <= 0.0 {
            return 0.0;
        }
        if t >= 1.0 {
            return 1.0;
        }

        let mut start = 0.0;
        let mut end = 1.0;
        const EPSILON: f64 = 0.001;

        while end - start > EPSILON {
            let mid = (start + end) / 2.0;
            let x = self.bezier_x(mid);
            if x < t {
                start = mid;
            } else {
                end = mid;
            }
        }

        let final_t = (start + end) / 2.0;
        self.bezier_y(final_t)
    }

    fn bezier_x(&self, t: f64) -> f64 {
        let u = 1.0 - t;
        3.0 * u * u * t * self.x1 + 3.0 * u * t * t * self.x2 + t * t * t
    }

    fn bezier_y(&self, t: f64) -> f64 {
        let u = 1.0 - t;
        3.0 * u * u * t * self.y1 + 3.0 * u * t * t * self.y2 + t * t * t
    }
}