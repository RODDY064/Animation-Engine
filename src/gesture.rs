use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

// ============================================================================
// GESTURE CONTROL - Integrated with Animation
// ============================================================================

#[wasm_bindgen]
pub struct GestureController {
    fraction: f64,
    tracking: bool,
    velocity: f64,
    
    // Physics
    friction: f64,
    spring_tension: f64,
    
    // Position
    start_x: f64,
    start_y: f64,
    current_x: f64,
    current_y: f64,
    last_time: f64,
    
    // Connected animation
    animation: Option<Rc<RefCell<crate::Animation>>>,
}

#[wasm_bindgen]
impl GestureController {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GestureController {
        GestureController {
            fraction: 0.0,
            tracking: false,
            velocity: 0.0,
            friction: 0.92,
            spring_tension: 0.3,
            start_x: 0.0,
            start_y: 0.0,
            current_x: 0.0,
            current_y: 0.0,
            last_time: 0.0,
            animation: None,
        }
    }

    /// Connect to an existing animation for gesture control
    #[wasm_bindgen(js_name = connectAnimation)]
    pub fn connect_animation(&mut self, handle: &crate::AnimationHandle) {
        // Clone the Rc to share ownership
        self.animation = Some(Rc::clone(&handle.animation));
    }

    // ========================================================================
    // GESTURE HANDLERS
    // ========================================================================

    #[wasm_bindgen(js_name = onTapDown)]
    pub fn on_tap_down(&mut self, x: f64, y: f64, timestamp: f64) {
        self.tracking = true;
        self.start_x = x;
        self.start_y = y;
        self.current_x = x;
        self.current_y = y;
        self.last_time = timestamp;
        self.velocity = 0.0;

        // Pause connected animation
        if let Some(ref anim) = self.animation {
            let _ = anim.borrow_mut().pause();
        }
    }

    #[wasm_bindgen(js_name = onTapMove)]
    pub fn on_tap_move(&mut self, x: f64, y: f64, timestamp: f64) {
        if !self.tracking { return; }

        let dy = y - self.current_y;
        let dt = (timestamp - self.last_time).max(1.0);
        
        self.velocity = (dy / dt) * self.friction;
        self.current_y = y;
        self.last_time = timestamp;

        // Update connected animation's fraction
        if let Some(ref anim) = self.animation {
            let displacement = self.current_y - self.start_y;
            let mut anim_ref = anim.borrow_mut();
            let current_fraction = anim_ref.get_fraction_complete();
            let delta = (displacement / 500.0).clamp(-0.1, 0.1);
            let new_fraction = (current_fraction - delta).clamp(0.0, 1.0);
            let _ = anim_ref.set_fraction_complete(new_fraction);
        }

        self.fraction = if let Some(ref anim) = self.animation {
            anim.borrow().get_fraction_complete()
        } else {
            self.fraction
        };
    }

    #[wasm_bindgen(js_name = onTapUp)]
    pub fn on_tap_up(&mut self) {
        self.tracking = false;
        
        if let Some(ref anim) = self.animation {
            let current = anim.borrow().get_fraction_complete();
            
            // Determine completion based on velocity and position
            let should_complete = current > 0.5 || self.velocity > 0.3;
            
            if should_complete {
                let _ = anim.borrow_mut().resume();
            } else {
                // Reverse animation to go back
                let _ = anim.borrow_mut().reverse();
                let _ = anim.borrow_mut().resume();
            }
        }
    }

    #[wasm_bindgen(js_name = onPress)]
    pub fn on_press(&mut self, pressed: bool) -> f64 {
        if pressed { 0.95 } else { 1.0 }
    }

    #[wasm_bindgen(js_name = onHover)]
    pub fn on_hover(&mut self, hovering: bool) -> f64 {
        if hovering { 1.05 } else { 1.0 }
    }

    // ========================================================================
    // PROPERTIES
    // ========================================================================

    #[wasm_bindgen(getter)]
    pub fn fraction(&self) -> f64 {
        self.fraction
    }

    #[wasm_bindgen(getter)]
    pub fn velocity(&self) -> f64 {
        self.velocity
    }

    #[wasm_bindgen(getter, js_name = isTracking)]
    pub fn is_tracking(&self) -> bool {
        self.tracking
    }

    #[wasm_bindgen(js_name = displacement)]
    pub fn displacement(&self) -> f64 {
        self.current_y - self.start_y
    }

    #[wasm_bindgen(setter)]
    pub fn set_friction(&mut self, value: f64) {
        self.friction = value.clamp(0.0, 1.0);
    }

    #[wasm_bindgen(setter, js_name = springTension)]
    pub fn set_spring_tension(&mut self, value: f64) {
        self.spring_tension = value.clamp(0.0, 1.0);
    }
}
