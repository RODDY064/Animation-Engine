use wasm_bindgen::prelude::*;

/// Gesture Recognition 
#[wasm_bindgen]
pub struct GestureRecognizer {
    touch_start_x: f64,
    touch_start_y: f64,
    touch_current_x: f64,
    touch_current_y: f64,
    is_active: bool,
    velocity_x: f64,
    velocity_y: f64,
    last_time: f64,
}

#[wasm_bindgen]
impl GestureRecognizer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GestureRecognizer {
        GestureRecognizer {
            touch_start_x: 0.0,
            touch_start_y: 0.0,
            touch_current_x: 0.0,
            touch_current_y: 0.0,
            is_active: false,
            velocity_x: 0.0,
            velocity_y: 0.0,
            last_time: 0.0,
        }
    }

    /// Handle touch start
    #[wasm_bindgen]
    pub fn on_touch_start(&mut self, x: f64, y: f64, timestamp: f64) {
        self.touch_start_x = x;
        self.touch_start_y = y;
        self.touch_current_x = x;
        self.touch_current_y = y;
        self.is_active = true;
        self.last_time = timestamp;
        self.velocity_x = 0.0;
        self.velocity_y = 0.0;
    }

    /// Handle touch move
    #[wasm_bindgen]
    pub fn on_touch_move(&mut self, x: f64, y: f64, timestamp: f64) {
        let dx = x - self.touch_current_x;
        let dy = y - self.touch_current_y;
        let dt = (timestamp - self.last_time).max(0.001);

        self.velocity_x = dx / dt;
        self.velocity_y = dy / dt;

        self.touch_current_x = x;
        self.touch_current_y = y;
        self.last_time = timestamp;
    }

    /// Handle touch end
    #[wasm_bindgen]
    pub fn on_touch_end(&mut self) {
        self.is_active = false;
    }

    /// Get total displacement X
    #[wasm_bindgen]
    pub fn get_displacement_x(&self) -> f64 {
        self.touch_current_x - self.touch_start_x
    }

    /// Get total displacement Y
    #[wasm_bindgen]
    pub fn get_displacement_y(&self) -> f64 {
        self.touch_current_y - self.touch_start_y
    }

    /// Get current velocity X
    #[wasm_bindgen]
    pub fn get_velocity_x(&self) -> f64 {
        self.velocity_x
    }

    /// Get current velocity Y
    #[wasm_bindgen]
    pub fn get_velocity_y(&self) -> f64 {
        self.velocity_y
    }

    /// Check if gesture is active
    #[wasm_bindgen]
    pub fn is_gesture_active(&self) -> bool {
        self.is_active
    }

    /// Get distance traveled
    #[wasm_bindgen]
    pub fn get_distance(&self) -> f64 {
        let dx = self.touch_current_x - self.touch_start_x;
        let dy = self.touch_current_y - self.touch_start_y;
        (dx * dx + dy * dy).sqrt()
    }
}