use crate::cubic::CubicBezier;
use crate::spring::Spring;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlElement, Performance};

mod cubic;
mod spring;

#[derive(Clone)]
pub struct AnimationProps {
    pub opacity: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
    pub scale: Option<f64>,
    pub scale_x: Option<f64>,
    pub scale_y: Option<f64>,
    pub scale_z: Option<f64>,
}

impl AnimationProps {
    pub fn new() -> Self {
        Self {
            opacity: None,
            x: None,
            y: None,
            z: None,
            scale: None,
            scale_x: None,
            scale_y: None,
            scale_z: None,
        }
    }
}

#[wasm_bindgen]
pub struct Animation {
    element: HtmlElement,
    start_props: AnimationProps,
    end_props: AnimationProps,
    current_props: AnimationProps,
    bezier: Option<CubicBezier>,
    springs: Option<Vec<Spring>>,
    duration: f64,
    start_time: f64,
    last_time: f64,
    performance: Performance,
    raf_id: Option<i32>,
    use_spring: bool,
}

#[wasm_bindgen]
impl Animation {
    #[wasm_bindgen(constructor)]
    pub fn new(element: HtmlElement) -> Result<Animation, JsValue> {
        let window = window().ok_or("No window")?;
        let performance = window.performance().ok_or("No performance")?;

        Ok(Animation {
            element,
            start_props: AnimationProps::new(),
            end_props: AnimationProps::new(),
            current_props: AnimationProps::new(),
            bezier: Some(CubicBezier::smooth()),
            springs: None,
            duration: 400.0,
            start_time: 0.0,
            last_time: 0.0,
            performance,
            raf_id: None,
            use_spring: false,
        })
    }


    // ========================================================================
    // CUBIC BEZIER METHODS
    // ========================================================================
    
    #[wasm_bindgen]
    #[wasm_bindgen]
    pub fn cubic(mut self, x1: f64, y1: f64, x2: f64, y2: f64, duration: f64) -> Self {
        self.bezier = Some(CubicBezier::new(x1, y1, x2, y2));
        self.duration = duration;
        self.use_spring = false;
        self
    }
    // Smooth -
    pub fn smooth(mut self, duration: f64) -> Self {
        self.bezier = Some(CubicBezier::smooth());
        self.duration = duration;
        self.use_spring = false;
        self
    }

    /// Snappy - Quick, responsive feel
    #[wasm_bindgen]
    pub fn snappy(mut self, duration: f64) -> Self {
        self.bezier = Some(CubicBezier::snappy());
        self.duration = duration;
        self.use_spring = false;
        self
    }

    /// Bounce - Playful overshoot effect
    #[wasm_bindgen]
    pub fn bounce(mut self, duration: f64) -> Self {
        self.bezier = Some(CubicBezier::bounce());
        self.duration = duration;
        self.use_spring = false;
        self
    }

    /// Ease Out - Classic iOS deceleration
    #[wasm_bindgen]
    pub fn ease_out(mut self, duration: f64) -> Self {
        self.bezier = Some(CubicBezier::ease_out());
        self.duration = duration;
        self.use_spring = false;
        self
    }

    /// Ease In - Acceleration
    #[wasm_bindgen]
    pub fn ease_in(mut self, duration: f64) -> Self {
        self.bezier = Some(CubicBezier::ease_in());
        self.duration = duration;
        self.use_spring = false;
        self
    }

    /// Ease In/Out - Smooth acceleration and deceleration
    #[wasm_bindgen]
    pub fn ease_in_out(mut self, duration: f64) -> Self {
        self.bezier = Some(CubicBezier::ease_in_out());
        self.duration = duration;
        self.use_spring = false;
        self
    }

      #[wasm_bindgen]
    pub fn fluid_ease_out(mut self, duration: f64) -> Self {
        self.bezier = Some(CubicBezier::fluid_ease_out());
        self.duration = duration;
        self.use_spring = false;
        self
    }

     pub fn fluid_spring(mut self, duration: f64) -> Self {
        self.bezier = Some(CubicBezier::fluid_spring());
        self.duration = duration;
        self.use_spring = false;
        self
    }

    // Cubic defualt
     pub fn cubic_defualt(mut self, duration: f64) -> Self {
        self.bezier = Some(CubicBezier::default());
        self.duration = duration;
        self.use_spring = false;
        self
    }

     // Cubic emphasized - 
     pub fn cubic_emphasized(mut self, duration: f64) -> Self {
        self.bezier = Some(CubicBezier::emphasized());
        self.duration = duration;
        self.use_spring = false;
        self
    }
    
    /// Linear - Constant speed
    #[wasm_bindgen]
    pub fn linear(mut self, duration: f64) -> Self {
        self.bezier = Some(CubicBezier::linear());
        self.duration = duration;
        self.use_spring = false;
        self
    }

   // ========================================================================
    // SPRING PHYSICS METHODS
    // ========================================================================

    /// Custom spring physics
    #[wasm_bindgen]
    pub fn spring(mut self, stiffness: f64, damping: f64) -> Self {
        self.springs = Some(vec![Spring::new(stiffness, damping); 8]);
        self.use_spring = true;
        self
    }

    /// Default spring - Balanced feel
    #[wasm_bindgen]
    pub fn spring_default(mut self) -> Self {
        self.springs = Some(vec![Spring::default(); 8]);
        self.use_spring = true;
        self
    }

    /// Bouncy spring - More oscillation
    #[wasm_bindgen]
    pub fn spring_bouncy(mut self) -> Self {
        self.springs = Some(vec![Spring::bouncy(); 8]);
        self.use_spring = true;
        self
    }

    /// Smooth spring - Minimal bounce
    #[wasm_bindgen]
    pub fn spring_smooth(mut self) -> Self {
        self.springs = Some(vec![Spring::smooth(); 8]);
        self.use_spring = true;
        self
    }

    // ========================================================================
    // ANIMATION CONTROL
    // ========================================================================

    // Set target properties
    #[wasm_bindgen]
    pub fn to(
        mut self,
        opacity: Option<f64>,
        x: Option<f64>,
        y: Option<f64>,
        z: Option<f64>,
        scale: Option<f64>,
        scale_x: Option<f64>,
        scale_y: Option<f64>,
        scale_z: Option<f64>,
    ) -> Self {
        self.end_props = AnimationProps {
            opacity,
            x,
            y,
            z,
            scale,
            scale_x,
            scale_y,
            scale_z,
        };
        self
    }

    // Start animation
    #[wasm_bindgen]
    pub fn start(&mut self) -> Result<(), JsValue> {
        self.capture_start_values()?;
        self.start_time = self.performance.now();
        self.last_time = self.start_time;
        self.animate_frame()?;
        Ok(())
    }

    // Capture current element values as start
      fn capture_start_values(&mut self) -> Result<(), JsValue> {
        self.start_props.x = Some(0.0);
        self.start_props.y = Some(0.0);
        self.start_props.z = Some(0.0);
        self.start_props.opacity = Some(1.0);
        self.start_props.scale = Some(1.0);
        self.current_props = self.start_props.clone();

        if let Some(springs) = &mut self.springs {
            for spring in springs.iter_mut() {
                spring.reset(0.0);
            }
        }
        Ok(())
    }

    // Animation loop
    fn animate_frame(&mut self) -> Result<(), JsValue> {
        let now = self.performance.now();
        let delta = (now - self.last_time).min(32.0); // Cap at 32ms
        self.last_time = now;

        let should_continue = if self.use_spring {
            self.update_spring(delta / 1000.0)?
        } else {
            self.update_cubic(now)?
        };

        self.apply_transform()?;

        if should_continue {
            self.request_next_frame()?;
        }

        Ok(())
    }

    // Update with cubic bezier
    fn update_cubic(&mut self, now: f64) -> Result<bool, JsValue> {
        let elapsed = now - self.start_time;
        let progress = (elapsed / self.duration).min(1.0);

        let eased = if let Some(bezier) = &self.bezier {
            bezier.solve(progress)
        } else {
            progress
        };

        self.interpolate_props(eased);

        Ok(progress < 1.0)
    }

    // Update with spring physics
    fn update_spring(&mut self, delta_time: f64) -> Result<bool, JsValue> {
        let mut at_rest = true;

        if let Some(springs) = &mut self.springs {
            let targets = [
                self.end_props.x.unwrap_or(0.0),
                self.end_props.y.unwrap_or(0.0),
                self.end_props.z.unwrap_or(0.0),
                self.end_props.opacity.unwrap_or(1.0),
                self.end_props.scale.unwrap_or(1.0),
                self.end_props.scale_x.unwrap_or(1.0),
                self.end_props.scale_y.unwrap_or(1.0),
                self.end_props.scale_z.unwrap_or(1.0),
            ];

            for (i, spring) in springs.iter_mut().enumerate() {
                let value = spring.update(targets[i], delta_time);

                // Check if spring is still moving
                if spring.velocity.abs() > 0.01 || (value - targets[i]).abs() > 0.01 {
                    at_rest = false;
                }

                // Update current props
                match i {
                    0 => self.current_props.x = Some(value),
                    1 => self.current_props.y = Some(value),
                    2 => self.current_props.z = Some(value),
                    3 => self.current_props.opacity = Some(value),
                    4 => self.current_props.scale = Some(value),
                    5 => self.current_props.scale_x = Some(value),
                    6 => self.current_props.scale_y = Some(value),
                    7 => self.current_props.scale_z = Some(value),
                    _ => {}
                }
            }
        }

        Ok(!at_rest)
    }

    // Interpolate properties
    fn interpolate_props(&mut self, t: f64) {
        let lerp = |start: f64, end: f64| start + (end - start) * t;

        if let (Some(start), Some(end)) = (self.start_props.opacity, self.end_props.opacity) {
            self.current_props.opacity = Some(lerp(start, end));
        }
        if let (Some(start), Some(end)) = (self.start_props.x, self.end_props.x) {
            self.current_props.x = Some(lerp(start, end));
        }
        if let (Some(start), Some(end)) = (self.start_props.y, self.end_props.y) {
            self.current_props.y = Some(lerp(start, end));
        }
        if let (Some(start), Some(end)) = (self.start_props.z, self.end_props.z) {
            self.current_props.z = Some(lerp(start, end));
        }
        if let (Some(start), Some(end)) = (self.start_props.scale, self.end_props.scale) {
            self.current_props.scale = Some(lerp(start, end));
        }
    }

    // Apply transform to element
    fn apply_transform(&self) -> Result<(), JsValue> {
        let style = self.element.style();

        // Build transform string
        let mut transforms = Vec::new();

        let x = self.current_props.x.unwrap_or(0.0);
        let y = self.current_props.y.unwrap_or(0.0);
        let z = self.current_props.z.unwrap_or(0.0);

        // Round to pixels for crisp rendering
        let x_rounded = x.round();
        let y_rounded = y.round();
        let z_rounded = z.round();

        if x_rounded != 0.0 || y_rounded != 0.0 || z_rounded != 0.0 {
            transforms.push(format!(
                "translate3d({}px, {}px, {}px)",
                x_rounded, y_rounded, z_rounded
            ));
        }

        if let Some(scale) = self.current_props.scale {
            transforms.push(format!("scale({})", scale));
        }

        if !transforms.is_empty() {
            style.set_property("transform", &transforms.join(" "))?;
        }

        if let Some(opacity) = self.current_props.opacity {
            style.set_property("opacity", &opacity.to_string())?;
        }

        Ok(())
    }

    // Request next animation frame
    fn request_next_frame(&self) -> Result<(), JsValue> {
        let window = window().ok_or("No window")?;

        let animation = Rc::new(RefCell::new(self.clone()));
        let closure = Closure::wrap(Box::new(move || {
            if let Ok(mut anim) = animation.try_borrow_mut() {
                let _ = anim.animate_frame();
            }
        }) as Box<dyn FnMut()>);

        window.request_animation_frame(closure.as_ref().unchecked_ref())?;
        closure.forget();

        Ok(())
    }
}

impl Clone for Animation {
    fn clone(&self) -> Self {
        Self {
            element: self.element.clone(),
            start_props: self.start_props.clone(),
            end_props: self.end_props.clone(),
            current_props: self.current_props.clone(),
            bezier: self.bezier.clone(),
            springs: self.springs.clone(),
            duration: self.duration,
            start_time: self.start_time,
            last_time: self.last_time,
            performance: self.performance.clone(),
            raf_id: self.raf_id,
            use_spring: self.use_spring,
        }
    }
}

impl Clone for CubicBezier {
    fn clone(&self) -> Self {
        Self {
            x1: self.x1,
            y1: self.y1,
            x2: self.x2,
            y2: self.y2,
        }
    }
}

impl Clone for Spring {
    fn clone(&self) -> Self {
        Self {
            stiffness: self.stiffness,
            damping: self.damping,
            mass: self.mass,
            velocity: self.velocity,
            current: self.current,
        }
    }
}
