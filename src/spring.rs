pub struct Spring {
    pub stiffness: f64,
    pub damping: f64,
    pub mass: f64,
    pub velocity: f64,
    pub current: f64,
}

impl Spring {
    pub fn new(stiffness: f64, damping: f64) -> Self {
        Self {
            stiffness,
            damping,
            mass: 1.0,
            velocity: 0.0,
            current: 0.0,
        }
    }

    // iOS UISpringTimingParameters presets
    pub fn default() -> Self {
        Self::new(300.0, 30.0) // Balanced
    }

    pub fn bouncy() -> Self {
        Self::new(250.0, 15.0) // More bounce
    }

    pub fn smooth() -> Self {
        Self::new(400.0, 40.0) // Less bounce
    }

    pub fn update(&mut self, target: f64, delta_time: f64) -> f64 {
        let spring_force = -self.stiffness * (self.current - target);
        let damping_force = -self.damping * self.velocity;
        let acceleration = (spring_force + damping_force) / self.mass;

        self.velocity += acceleration * delta_time;
        self.current += self.velocity * delta_time;

        self.current
    }

    pub fn reset(&mut self, value: f64) {
        self.current = value;
        self.velocity = 0.0;
    }
}
