use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen]
pub struct Sequencer {
    steps: Vec<TimelineStep>,
    fraction: f64,
    running: bool,
    total_duration: f64,
}

#[derive(Clone)]
struct TimelineStep {
    animation: Rc<RefCell<crate::Animation>>,
    start: f64,      // Start time in ms
    duration: f64,   // Duration in ms
    overlap: f64,    // 0.0 = sequential, 1.0 = parallel
}

#[wasm_bindgen]
impl Sequencer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Sequencer {
        Sequencer {
            steps: Vec::new(),
            fraction: 0.0,
            running: false,
            total_duration: 0.0,
        }
    }

    // ========================================================================
    // DECLARATIVE BUILDER
    // ========================================================================

    /// Add animation step with overlap control
    #[wasm_bindgen(js_name = addStep)]
    pub fn add_step(&mut self, handle: &crate::AnimationHandle, overlap: f64) {
        let anim = Rc::clone(&handle.animation);
        let duration = anim.borrow().duration;
        
        let start = if self.steps.is_empty() {
            0.0
        } else {
            let prev = &self.steps[self.steps.len() - 1];
            let prev_end = prev.start + prev.duration;
            let overlap_offset = prev.duration * overlap.clamp(0.0, 1.0);
            prev_end - overlap_offset
        };

        self.steps.push(TimelineStep {
            animation: anim,
            start,
            duration,
            overlap: overlap.clamp(0.0, 1.0),
        });

        self.recalculate_duration();
    }

    /// Sequential step (starts after previous)
    #[wasm_bindgen(js_name = then)]
    pub fn then(&mut self, handle: &crate::AnimationHandle) {
        self.add_step(handle, 0.0);
    }

    /// Parallel step (starts with previous)
    #[wasm_bindgen(js_name = with)]
    pub fn with(&mut self, handle: &crate::AnimationHandle) {
        self.add_step(handle, 1.0);
    }

    /// Overlapping step
    #[wasm_bindgen(js_name = overlap)]
    pub fn overlap(&mut self, handle: &crate::AnimationHandle, at: f64) {
        self.add_step(handle, at);
    }

    // ========================================================================
    // PLAYBACK
    // ========================================================================

    #[wasm_bindgen]
    pub fn play(&mut self) -> Result<(), JsValue> {
        self.running = true;
        self.fraction = 0.0;
        
        // Start all animations
        for step in &self.steps {
            step.animation.borrow_mut().start_internal()?;
        }
        
        Ok(())
    }

    #[wasm_bindgen]
    pub fn pause(&mut self) -> Result<(), JsValue> {
        self.running = false;
        for step in &self.steps {
            step.animation.borrow_mut().pause()?;
        }
        Ok(())
    }

    #[wasm_bindgen]
    pub fn stop(&mut self) -> Result<(), JsValue> {
        self.running = false;
        self.fraction = 0.0;
        for step in &self.steps {
            step.animation.borrow_mut().stop()?;
        }
        Ok(())
    }

    /// Scrub to specific time fraction (0.0 - 1.0)
    #[wasm_bindgen(js_name = seekTo)]
    pub fn seek_to(&mut self, fraction: f64) -> Result<(), JsValue> {
        self.fraction = fraction.clamp(0.0, 1.0);
        let current_time = self.fraction * self.total_duration;
        
        // Update each animation's fraction based on timeline position
        for step in &self.steps {
            let step_end = step.start + step.duration;
            
            if current_time < step.start {
                step.animation.borrow_mut().set_fraction_complete(0.0)?;
            } else if current_time > step_end {
                step.animation.borrow_mut().set_fraction_complete(1.0)?;
            } else {
                let local_fraction = (current_time - step.start) / step.duration;
                step.animation.borrow_mut().set_fraction_complete(local_fraction)?;
            }
        }
        
        Ok(())
    }

    // ========================================================================
    // QUERIES
    // ========================================================================

    #[wasm_bindgen(getter, js_name = totalDuration)]
    pub fn total_duration(&self) -> f64 {
        self.total_duration
    }

    #[wasm_bindgen(getter, js_name = stepCount)]
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    #[wasm_bindgen(getter)]
    pub fn fraction(&self) -> f64 {
        self.fraction
    }

    #[wasm_bindgen(getter, js_name = isRunning)]
    pub fn is_running(&self) -> bool {
        self.running
    }

    fn recalculate_duration(&mut self) {
        self.total_duration = self.steps.iter()
            .map(|step| step.start + step.duration)
            .fold(0.0, f64::max);
    }
}
