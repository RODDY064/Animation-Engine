use crate::cubic::CubicBezier;
use crate::spring::Spring;
use crate::types::*;
use js_sys::{self, Function};
use serde_wasm_bindgen::from_value;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use web_sys::{window, Element, HtmlElement, Performance, SvgElement};

mod choreographer;
mod cubic;
mod gesture;
mod metal_acceleration;
mod particle_effects;
mod sequencer;
mod shape_morphing;
mod spring;
mod transaction;
mod types;

pub use choreographer::Choreographer;
pub use cubic::CubicBezier as CubicBezierCurve;
pub use gesture::GestureController;
pub use metal_acceleration::GPUAccelerator;
pub use particle_effects::ParticleEmitter;
pub use sequencer::Sequencer;
pub use shape_morphing::PathMorph;
pub use spring::Spring as SpringPhysics;
pub use transaction::AnimationTransaction;

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
    completion_callback: Option<Function>,
    gesture_velocity: Vec<(PropertyType, f64)>,
    is_additive: bool,
    repeat_count: i32,
    current_repeat: i32,
    auto_reverse: bool,
    transform_origin: (String, String, String),
    shadow_layers: Vec<ShadowValue>,
    continue_animate: bool,
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
        let window = window().ok_or_else(|| JsValue::from_str("No window available"))?;
        let performance = window
            .performance()
            .ok_or_else(|| JsValue::from_str("No performance API"))?;

        Ok(Animation {
            element,
            properties: Vec::with_capacity(32),
            springs: Vec::with_capacity(32),
            keyframes: Vec::with_capacity(16),
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
            repeat_count: 1,
            current_repeat: 0,
            auto_reverse: false,
            transform_origin: ("50%".to_string(), "50%".to_string(), "0".to_string()),
            shadow_layers: Vec::new(),
            continue_animate: false,
        })
    }

    // ========================================================================
    // TIMING CURVES
    // ========================================================================

    pub fn create_gesture_controller(&self) -> GestureController {
        GestureController::new()
    }

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
    pub fn spring(mut self, _stiffness: f64, _damping: f64) -> Self {
        self.use_spring = true;
        self
    }

    #[wasm_bindgen]
    pub fn spring_default(mut self) -> Self {
        self.use_spring = true;
        self
    }

    #[wasm_bindgen]
    pub fn spring_bouncy(mut self) -> Self {
        self.use_spring = true;
        self
    }

    #[wasm_bindgen]
    pub fn spring_smooth(mut self) -> Self {
        self.use_spring = true;
        self
    }

    // ========================================================================
    // ANIMATION OPTIONS
    // ========================================================================

    #[wasm_bindgen]
    pub fn repeat(mut self, count: i32) -> Self {
        self.repeat_count = count;
        self
    }

    #[wasm_bindgen]
    pub fn auto_reverse(mut self) -> Self {
        self.auto_reverse = true;
        self
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
    pub fn continue_animate(mut self) -> Self {
        self.continue_animate = true;
        self
    }

    #[wasm_bindgen]
    pub fn on_complete(mut self, callback: Function) -> Self {
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

    #[wasm_bindgen]
    pub fn set_transform_origin(mut self, x: String, y: String, z: String) -> Self {
        self.transform_origin = (x, y, z);
        self
    }

    #[wasm_bindgen]
    pub fn add_shadow_layer(
        mut self,
        offset_x: f64,
        offset_y: f64,
        blur: f64,
        spread: f64,
        color: String,
        inset: bool,
    ) -> Result<Animation, JsValue> {
        let (r, g, b, a) = parse_css_color(&color)?;
        self.shadow_layers.push(ShadowValue {
            offset_x,
            offset_y,
            blur,
            spread,
            color: (r, g, b, a),
            inset,
        });
        Ok(self)
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

    // ========================================================================
    // KEYFRAMES
    // ========================================================================

    #[wasm_bindgen]
    pub fn add_keyframe(mut self, config: JsValue) -> Result<Animation, JsValue> {
        let kf: KeyframeConfig = from_value(config)
            .map_err(|e| JsValue::from_str(&format!("Invalid keyframe: {:?}", e)))?;

        self.push_keyframe(kf)?;
        self.use_keyframes = true;
        Ok(self)
    }

    #[wasm_bindgen]
    pub fn add_keyframes(mut self, configs: JsValue) -> Result<Animation, JsValue> {
        let keyframe_configs: Vec<KeyframeConfig> = from_value(configs)
            .map_err(|e| JsValue::from_str(&format!("Invalid keyframes config: {:?}", e)))?;

        for kf in keyframe_configs {
            self.push_keyframe(kf)?;
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
        self.current_repeat = 0;

        let animation = Rc::new(RefCell::new(self));
        spawn_animation_loop(animation.clone())?;

        Ok(AnimationHandle { animation })
    }

    #[wasm_bindgen]
    pub fn start_internal(&mut self) -> Result<(), JsValue> {
        if self.state == AnimationState::Running {
            return Err(JsValue::from_str("Animation already running"));
        }

        self.capture_start_values()?;

        let now = self.performance.now();
        self.start_time = now + self.delay;
        self.last_time = now;
        self.state = AnimationState::Running;
        self.fraction_complete = 0.0;
        self.current_repeat = 0;

        Ok(())
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
        Ok(())
    }

    // ========================================================================
    // SCRUBBING
    // ========================================================================

    #[wasm_bindgen]
    pub fn set_fraction_complete(&mut self, fraction: f64) -> Result<(), JsValue> {
        self.fraction_complete = fraction.clamp(0.0, 1.0);

        let eased = match &self.bezier {
            Some(bezier) => bezier.solve(self.fraction_complete),
            None => self.fraction_complete,
        };

        if self.use_keyframes {
            self.update_keyframes(self.fraction_complete)?;
        } else {
            for prop in self.properties.iter_mut() {
                prop.current = interpolate_value(&prop.start, &prop.end, eased);
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

    fn push_keyframe(&mut self, kf: KeyframeConfig) -> Result<(), JsValue> {
        let mut props = Vec::with_capacity(20);

        macro_rules! add_number {
            ($opt:expr, $prop_type:expr) => {
                if let Some(val) = $opt {
                    props.push(($prop_type, AnimatableValue::Number(val)));
                }
            };
        }

        macro_rules! add_length {
            ($opt:expr, $prop_type:expr) => {
                if let Some(ref val) = $opt {
                    let (num, unit) = parse_css_length(val)?;
                    props.push(($prop_type, AnimatableValue::Length(num, unit)));
                }
            };
        }

        // Transform - Numbers
        add_number!(kf.x, PropertyType::X);
        add_number!(kf.y, PropertyType::Y);
        add_number!(kf.z, PropertyType::Z);
        add_number!(kf.scale, PropertyType::Scale);
        add_number!(kf.scale_x, PropertyType::ScaleX);
        add_number!(kf.scale_y, PropertyType::ScaleY);
        add_number!(kf.opacity, PropertyType::Opacity);
        add_number!(kf.rotate, PropertyType::Rotate);
        add_number!(kf.rotate_x, PropertyType::RotateX);
        add_number!(kf.rotate_y, PropertyType::RotateY);

        // Size - Lengths
        add_length!(kf.width, PropertyType::Width);
        add_length!(kf.height, PropertyType::Height);
        add_length!(kf.border_radius, PropertyType::BorderRadius);

        // Filters - Numbers
        add_number!(kf.blur, PropertyType::Blur);
        add_number!(kf.brightness, PropertyType::Brightness);
        add_number!(kf.contrast, PropertyType::Contrast);

        // Shadows - Numbers
        add_number!(kf.shadow_blur, PropertyType::ShadowBlur);
        add_number!(kf.shadow_offset_x, PropertyType::ShadowOffsetX);
        add_number!(kf.shadow_offset_y, PropertyType::ShadowOffsetY);

        self.keyframes.push(Keyframe {
            time: kf.time.clamp(0.0, 1.0),
            properties: props,
        });

        Ok(())
    }
    fn setup_properties(&mut self, cfg: &AnimateConfig) -> Result<(), JsValue> {
        // Clear properties to start fresh FIRST
        self.properties.clear();

        macro_rules! setup_number {
            ($opt:expr, $prop_type:expr) => {
                if let Some(val) = $opt {
                    self.add_number_property($prop_type, val);
                }
            };
        }

        macro_rules! setup_length {
            ($opt:expr, $prop_type:expr) => {
                if let Some(ref val) = $opt {
                    self.parse_and_add_length($prop_type, val)?;
                }
            };
        }
        macro_rules! setup_color {
            ($opt:expr, $prop_type:expr) => {
                if let Some(ref val) = $opt {
                    self.parse_and_add_color($prop_type, val)?;
                }
            };
        }

        macro_rules! setup_visibility {
            ($opt:expr) => {
                if let Some(ref val) = $opt {
                    let vis_val = crate::types::VisibilityValue::from_str(val);
                    self.properties.push(AnimationProperty {
                        property_type: PropertyType::Visibility,
                        start: AnimatableValue::Visibility(crate::types::VisibilityValue::Visible),
                        end: AnimatableValue::Visibility(vis_val),
                        current: AnimatableValue::Visibility(
                            crate::types::VisibilityValue::Visible,
                        ),
                    });
                }
            };
        }

        // Transform
        setup_number!(cfg.x, PropertyType::X);
        setup_number!(cfg.y, PropertyType::Y);
        setup_number!(cfg.z, PropertyType::Z);
        setup_number!(cfg.scale, PropertyType::Scale);
        setup_number!(cfg.scale_x, PropertyType::ScaleX);
        setup_number!(cfg.scale_y, PropertyType::ScaleY);
        setup_number!(cfg.rotate, PropertyType::Rotate);
        setup_number!(cfg.rotate_x, PropertyType::RotateX);
        setup_number!(cfg.rotate_y, PropertyType::RotateY);
        setup_number!(cfg.rotate_z, PropertyType::RotateZ);
        setup_number!(cfg.skew_x, PropertyType::SkewX);
        setup_number!(cfg.skew_y, PropertyType::SkewY);

        // Size
        setup_length!(cfg.width, PropertyType::Width);
        setup_length!(cfg.height, PropertyType::Height);
        setup_length!(cfg.min_width, PropertyType::MinWidth);
        setup_length!(cfg.min_height, PropertyType::MinHeight);
        setup_length!(cfg.max_width, PropertyType::MaxWidth);
        setup_length!(cfg.max_height, PropertyType::MaxHeight);

        // Visual
        setup_number!(cfg.opacity, PropertyType::Opacity);
        setup_visibility!(cfg.visibility);
        setup_color!(cfg.background_color, PropertyType::BackgroundColor);
        setup_color!(cfg.color, PropertyType::Color);
        setup_color!(cfg.border_color, PropertyType::BorderColor);
        setup_length!(cfg.border_radius, PropertyType::BorderRadius);
        setup_length!(cfg.border_width, PropertyType::BorderWidth);

        // Shadows
        setup_number!(cfg.shadow_offset_x, PropertyType::ShadowOffsetX);
        setup_number!(cfg.shadow_offset_y, PropertyType::ShadowOffsetY);
        setup_number!(cfg.shadow_blur, PropertyType::ShadowBlur);
        setup_number!(cfg.shadow_spread, PropertyType::ShadowSpread);
        setup_color!(cfg.shadow_color, PropertyType::ShadowColor);

        // Filters
        setup_number!(cfg.blur, PropertyType::Blur);
        setup_number!(cfg.brightness, PropertyType::Brightness);
        setup_number!(cfg.contrast, PropertyType::Contrast);
        setup_number!(cfg.saturate, PropertyType::Saturate);
        setup_number!(cfg.hue, PropertyType::Hue);
        setup_number!(cfg.grayscale, PropertyType::Grayscale);
        setup_number!(cfg.invert, PropertyType::Invert);
        setup_number!(cfg.sepia, PropertyType::Sepia);

        // SVG
        setup_number!(cfg.stroke_dashoffset, PropertyType::StrokeDashOffset);
        setup_number!(cfg.stroke_width, PropertyType::StrokeWidth);
        setup_number!(cfg.fill_opacity, PropertyType::FillOpacity);
        setup_number!(cfg.stroke_opacity, PropertyType::StrokeOpacity);

        // Advanced
        setup_length!(cfg.transform_origin_x, PropertyType::TransformOriginX);
        setup_length!(cfg.transform_origin_y, PropertyType::TransformOriginY);
        setup_length!(cfg.transform_origin_z, PropertyType::TransformOriginZ);
        setup_number!(cfg.perspective, PropertyType::Perspective);
        setup_length!(cfg.perspective_origin_x, PropertyType::PerspectiveOriginX);
        setup_length!(cfg.perspective_origin_y, PropertyType::PerspectiveOriginY);

        // ✨ If continue_animate, read stored values and add as frozen properties
        if self.continue_animate {
            if let Ok(html_elem) = self.element.clone().dyn_into::<HtmlElement>() {
                let get_attr = |name: &str| -> Option<String> { html_elem.get_attribute(name) };

                // Read stored X
                if cfg.x.is_none() {
                    if let Some(x_str) = get_attr("data-anim-x") {
                        if let Ok(x_val) = x_str.parse::<f64>() {
                            if x_val != 0.0 {
                                self.properties.push(AnimationProperty {
                                    property_type: PropertyType::X,
                                    start: AnimatableValue::Number(x_val),
                                    end: AnimatableValue::Number(x_val),
                                    current: AnimatableValue::Number(x_val),
                                });
                            }
                        }
                    }
                }

                // Read stored Y
                if cfg.y.is_none() {
                    if let Some(y_str) = get_attr("data-anim-y") {
                        if let Ok(y_val) = y_str.parse::<f64>() {
                            if y_val != 0.0 {
                                self.properties.push(AnimationProperty {
                                    property_type: PropertyType::Y,
                                    start: AnimatableValue::Number(y_val),
                                    end: AnimatableValue::Number(y_val),
                                    current: AnimatableValue::Number(y_val),
                                });
                            }
                        }
                    }
                }

                // Read stored Z
                if cfg.z.is_none() {
                    if let Some(z_str) = get_attr("data-anim-z") {
                        if let Ok(z_val) = z_str.parse::<f64>() {
                            if z_val != 0.0 {
                                self.properties.push(AnimationProperty {
                                    property_type: PropertyType::Z,
                                    start: AnimatableValue::Number(z_val),
                                    end: AnimatableValue::Number(z_val),
                                    current: AnimatableValue::Number(z_val),
                                });
                            }
                        }
                    }
                }

                // Read stored Scale
                if cfg.scale.is_none() {
                    if let Some(scale_str) = get_attr("data-anim-scale") {
                        if let Ok(scale_val) = scale_str.parse::<f64>() {
                            if scale_val != 1.0 {
                                self.properties.push(AnimationProperty {
                                    property_type: PropertyType::Scale,
                                    start: AnimatableValue::Number(scale_val),
                                    end: AnimatableValue::Number(scale_val),
                                    current: AnimatableValue::Number(scale_val),
                                });
                            }
                        }
                    }
                }

                // Read stored Opacity
                if cfg.opacity.is_none() {
                    if let Some(opacity_str) = get_attr("data-anim-opacity") {
                        if let Ok(opacity_val) = opacity_str.parse::<f64>() {
                            if opacity_val != 1.0 {
                                self.properties.push(AnimationProperty {
                                    property_type: PropertyType::Opacity,
                                    start: AnimatableValue::Number(opacity_val),
                                    end: AnimatableValue::Number(opacity_val),
                                    current: AnimatableValue::Number(opacity_val),
                                });
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    #[inline]
    fn add_number_property(&mut self, prop_type: PropertyType, end_value: f64) {
        let start_value = if self.continue_animate {
            self.get_current_number_value(prop_type)
        } else {
            0.0
        };

        self.properties.push(AnimationProperty {
            property_type: prop_type,
            start: AnimatableValue::Number(start_value),
            end: AnimatableValue::Number(end_value),
            current: AnimatableValue::Number(start_value),
        });
    }

    #[inline]
    fn add_length_property(&mut self, prop_type: PropertyType, value: f64, unit: LengthUnit) {
        let start_value = self.get_current_length_value(prop_type);

        self.properties.push(AnimationProperty {
            property_type: prop_type,
            start: AnimatableValue::Length(start_value, unit.clone()),
            end: AnimatableValue::Length(value, unit.clone()),
            current: AnimatableValue::Length(start_value, unit),
        });
    }

    fn get_current_length_value(&self, prop_type: PropertyType) -> f64 {
        if let Ok(html_elem) = self.element.clone().dyn_into::<HtmlElement>() {
            let property_name = match prop_type {
                PropertyType::Width => "width",
                PropertyType::Height => "height",
                PropertyType::MinWidth => "min-width",
                PropertyType::MinHeight => "min-height",
                PropertyType::MaxWidth => "max-width",
                PropertyType::MaxHeight => "max-height",
                PropertyType::BorderRadius => "border-radius",
                PropertyType::BorderWidth => "border-width",
                _ => return 0.0,
            };

            // Try computed style first
            if let Some(window) = window() {
                if let Ok(Some(computed)) = window.get_computed_style(&html_elem) {
                    if let Ok(value) = computed.get_property_value(property_name) {
                        if !value.is_empty() && value != "auto" {
                            if let Ok((num, _)) = parse_css_length(&value) {
                                return num;
                            }
                        }
                    }
                }
            }

            // Fallback to inline style
            if let Ok(value) = html_elem.style().get_property_value(property_name) {
                if !value.is_empty() && value != "auto" {
                    if let Ok((num, _)) = parse_css_length(&value) {
                        return num;
                    }
                }
            }
        }

        0.0
    }

    #[inline]
    fn get_current_number_value(&self, prop_type: PropertyType) -> f64 {
        if let Ok(html_elem) = self.element.clone().dyn_into::<HtmlElement>() {
            let transform_str = html_elem
                .style()
                .get_property_value("transform")
                .unwrap_or_default();

            // Parse transform string to extract current values
            match prop_type {
                PropertyType::X | PropertyType::Y | PropertyType::Z => {
                    // Extract from translate3d
                    if let Some(start) = transform_str.find("translate3d(") {
                        if let Some(end) = transform_str[start..].find(")") {
                            let values_str = &transform_str[start + 12..start + end];
                            let parts: Vec<&str> = values_str.split(',').collect();

                            if parts.len() >= 3 {
                                return match prop_type {
                                    PropertyType::X => parts[0]
                                        .trim()
                                        .trim_end_matches("px")
                                        .parse()
                                        .unwrap_or(0.0),
                                    PropertyType::Y => parts[1]
                                        .trim()
                                        .trim_end_matches("px")
                                        .parse()
                                        .unwrap_or(0.0),
                                    PropertyType::Z => parts[2]
                                        .trim()
                                        .trim_end_matches("px")
                                        .parse()
                                        .unwrap_or(0.0),
                                    _ => 0.0,
                                };
                            }
                        }
                    }
                    0.0
                }
                PropertyType::Scale => {
                    if let Some(start) = transform_str.find("scale(") {
                        if let Some(end) = transform_str[start..].find(")") {
                            let val_str = &transform_str[start + 6..start + end];
                            return val_str.trim().parse().unwrap_or(1.0);
                        }
                    }
                    1.0
                }
                PropertyType::ScaleX => {
                    if let Some(start) = transform_str.find("scaleX(") {
                        if let Some(end) = transform_str[start..].find(")") {
                            let val_str = &transform_str[start + 7..start + end];
                            return val_str.trim().parse().unwrap_or(1.0);
                        }
                    }
                    1.0
                }
                PropertyType::ScaleY => {
                    if let Some(start) = transform_str.find("scaleY(") {
                        if let Some(end) = transform_str[start..].find(")") {
                            let val_str = &transform_str[start + 7..start + end];
                            return val_str.trim().parse().unwrap_or(1.0);
                        }
                    }
                    1.0
                }
                PropertyType::Opacity => {
                    if let Ok(opacity_str) = html_elem.style().get_property_value("opacity") {
                        return opacity_str.trim().parse().unwrap_or(1.0);
                    }
                    1.0
                }
                PropertyType::Rotate => {
                    if let Some(start) = transform_str.find("rotate(") {
                        if let Some(end) = transform_str[start..].find("deg") {
                            let val_str = &transform_str[start + 7..start + end];
                            return val_str.trim().parse().unwrap_or(0.0);
                        }
                    }
                    0.0
                }
                _ => 0.0,
            }
        } else {
            0.0
        }
    }

    #[inline]
    fn parse_and_add_length(
        &mut self,
        prop_type: PropertyType,
        value: &str,
    ) -> Result<(), JsValue> {
        let (num, unit) = parse_css_length(value)?;
        self.add_length_property(prop_type, num, unit);
        Ok(())
    }

    #[inline]
    fn parse_and_add_color(&mut self, prop_type: PropertyType, value: &str) -> Result<(), JsValue> {
        let (r, g, b, a) = parse_css_color(value).map_err(|e| JsValue::from_str(&e))?;

        // Capture current color from element
        let (start_r, start_g, start_b, start_a) = self.get_current_color_value(prop_type);

        self.properties.push(AnimationProperty {
            property_type: prop_type,
            start: AnimatableValue::Color(start_r, start_g, start_b, start_a),
            end: AnimatableValue::Color(r, g, b, a),
            current: AnimatableValue::Color(start_r, start_g, start_b, start_a),
        });
        Ok(())
    }

    fn get_current_color_value(&self, prop_type: PropertyType) -> (f64, f64, f64, f64) {
        if let Ok(html_elem) = self.element.clone().dyn_into::<HtmlElement>() {
            let property_name = match prop_type {
                PropertyType::BackgroundColor => "background-color",
                PropertyType::Color => "color",
                PropertyType::BorderColor => "border-color",
                _ => return (0.0, 0.0, 0.0, 1.0),
            };

            // Try computed style first (most reliable)
            if let Some(window) = window() {
                if let Ok(Some(computed)) = window.get_computed_style(&html_elem) {
                    if let Ok(value) = computed.get_property_value(property_name) {
                        if !value.is_empty() {
                            if let Ok(color) = parse_css_color(&value) {
                                return color;
                            }
                        }
                    }
                }
            }

            // Fallback to inline style
            if let Ok(value) = html_elem.style().get_property_value(property_name) {
                if !value.is_empty() {
                    if let Ok(color) = parse_css_color(&value) {
                        return color;
                    }
                }
            }
        }

        //  defaults
        match prop_type {
            PropertyType::BackgroundColor => (0.0, 0.0, 0.0, 0.0), // transparent
            PropertyType::Color => (0.0, 0.0, 0.0, 1.0),           // black text
            PropertyType::BorderColor => (0.0, 0.0, 0.0, 1.0),     // black border
            _ => (0.0, 0.0, 0.0, 1.0),
        }
    }

    fn capture_start_values(&mut self) -> Result<(), JsValue> {
        for prop in self.properties.iter_mut() {
            prop.current = prop.start.clone();
        }

        if self.use_spring && !self.properties.is_empty() {
            self.springs = self
                .properties
                .iter()
                .map(|prop| {
                    let mut spring = Spring::default();

                    if let Some(&(_, velocity)) = self
                        .gesture_velocity
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
            self.handle_completion()?;
        }

        Ok(())
    }

    fn handle_completion(&mut self) -> Result<(), JsValue> {
        // ✨ Store final values on the element as data attributes
        if let Ok(html_elem) = self.element.clone().dyn_into::<HtmlElement>() {
            for prop in &self.properties {
                match prop.property_type {
                    PropertyType::X => {
                        if let AnimatableValue::Number(val) = prop.current {
                            let _ = html_elem.set_attribute("data-anim-x", &val.to_string());
                        }
                    }
                    PropertyType::Y => {
                        if let AnimatableValue::Number(val) = prop.current {
                            let _ = html_elem.set_attribute("data-anim-y", &val.to_string());
                        }
                    }
                    PropertyType::Z => {
                        if let AnimatableValue::Number(val) = prop.current {
                            let _ = html_elem.set_attribute("data-anim-z", &val.to_string());
                        }
                    }
                    PropertyType::Scale => {
                        if let AnimatableValue::Number(val) = prop.current {
                            let _ = html_elem.set_attribute("data-anim-scale", &val.to_string());
                        }
                    }
                    PropertyType::Opacity => {
                        if let AnimatableValue::Number(val) = prop.current {
                            let _ = html_elem.set_attribute("data-anim-opacity", &val.to_string());
                        }
                    }
                    _ => {}
                }
            }
        }

        self.current_repeat += 1;

        if self.repeat_count < 0 || self.current_repeat < self.repeat_count {
            if self.auto_reverse {
                self.reverse()?;
            } else {
                self.start_time = self.performance.now();
                self.fraction_complete = 0.0;
            }
        } else {
            self.state = AnimationState::Completed;

            if let Some(ref callback) = self.completion_callback {
                let _ = callback.call0(&JsValue::NULL);
            }
        }

        Ok(())
    }

    #[inline]
    fn update_cubic(&mut self, now: f64) -> Result<bool, JsValue> {
        let elapsed = now - self.start_time;
        let progress = (elapsed / self.duration).min(1.0);
        self.fraction_complete = progress;

        let eased = match &self.bezier {
            Some(bezier) => bezier.solve(progress),
            None => progress,
        };

        for prop in self.properties.iter_mut() {
            prop.current = interpolate_value(&prop.start, &prop.end, eased);
        }

        Ok(progress < 1.0)
    }

    #[inline]
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

    #[inline]
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
        sorted_kf.sort_by(|a, b| {
            a.time
                .partial_cmp(&b.time)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let (start_kf, end_kf, local_progress) = self.find_keyframe_range(&sorted_kf, progress);

        let eased = match &self.bezier {
            Some(bezier) => bezier.solve(local_progress),
            None => local_progress,
        };

        for prop in self.properties.iter_mut() {
            if let (Some(start_val), Some(end_val)) = (
                start_kf
                    .properties
                    .iter()
                    .find(|(p, _)| p == &prop.property_type)
                    .map(|(_, v)| v),
                end_kf
                    .properties
                    .iter()
                    .find(|(p, _)| p == &prop.property_type)
                    .map(|(_, v)| v),
            ) {
                prop.current = interpolate_value(start_val, end_val, eased);
            }
        }

        Ok(())
    }

    fn find_keyframe_range<'a>(
        &self,
        sorted_kf: &'a [Keyframe],
        progress: f64,
    ) -> (&'a Keyframe, &'a Keyframe, f64) {
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

        (start_kf, end_kf, local_progress)
    }

    fn apply_properties(&self) -> Result<(), JsValue> {
        let mut transform_parts = Vec::with_capacity(16);
        let mut filter_parts = Vec::with_capacity(8);
        let mut has_translate = false;

        for prop in self.properties.iter() {
            match prop.property_type {
                // Transform Group
                PropertyType::X | PropertyType::Y | PropertyType::Z => {
                    if !has_translate {
                        self.apply_translate(&mut transform_parts);
                        has_translate = true;
                    }
                }
                PropertyType::Scale => {
                    if let AnimatableValue::Number(val) = prop.current {
                        transform_parts.push(format!("scale({})", val));
                    }
                }
                PropertyType::ScaleX => {
                    if let AnimatableValue::Number(val) = prop.current {
                        transform_parts.push(format!("scaleX({})", val));
                    }
                }
                PropertyType::ScaleY => {
                    if let AnimatableValue::Number(val) = prop.current {
                        transform_parts.push(format!("scaleY({})", val));
                    }
                }
                PropertyType::Rotate
                | PropertyType::RotateX
                | PropertyType::RotateY
                | PropertyType::RotateZ => {
                    self.apply_rotation(&mut transform_parts, prop);
                }
                PropertyType::SkewX | PropertyType::SkewY => {
                    self.apply_skew(&mut transform_parts, prop);
                }
                PropertyType::Perspective => {
                    if let AnimatableValue::Number(val) = prop.current {
                        transform_parts.push(format!("perspective({}px)", val));
                    }
                }
                PropertyType::PerspectiveOriginX | PropertyType::PerspectiveOriginY => {
                    self.apply_perspective_origin()?;
                }
                PropertyType::TransformOriginX
                | PropertyType::TransformOriginY
                | PropertyType::TransformOriginZ => {
                    self.apply_transform_origin()?;
                }
                PropertyType::BackfaceVisibility => {
                    // Backface visibility - hidden or visible
                    if let AnimatableValue::Number(val) = prop.current {
                        let visibility = if val > 0.5 { "visible" } else { "hidden" };
                        self.set_element_property("backfaceVisibility", visibility)?;
                    }
                }

                // Layout
                PropertyType::Width
                | PropertyType::Height
                | PropertyType::MinWidth
                | PropertyType::MinHeight
                | PropertyType::MaxWidth
                | PropertyType::MaxHeight => {
                    self.apply_layout(prop)?;
                }

                // Visual
                PropertyType::Opacity => {
                    if let AnimatableValue::Number(val) = prop.current {
                        self.set_element_property("opacity", &val.to_string())?;
                    }
                }
                PropertyType::BackgroundColor | PropertyType::Color | PropertyType::BorderColor => {
                    self.set_color_property(prop)?;
                }
                PropertyType::BorderRadius | PropertyType::BorderWidth => {
                    self.apply_border(prop)?;
                }

                PropertyType::Visibility => {
                    // ✨ NEW
                    if let AnimatableValue::Visibility(val) = &prop.current {
                        self.set_element_property("visibility", val.as_str())?;
                    }
                }

                // Shadows
                PropertyType::ShadowOffsetX
                | PropertyType::ShadowOffsetY
                | PropertyType::ShadowBlur
                | PropertyType::ShadowSpread
                | PropertyType::ShadowColor => {
                    let shadow_string = self.build_shadow_string();
                    if !shadow_string.is_empty() {
                        self.set_element_property("boxShadow", &shadow_string)?;
                    }
                }

                // Filters
                PropertyType::Blur
                | PropertyType::Brightness
                | PropertyType::Contrast
                | PropertyType::Saturate
                | PropertyType::Hue
                | PropertyType::Grayscale
                | PropertyType::Invert
                | PropertyType::Sepia => {
                    self.apply_filter(&mut filter_parts, prop);
                }
                PropertyType::Dropoff => {
                    // Dropoff filter (drop shadow filter)
                    if let AnimatableValue::Number(val) = prop.current {
                        filter_parts
                            .push(format!("drop-shadow(0px 0px {}px rgba(0, 0, 0, 0.5))", val));
                    }
                }
                PropertyType::BackgroundBlur => {
                    // Backdrop filter for background blur effect
                    if let AnimatableValue::Number(val) = prop.current {
                        self.set_element_property("backdropFilter", &format!("blur({}px)", val))?;
                    }
                }

                // SVG
                PropertyType::StrokeDashOffset
                | PropertyType::StrokeWidth
                | PropertyType::FillOpacity
                | PropertyType::StrokeOpacity => {
                    self.apply_svg(prop)?;
                }

                PropertyType::StrokeDashArray => {
                    // Handled separately if needed
                }

                PropertyType::Inset => {
                    // Inset shadow property
                    if let AnimatableValue::Number(val) = prop.current {
                        let inset_value = if val > 0.5 { "inset" } else { "outset" };
                        // Note: CSS doesn't have outset, so we handle inset boolean value
                        if val > 0.5 {
                            self.set_element_property(
                                "boxShadow",
                                &format!("{} 0px 0px 0px rgba(0,0,0,0.5)", inset_value),
                            )?;
                        }
                    }
                }
            }
        }

        if !transform_parts.is_empty() {
            self.set_element_property("transform", &transform_parts.join(" "))?;
        }

        if !filter_parts.is_empty() {
            self.set_element_property("filter", &filter_parts.join(" "))?;
        }

        Ok(())
    }

    #[inline]
    fn apply_perspective_origin(&self) -> Result<(), JsValue> {
        let origin_x = self
            .properties
            .iter()
            .find(|p| p.property_type == PropertyType::PerspectiveOriginX)
            .and_then(|p| match &p.current {
                AnimatableValue::Length(val, unit) => Some(format!("{}{}", val, unit.as_str())),
                _ => None,
            })
            .unwrap_or_else(|| "50%".to_string());

        let origin_y = self
            .properties
            .iter()
            .find(|p| p.property_type == PropertyType::PerspectiveOriginY)
            .and_then(|p| match &p.current {
                AnimatableValue::Length(val, unit) => Some(format!("{}{}", val, unit.as_str())),
                _ => None,
            })
            .unwrap_or_else(|| "50%".to_string());

        let origin_string = format!("{} {}", origin_x, origin_y);
        self.set_element_property("perspectiveOrigin", &origin_string)?;
        Ok(())
    }

    #[inline]
    fn apply_translate(&self, transform_parts: &mut Vec<String>) {
        let x = self.get_number_value(PropertyType::X).round();
        let y = self.get_number_value(PropertyType::Y).round();
        let z = self.get_number_value(PropertyType::Z).round();

        if x != 0.0 || y != 0.0 || z != 0.0 {
            transform_parts.push(format!(
                "translate3d({}px, {}px, {}px)",
                x as i32, y as i32, z as i32
            ));
        }
    }

    #[inline]
    fn apply_rotation(&self, transform_parts: &mut Vec<String>, prop: &AnimationProperty) {
        if let AnimatableValue::Number(val) = prop.current {
            match prop.property_type {
                PropertyType::Rotate => transform_parts.push(format!("rotate({}deg)", val)),
                PropertyType::RotateX => transform_parts.push(format!("rotateX({}deg)", val)),
                PropertyType::RotateY => transform_parts.push(format!("rotateY({}deg)", val)),
                PropertyType::RotateZ => transform_parts.push(format!("rotateZ({}deg)", val)),
                _ => {}
            }
        }
    }

    #[inline]
    fn apply_skew(&self, transform_parts: &mut Vec<String>, prop: &AnimationProperty) {
        if let AnimatableValue::Number(val) = prop.current {
            match prop.property_type {
                PropertyType::SkewX => transform_parts.push(format!("skewX({}deg)", val)),
                PropertyType::SkewY => transform_parts.push(format!("skewY({}deg)", val)),
                _ => {}
            }
        }
    }

    #[inline]
    fn apply_layout(&self, prop: &AnimationProperty) -> Result<(), JsValue> {
        let property_name = match prop.property_type {
            PropertyType::Width => "width",
            PropertyType::Height => "height",
            PropertyType::MinWidth => "minWidth",
            PropertyType::MinHeight => "minHeight",
            PropertyType::MaxWidth => "maxWidth",
            PropertyType::MaxHeight => "maxHeight",
            _ => return Ok(()),
        };
        self.set_element_dimension(property_name, &prop.current)?;
        Ok(())
    }

    #[inline]
    fn apply_border(&self, prop: &AnimationProperty) -> Result<(), JsValue> {
        if let AnimatableValue::Length(val, unit) = &prop.current {
            let property_name = match prop.property_type {
                PropertyType::BorderRadius => "border-radius",
                PropertyType::BorderWidth => "border-width",
                _ => return Ok(()),
            };
            self.set_element_property(property_name, &format!("{}{}", val, unit.as_str()))?;
        }
        Ok(())
    }

    #[inline]
    fn apply_filter(&self, filter_parts: &mut Vec<String>, prop: &AnimationProperty) {
        if let AnimatableValue::Number(val) = prop.current {
            match prop.property_type {
                PropertyType::Blur => filter_parts.push(format!("blur({}px)", val)),
                PropertyType::Brightness => filter_parts.push(format!("brightness({})", val)),
                PropertyType::Contrast => filter_parts.push(format!("contrast({})", val)),
                PropertyType::Saturate => filter_parts.push(format!("saturate({})", val)),
                PropertyType::Hue => filter_parts.push(format!("hue-rotate({}deg)", val)),
                PropertyType::Grayscale => {
                    filter_parts.push(format!("grayscale({}%)", (val * 100.0).round() as i32))
                }
                PropertyType::Invert => {
                    filter_parts.push(format!("invert({}%)", (val * 100.0).round() as i32))
                }
                PropertyType::Sepia => {
                    filter_parts.push(format!("sepia({}%)", (val * 100.0).round() as i32))
                }
                PropertyType::Dropoff
                | PropertyType::BackgroundBlur
                | PropertyType::Inset
                | PropertyType::BackfaceVisibility => {
                    // Handled in apply_properties
                }
                _ => {}
            }
        }
    }

    #[inline]
    fn apply_svg(&self, prop: &AnimationProperty) -> Result<(), JsValue> {
        if let AnimatableValue::Number(val) = prop.current {
            let attribute = match prop.property_type {
                PropertyType::StrokeDashOffset => "stroke-dashoffset",
                PropertyType::StrokeWidth => "stroke-width",
                PropertyType::FillOpacity => "fill-opacity",
                PropertyType::StrokeOpacity => "stroke-opacity",
                _ => return Ok(()),
            };
            self.set_svg_attribute(attribute, &val.to_string())?;
        }
        Ok(())
    }

    #[inline]
    fn get_number_value(&self, prop_type: PropertyType) -> f64 {
        self.properties
            .iter()
            .find(|p| p.property_type == prop_type)
            .and_then(|p| match p.current {
                AnimatableValue::Number(n) => Some(n),
                AnimatableValue::Length(n, _) => Some(n),
                _ => None,
            })
            .unwrap_or(0.0)
    }

    #[inline]
    fn set_element_property(&self, property: &str, value: &str) -> Result<(), JsValue> {
        if let Ok(html_element) = self.element.clone().dyn_into::<HtmlElement>() {
            html_element
                .style()
                .set_property(property, value)
                .map_err(|_| JsValue::from_str(&format!("Failed to set {}", property)))?;
        }
        Ok(())
    }

    #[inline]
    fn set_element_dimension(
        &self,
        property: &str,
        value: &AnimatableValue,
    ) -> Result<(), JsValue> {
        match value {
            AnimatableValue::Length(val, unit) => {
                self.set_element_property(property, &format!("{}{}", val, unit.as_str()))?;
            }
            AnimatableValue::Number(val) => {
                self.set_element_property(property, &format!("{}px", val))?;
            }
            _ => {}
        }
        Ok(())
    }

    #[inline]
    fn set_color_property(&self, prop: &AnimationProperty) -> Result<(), JsValue> {
        let property_name = match prop.property_type {
            PropertyType::BackgroundColor => "background-color",
            PropertyType::Color => "color",
            PropertyType::BorderColor => "border-color",
            _ => return Ok(()),
        };

        if let AnimatableValue::Color(r, g, b, a) = prop.current {
            let css_value = format!(
                "rgba({}, {}, {}, {})",
                r.round() as u8,
                g.round() as u8,
                b.round() as u8,
                a
            );
            self.set_element_property(property_name, &css_value)?;
        }
        Ok(())
    }

    #[inline]
    fn set_svg_attribute(&self, attribute: &str, value: &str) -> Result<(), JsValue> {
        if let Ok(svg_element) = self.element.clone().dyn_into::<SvgElement>() {
            svg_element.set_attribute(attribute, value).map_err(|_| {
                JsValue::from_str(&format!("Failed to set SVG attribute {}", attribute))
            })?;
        }
        Ok(())
    }

    #[inline]
    fn apply_transform_origin(&self) -> Result<(), JsValue> {
        let origin_string = format!(
            "{} {} {}",
            self.transform_origin.0, self.transform_origin.1, self.transform_origin.2
        );
        self.set_element_property("transformOrigin", &origin_string)?;
        Ok(())
    }

    #[inline]
    fn build_shadow_string(&self) -> String {
        let offset_x = self.get_number_value(PropertyType::ShadowOffsetX);
        let offset_y = self.get_number_value(PropertyType::ShadowOffsetY);
        let blur = self.get_number_value(PropertyType::ShadowBlur);
        let spread = self.get_number_value(PropertyType::ShadowSpread);

        if offset_x == 0.0 && offset_y == 0.0 && blur == 0.0 && spread == 0.0 {
            return String::new();
        }

        let color = self
            .properties
            .iter()
            .find(|p| p.property_type == PropertyType::ShadowColor)
            .and_then(|p| match &p.current {
                AnimatableValue::Color(r, g, b, a) => Some(format!(
                    "rgba({}, {}, {}, {})",
                    r.round() as u8,
                    g.round() as u8,
                    b.round() as u8,
                    a
                )),
                _ => None,
            })
            .unwrap_or_else(|| "rgba(0, 0, 0, 0.5)".to_string());

        format!(
            "{}px {}px {}px {}px {}",
            offset_x.round() as i32,
            offset_y.round() as i32,
            blur.round() as i32,
            spread.round() as i32,
            color
        )
    }

    // ========================================================================
    // ANIMATION CONDITIONS
    // ========================================================================

    #[wasm_bindgen]
    pub fn animate_if(
        mut self,
        condition: bool,
        true_config: JsValue,
        false_config: JsValue,
    ) -> Result<Animation, JsValue> {
        let config = if condition { true_config } else { false_config };
        let cfg: AnimateConfig = from_value(config)
            .map_err(|e| JsValue::from_str(&format!("Invalid config: {:?}", e)))?;

        self.setup_properties(&cfg)?;
        Ok(self)
    }

    #[wasm_bindgen]
    pub fn animate_match(mut self, value: JsValue, cases: JsValue) -> Result<Animation, JsValue> {
        // Convert cases to object
        let cases_obj = cases
            .dyn_ref::<js_sys::Object>()
            .ok_or_else(|| JsValue::from_str("cases must be an object"))?;

        // Get keys from the object
        let keys = js_sys::Object::keys(cases_obj);

        // Convert value to string for comparison
        let value_str = value.as_string().unwrap_or_default();

        for i in 0..keys.length() {
            let key_val = keys.get(i);

            if let Some(key_str) = key_val.as_string() {
                if key_str == value_str {
                    match js_sys::Reflect::get(cases_obj, &key_val) {
                        Ok(config) => match from_value::<AnimateConfig>(config) {
                            Ok(cfg) => {
                                self.setup_properties(&cfg)?;
                                break;
                            }
                            Err(e) => {
                                return Err(JsValue::from_str(&format!("Invalid config: {:?}", e)));
                            }
                        },
                        Err(_) => {
                            return Err(JsValue::from_str("Failed to get case config"));
                        }
                    }
                }
            }
        }

        Ok(self)
    }

    #[wasm_bindgen]
    pub fn animate_ternary(
        mut self,
        condition: bool,
        true_val: f64,
        false_val: f64,
        property: String,
    ) -> Result<Animation, JsValue> {
        let target = if condition { true_val } else { false_val };

        if let Some(prop_type) = PropertyType::from_str(&property) {
            self.add_number_property(prop_type, target);
        } else {
            return Err(JsValue::from_str(&format!(
                "Unknown property: {}",
                property
            )));
        }

        Ok(self)
    }
}

// ============================================================================
// ANIMATION LOOP SPAWNING
// ============================================================================

type AnimationCallback = Closure<dyn FnMut()>;

fn spawn_animation_loop(animation: Rc<RefCell<Animation>>) -> Result<(), JsValue> {
    let window = window().ok_or_else(|| JsValue::from_str("No window available"))?;

    let animation_clone = animation.clone();
    let window_clone = window.clone();

    let closure: Rc<RefCell<Option<AnimationCallback>>> = Rc::new(RefCell::new(None));
    let closure_clone = closure.clone();

    let animate = move || {
        let mut anim = animation_clone.borrow_mut();
        let _ = anim.animate_frame();

        if anim.state != AnimationState::Completed {
            if let Some(ref callback) = *closure_clone.borrow() {
                let _ = window_clone.request_animation_frame(callback.as_ref().unchecked_ref());
            }
        }
    };

    let c = Closure::wrap(Box::new(animate) as Box<dyn FnMut()>);
    window.request_animation_frame(c.as_ref().unchecked_ref())?;
    *closure.borrow_mut() = Some(c);

    Ok(())
}
