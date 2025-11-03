use crate::cubic::CubicBezier;
use crate::spring::Spring;
use crate::types::*;
use js_sys;
use serde_wasm_bindgen::from_value;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{window, Element, HtmlElement, Performance};

mod cubic;
mod spring;
mod types;

pub use cubic::CubicBezier as CubicBezierCurve;
pub use spring::Spring as SpringPhysics;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub enum AnimationState {
    Idle,
    Running,
    Paused,
    Completed,
}

#[wasm_bindgen]
pub struct Animation {
    element: Element,
    properties: Vec<AnimationProperty>,
    springs: Vec<Spring>,
    keyframes: Vec<Keyframe>,
    bezier: Option<CubicBezier>,
    duration: f64,
    delay: f64,
    start_time: f64,
    last_time: f64,
    pause_time: f64,
    performance: Performance,
    use_spring: bool,
    use_keyframes: bool,
    state: AnimationState,
    fraction_complete: f64,
    completion_callback: Option<js_sys::Function>,
    gesture_velocity: Vec<(PropertyType, f64)>,
    is_additive: bool,
}

#[wasm_bindgen]
pub struct AnimationHandle {
    animation: Rc<RefCell<Animation>>,
}

#[wasm_bindgen]
impl AnimationHandle {
    #[wasm_bindgen]
    pub fn pause(&self) -> Result<(), JsValue> {
        self.animation.borrow_mut().pause()
    }

    #[wasm_bindgen]
    pub fn resume(&self) -> Result<(), JsValue> {
        self.animation.borrow_mut().resume()
    }

    #[wasm_bindgen]
    pub fn stop(&self) -> Result<(), JsValue> {
        self.animation.borrow_mut().stop()
    }

    #[wasm_bindgen]
    pub fn reverse(&self) -> Result<(), JsValue> {
        self.animation.borrow_mut().reverse()
    }

    #[wasm_bindgen]
    pub fn set_fraction_complete(&self, fraction: f64) -> Result<(), JsValue> {
        self.animation.borrow_mut().set_fraction_complete(fraction)
    }

    #[wasm_bindgen]
    pub fn get_fraction_complete(&self) -> f64 {
        self.animation.borrow().get_fraction_complete()
    }

    #[wasm_bindgen]
    pub fn get_state(&self) -> AnimationState {
        self.animation.borrow().get_state()
    }
}

#[wasm_bindgen]
impl Animation {
    #[wasm_bindgen(constructor)]
    pub fn new(element: Element) -> Result<Animation, JsValue> {
        let window = window().ok_or("No window")?;
        let performance = window.performance().ok_or("No performance")?;

        Ok(Animation {
            element,
            properties: Vec::new(),
            springs: Vec::new(),
            keyframes: Vec::new(),
            bezier: Some(CubicBezier::smooth()),
            duration: 400.0,
            delay: 0.0,
            start_time: 0.0,
            last_time: 0.0,
            pause_time: 0.0,
            performance,
            use_spring: false,
            use_keyframes: false,
            state: AnimationState::Idle,
            fraction_complete: 0.0,
            completion_callback: None,
            gesture_velocity: Vec::new(),
            is_additive: false,
        })
    }

    // ========================================================================
    // TIMING CURVES
    // ========================================================================

    #[wasm_bindgen]
    pub fn cubic(mut self, x1: f64, y1: f64, x2: f64, y2: f64, duration: f64) -> Self {
        self.bezier = Some(CubicBezier::new(x1, y1, x2, y2));
        self.duration = duration;
        self.use_spring = false;
        self
    }

    #[wasm_bindgen]
    pub fn smooth(mut self, duration: f64) -> Self {
        self.bezier = Some(CubicBezier::smooth());
        self.duration = duration;
        self.use_spring = false;
        self
    }

    #[wasm_bindgen]
    pub fn snappy(mut self, duration: f64) -> Self {
        self.bezier = Some(CubicBezier::snappy());
        self.duration = duration;
        self.use_spring = false;
        self
    }

    #[wasm_bindgen]
    pub fn bounce(mut self, duration: f64) -> Self {
        self.bezier = Some(CubicBezier::bounce());
        self.duration = duration;
        self.use_spring = false;
        self
    }

    #[wasm_bindgen]
    pub fn ease_out(mut self, duration: f64) -> Self {
        self.bezier = Some(CubicBezier::ease_out());
        self.duration = duration;
        self.use_spring = false;
        self
    }

    #[wasm_bindgen]
    pub fn ease_in(mut self, duration: f64) -> Self {
        self.bezier = Some(CubicBezier::ease_in());
        self.duration = duration;
        self.use_spring = false;
        self
    }

    #[wasm_bindgen]
    pub fn ease_in_out(mut self, duration: f64) -> Self {
        self.bezier = Some(CubicBezier::ease_in_out());
        self.duration = duration;
        self.use_spring = false;
        self
    }

    #[wasm_bindgen]
    pub fn linear(mut self, duration: f64) -> Self {
        self.bezier = Some(CubicBezier::linear());
        self.duration = duration;
        self.use_spring = false;
        self
    }

    // ========================================================================
    // SPRING PHYSICS
    // ========================================================================

    #[wasm_bindgen]
    pub fn spring(mut self, stiffness: f64, damping: f64) -> Self {
        self.springs = self.properties
            .iter()
            .map(|_| Spring::new(stiffness, damping))
            .collect();
        self.use_spring = true;
        self
    }

    #[wasm_bindgen]
    pub fn spring_default(mut self) -> Self {
        self.springs = (0..self.properties.len())
            .map(|_| Spring::default())
            .collect();
        self.use_spring = true;
        self
    }

    #[wasm_bindgen]
    pub fn spring_bouncy(mut self) -> Self {
        self.springs = (0..self.properties.len())
            .map(|_| Spring::bouncy())
            .collect();
        self.use_spring = true;
        self
    }

    #[wasm_bindgen]
    pub fn spring_smooth(mut self) -> Self {
        self.springs = (0..self.properties.len())
            .map(|_| Spring::smooth())
            .collect();
        self.use_spring = true;
        self
    }

    // ========================================================================
    // CONFIGURATION
    // ========================================================================

    #[wasm_bindgen]
    pub fn animate(mut self, config: JsValue) -> Result<Animation, JsValue> {
        let cfg: AnimateConfig = from_value(config)
            .map_err(|e| JsValue::from_str(&format!("Invalid config: {:?}", e)))?;

        self.setup_properties(&cfg)?;
        Ok(self)
    }

    #[wasm_bindgen]
    pub fn set_delay(mut self, delay: f64) -> Self {
        self.delay = delay;
        self
    }

    #[wasm_bindgen]
    pub fn additive(mut self) -> Self {
        self.is_additive = true;
        self
    }

    #[wasm_bindgen]
    pub fn on_complete(mut self, callback: js_sys::Function) -> Self {
        self.completion_callback = Some(callback);
        self
    }

    #[wasm_bindgen]
    pub fn with_velocity(mut self, property: String, velocity: f64) -> Self {
        if let Some(prop_type) = PropertyType::from_str(&property) {
            self.gesture_velocity.push((prop_type, velocity));
        }
        self
    }

    // ========================================================================
    // KEYFRAMES
    // ========================================================================

    #[wasm_bindgen]
    pub fn add_keyframe(mut self, config: JsValue) -> Result<Animation, JsValue> {
        let kf: KeyframeConfig = from_value(config)
            .map_err(|e| JsValue::from_str(&format!("Invalid keyframe: {:?}", e)))?;

        let mut props = Vec::new();

        if let Some(x) = kf.x {
            props.push((PropertyType::X, AnimatableValue::Number(x)));
        }
        if let Some(y) = kf.y {
            props.push((PropertyType::Y, AnimatableValue::Number(y)));
        }
        if let Some(scale) = kf.scale {
            props.push((PropertyType::Scale, AnimatableValue::Number(scale)));
        }
        if let Some(opacity) = kf.opacity {
            props.push((PropertyType::Opacity, AnimatableValue::Number(opacity)));
        }
        if let Some(rotate) = kf.rotate {
            props.push((PropertyType::Rotate, AnimatableValue::Number(rotate)));
        }

        self.keyframes.push(Keyframe {
            time: kf.time.clamp(0.0, 1.0),
            properties: props,
        });

        self.use_keyframes = true;
        Ok(self)
    }

    #[wasm_bindgen]
    pub fn add_keyframes(mut self, keyframes_config: JsValue) -> Result<Animation, JsValue> {
        let keyframes_list: Vec<KeyframeConfig> = from_value(keyframes_config)
            .map_err(|e| JsValue::from_str(&format!("Invalid keyframes: {:?}", e)))?;

        for kf in keyframes_list {
            let mut props = Vec::new();

            if let Some(x) = kf.x {
                props.push((PropertyType::X, AnimatableValue::Number(x)));
            }
            if let Some(y) = kf.y {
                props.push((PropertyType::Y, AnimatableValue::Number(y)));
            }
            if let Some(scale) = kf.scale {
                props.push((PropertyType::Scale, AnimatableValue::Number(scale)));
            }
            if let Some(opacity) = kf.opacity {
                props.push((PropertyType::Opacity, AnimatableValue::Number(opacity)));
            }
            if let Some(rotate) = kf.rotate {
                props.push((PropertyType::Rotate, AnimatableValue::Number(rotate)));
            }

            self.keyframes.push(Keyframe {
                time: kf.time.clamp(0.0, 1.0),
                properties: props,
            });
        }

        self.use_keyframes = true;
        Ok(self)
    }

    // ========================================================================
    // PLAYBACK CONTROL
    // ========================================================================

    #[wasm_bindgen]
    pub fn start(mut self) -> Result<AnimationHandle, JsValue> {
        if self.state == AnimationState::Running {
            return Err(JsValue::from_str("Animation already running"));
        }

        self.capture_start_values()?;

        let now = self.performance.now();
        self.start_time = now + self.delay;
        self.last_time = now;
        self.state = AnimationState::Running;
        self.fraction_complete = 0.0;

        let animation = Rc::new(RefCell::new(self));
        spawn_animation_loop(animation.clone())?;

        Ok(AnimationHandle { animation })
    }

    #[wasm_bindgen]
    pub fn pause(&mut self) -> Result<(), JsValue> {
        if self.state == AnimationState::Running {
            self.state = AnimationState::Paused;
            self.pause_time = self.performance.now();
        }
        Ok(())
    }

    #[wasm_bindgen]
    pub fn resume(&mut self) -> Result<(), JsValue> {
        if self.state == AnimationState::Paused {
            let pause_duration = self.performance.now() - self.pause_time;
            self.start_time += pause_duration;
            self.state = AnimationState::Running;
            self.animate_frame()?;
        }
        Ok(())
    }

    #[wasm_bindgen]
    pub fn stop(&mut self) -> Result<(), JsValue> {
        self.state = AnimationState::Completed;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn reverse(&mut self) -> Result<(), JsValue> {
        for prop in self.properties.iter_mut() {
            std::mem::swap(&mut prop.start, &mut prop.end);
        }

        self.start_time = self.performance.now();
        self.fraction_complete = 0.0;
        self.state = AnimationState::Running;
        self.animate_frame()?;
        Ok(())
    }

    // ========================================================================
    // SCRUBBING
    // ========================================================================

    #[wasm_bindgen]
    pub fn set_fraction_complete(&mut self, fraction: f64) -> Result<(), JsValue> {
        self.fraction_complete = fraction.clamp(0.0, 1.0);

        let eased = if let Some(bezier) = &self.bezier {
            bezier.solve(self.fraction_complete)
        } else {
            self.fraction_complete
        };

        if self.use_keyframes {
            self.update_keyframes(self.fraction_complete)?;
        } else {
            for prop in self.properties.iter_mut() {
                let start = prop.start.clone();
                let end = prop.end.clone();
                prop.current = interpolate_value(&start, &end, eased);
            }
        }

        self.apply_properties()?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn get_fraction_complete(&self) -> f64 {
        self.fraction_complete
    }

    #[wasm_bindgen]
    pub fn get_state(&self) -> AnimationState {
        self.state
    }

    // ========================================================================
    // INTERNAL METHODS
    // ========================================================================

    fn setup_properties(&mut self, cfg: &AnimateConfig) -> Result<(), JsValue> {
        if let Some(x) = cfg.x {
            self.add_number_property(PropertyType::X, x);
        }
        if let Some(y) = cfg.y {
            self.add_number_property(PropertyType::Y, y);
        }
        if let Some(z) = cfg.z {
            self.add_number_property(PropertyType::Z, z);
        }
        if let Some(scale) = cfg.scale {
            self.add_number_property(PropertyType::Scale, scale);
        }
        if let Some(rotate) = cfg.rotate {
            self.add_number_property(PropertyType::Rotate, rotate);
        }
        if let Some(opacity) = cfg.opacity {
            self.add_number_property(PropertyType::Opacity, opacity);
        }
        if let Some(ref width) = cfg.width {
            self.parse_and_add_length(PropertyType::Width, width)?;
        }
        if let Some(ref height) = cfg.height {
            self.parse_and_add_length(PropertyType::Height, height)?;
        }
        if let Some(ref bg) = cfg.background_color {
            self.parse_and_add_color(PropertyType::BackgroundColor, bg)?;
        }
        if let Some(offset) = cfg.stroke_dashoffset {
            self.add_number_property(PropertyType::StrokeDashOffset, offset);
        }
        if let Some(width) = cfg.stroke_width {
            self.add_number_property(PropertyType::StrokeWidth, width);
        }

        Ok(())
    }

    fn add_number_property(&mut self, prop_type: PropertyType, end_value: f64) {
        self.properties.push(AnimationProperty {
            property_type: prop_type,
            start: AnimatableValue::Number(0.0),
            end: AnimatableValue::Number(end_value),
            current: AnimatableValue::Number(0.0),
        });
    }

    fn add_length_property(&mut self, prop_type: PropertyType, value: f64, unit: LengthUnit) {
        self.properties.push(AnimationProperty {
            property_type: prop_type,
            start: AnimatableValue::Length(0.0, unit.clone()),
            end: AnimatableValue::Length(value, unit.clone()),
            current: AnimatableValue::Length(0.0, unit),
        });
    }

    fn parse_and_add_length(&mut self, prop_type: PropertyType, value: &str) -> Result<(), JsValue> {
        let (num, unit) = parse_css_length(value).map_err(|e| JsValue::from_str(&e))?;
        self.add_length_property(prop_type, num, unit);
        Ok(())
    }

    fn parse_and_add_color(&mut self, prop_type: PropertyType, value: &str) -> Result<(), JsValue> {
        let (r, g, b, a) = parse_css_color(value).map_err(|e| JsValue::from_str(&e))?;
        self.properties.push(AnimationProperty {
            property_type: prop_type,
            start: AnimatableValue::Color(0.0, 0.0, 0.0, 1.0),
            end: AnimatableValue::Color(r, g, b, a),
            current: AnimatableValue::Color(0.0, 0.0, 0.0, 1.0),
        });
        Ok(())
    }

    fn capture_start_values(&mut self) -> Result<(), JsValue> {
        for prop in self.properties.iter_mut() {
            prop.current = prop.start.clone();
        }

        if self.use_spring && !self.properties.is_empty() {
            self.springs = self.properties
                .iter()
                .enumerate()
                .map(|(_i, prop)| {
                    let mut spring = Spring::default();
                    
                    if let Some(&(_, velocity)) = self.gesture_velocity
                        .iter()
                        .find(|(p_type, _)| *p_type == prop.property_type)
                    {
                        spring.velocity = velocity;
                    }
                    
                    spring.reset(extract_number(&prop.start));
                    spring
                })
                .collect();
        }

        Ok(())
    }

    fn animate_frame(&mut self) -> Result<(), JsValue> {
        if self.state != AnimationState::Running {
            return Ok(());
        }

        let now = self.performance.now();

        if now < self.start_time {
            return Ok(());
        }

        let delta = (now - self.last_time).min(32.0);
        self.last_time = now;

        let should_continue = if self.use_spring {
            self.update_spring(delta / 1000.0)?
        } else if self.use_keyframes {
            self.update_keyframes_time(now)?
        } else {
            self.update_cubic(now)?
        };

        self.apply_properties()?;

        if !should_continue {
            self.state = AnimationState::Completed;

            if let Some(ref callback) = self.completion_callback {
                let _ = callback.call0(&JsValue::NULL);
            }
        }

        Ok(())
    }

    fn update_cubic(&mut self, now: f64) -> Result<bool, JsValue> {
        let elapsed = now - self.start_time;
        let progress = (elapsed / self.duration).min(1.0);
        self.fraction_complete = progress;

        let eased = if let Some(bezier) = &self.bezier {
            bezier.solve(progress)
        } else {
            progress
        };

        for prop in self.properties.iter_mut() {
            let start = prop.start.clone();
            let end = prop.end.clone();
            prop.current = interpolate_value(&start, &end, eased);
        }

        Ok(progress < 1.0)
    }

    fn update_spring(&mut self, delta_time: f64) -> Result<bool, JsValue> {
        let mut at_rest = true;

        for (prop, spring) in self.properties.iter_mut().zip(self.springs.iter_mut()) {
            let target = extract_number(&prop.end);
            let value = spring.update(target, delta_time);

            if spring.velocity.abs() > 0.01 || (value - target).abs() > 0.01 {
                at_rest = false;
            }

            prop.current = create_value_with_number(&prop.end, value);
        }

        Ok(!at_rest)
    }

    fn update_keyframes_time(&mut self, now: f64) -> Result<bool, JsValue> {
        let elapsed = now - self.start_time;
        let progress = (elapsed / self.duration).min(1.0);
        self.fraction_complete = progress;

        self.update_keyframes(progress)?;
        Ok(progress < 1.0)
    }

    fn update_keyframes(&mut self, progress: f64) -> Result<(), JsValue> {
        if self.keyframes.is_empty() {
            return Ok(());
        }

        let mut sorted_kf = self.keyframes.clone();
        sorted_kf.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

        let mut start_kf = &sorted_kf[0];
        let mut end_kf = &sorted_kf[sorted_kf.len() - 1];
        let mut local_progress = 0.0;

        for i in 0..sorted_kf.len() - 1 {
            if progress >= sorted_kf[i].time && progress <= sorted_kf[i + 1].time {
                start_kf = &sorted_kf[i];
                end_kf = &sorted_kf[i + 1];
                local_progress = (progress - start_kf.time) / (end_kf.time - start_kf.time);
                break;
            }
        }

        let eased = if let Some(bezier) = &self.bezier {
            bezier.solve(local_progress)
        } else {
            local_progress
        };

        for prop in self.properties.iter_mut() {
            if let (Some(start_val), Some(end_val)) = (
                start_kf.properties.iter().find(|(p, _)| p == &prop.property_type).map(|(_, v)| v),
                end_kf.properties.iter().find(|(p, _)| p == &prop.property_type).map(|(_, v)| v),
            ) {
                prop.current = interpolate_value(start_val, end_val, eased);
            }
        }

        Ok(())
    }

    fn apply_properties(&self) -> Result<(), JsValue> {
        let mut transform_parts = Vec::new();
        let mut has_translate = false;

        for prop in self.properties.iter() {
            match prop.property_type {
                PropertyType::X | PropertyType::Y | PropertyType::Z => {
                    if !has_translate {
                        let x = self.get_number_value(PropertyType::X).round();
                        let y = self.get_number_value(PropertyType::Y).round();
                        let z = self.get_number_value(PropertyType::Z).round();

                        if x != 0.0 || y != 0.0 || z != 0.0 {
                            transform_parts.push(format!("translate3d({}px, {}px, {}px)", x, y, z));
                        }
                        has_translate = true;
                    }
                }
                PropertyType::Scale => {
                    if let AnimatableValue::Number(val) = prop.current {
                        transform_parts.push(format!("scale({})", val));
                    }
                }
                PropertyType::Rotate => {
                    if let AnimatableValue::Number(val) = prop.current {
                        transform_parts.push(format!("rotate({}deg)", val));
                    }
                }
                PropertyType::Opacity => {
                    if let AnimatableValue::Number(val) = prop.current {
                        self.set_style_property("opacity", &val.to_string())?;
                    }
                }
                PropertyType::Width | PropertyType::Height | PropertyType::BackgroundColor |
                PropertyType::StrokeDashOffset | PropertyType::StrokeWidth => {
                    let value = format_value(&prop.current);
                    self.set_style_property(prop.property_type.as_str(), &value)?;
                }
            }
        }

        if !transform_parts.is_empty() {
            self.set_style_property("transform", &transform_parts.join(" "))?;
        }

        Ok(())
    }

    fn get_number_value(&self, prop_type: PropertyType) -> f64 {
        self.properties
            .iter()
            .find(|p| p.property_type == prop_type)
            .and_then(|p| match p.current {
                AnimatableValue::Number(n) => Some(n),
                _ => None,
            })
            .unwrap_or(0.0)
    }

    fn set_style_property(&self, name: &str, value: &str) -> Result<(), JsValue> {
        if let Some(html_el) = self.element.dyn_ref::<HtmlElement>() {
            html_el.style().set_property(name, value)?;
        }
        Ok(())
    }
}

fn spawn_animation_loop(animation: Rc<RefCell<Animation>>) -> Result<(), JsValue> {
    let window = window().ok_or("No window")?;
    let animation_clone = animation.clone();

    let closure = Closure::wrap(Box::new(move || {
        let mut anim = match animation_clone.try_borrow_mut() {
            Ok(a) => a,
            Err(_) => return,
        };

        let should_continue = match anim.animate_frame() {
            Ok(()) => anim.state == AnimationState::Running,
            Err(_) => false,
        };

        if should_continue {
            drop(anim);
            let next_animation = animation_clone.clone();
            let _ = spawn_animation_loop(next_animation);
        }
    }) as Box<dyn FnMut()>);

    window.request_animation_frame(closure.as_ref().unchecked_ref())?;
    closure.forget();
    Ok(())
}