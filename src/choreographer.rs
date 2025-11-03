use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;


// ============================================================================
// CHOREOGRAPHER - Transition Coordinator
// ============================================================================

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub enum TransitionContext {
    Present = 0,
    Dismiss = 1,
    Push = 2,
    Pop = 3,
}

#[wasm_bindgen]
pub struct Choreographer {
    context: TransitionContext,
    fraction: f64,
    interactive: bool,
    cancelled: bool,
    animations: Vec<Rc<RefCell<crate::Animation>>>,
}

#[wasm_bindgen]
impl Choreographer {
    #[wasm_bindgen(constructor)]
    pub fn new(context: u8) -> Result<Choreographer, JsValue> {
        let ctx = match context {
            0 => TransitionContext::Present,
            1 => TransitionContext::Dismiss,
            2 => TransitionContext::Push,
            3 => TransitionContext::Pop,
            _ => return Err(JsValue::from_str("Invalid context: 0=Present, 1=Dismiss, 2=Push, 3=Pop")),
        };

        Ok(Choreographer {
            context: ctx,
            fraction: 0.0,
            interactive: false,
            cancelled: false,
            animations: Vec::new(),
        })
    }

    /// Add animation to be coordinated
    #[wasm_bindgen(js_name = addAnimation)]
    pub fn add_animation(&mut self, handle: &crate::AnimationHandle) {
        self.animations.push(Rc::clone(&handle.animation));
    }

    /// Start interactive transition
    #[wasm_bindgen(js_name = beginInteractive)]
    pub fn begin_interactive(&mut self) -> Result<(), JsValue> {
        self.interactive = true;
        self.fraction = 0.0;
        
        // Pause all animations
        for anim in &self.animations {
            anim.borrow_mut().pause()?;
        }
        
        Ok(())
    }

    /// Update all animations to match progress
    #[wasm_bindgen(js_name = updateInteractive)]
    pub fn update_interactive(&mut self, fraction: f64) -> Result<(), JsValue> {
        if !self.interactive { return Ok(()); }
        
        self.fraction = fraction.clamp(0.0, 1.0);
        
        // Scrub all animations to this fraction
        for anim in &self.animations {
            anim.borrow_mut().set_fraction_complete(self.fraction)?;
        }
        
        Ok(())
    }

    /// Finish interactive transition (auto-complete or cancel)
    #[wasm_bindgen(js_name = finishInteractive)]
    pub fn finish_interactive(&mut self, velocity: f64) -> Result<bool, JsValue> {
        self.interactive = false;
        
        let should_complete = self.fraction > 0.5 || velocity > 0.3;
        
        if should_complete {
            // Complete all animations
            for anim in &self.animations {
                anim.borrow_mut().resume()?;
            }
            Ok(true)
        } else {
            // Cancel - reverse all animations
            self.cancelled = true;
            for anim in &self.animations {
                let mut a = anim.borrow_mut();
                a.reverse()?;
                a.resume()?;
            }
            Ok(false)
        }
    }

    /// Cancel interactive transition
    #[wasm_bindgen(js_name = cancelInteractive)]
    pub fn cancel_interactive(&mut self) -> Result<(), JsValue> {
        self.cancelled = true;
        self.interactive = false;
        
        // Reverse all animations back to start
        for anim in &self.animations {
            let mut a = anim.borrow_mut();
            a.reverse()?;
            a.resume()?;
        }
        
        Ok(())
    }

    // Properties
    #[wasm_bindgen(getter)]
    pub fn fraction(&self) -> f64 {
        self.fraction
    }

    #[wasm_bindgen(getter, js_name = isInteractive)]
    pub fn is_interactive(&self) -> bool {
        self.interactive
    }

    #[wasm_bindgen(getter, js_name = isCancelled)]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    #[wasm_bindgen(getter)]
    pub fn context(&self) -> u8 {
        self.context as u8
    }
}

