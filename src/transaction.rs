
// ============================================================================
// TRANSACTION SYSTEM 
// ============================================================================

use web_sys::{window, Performance};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct AnimationTransaction {
    duration: f64,
    timing_function: TimingFunction,
    disable_actions: bool,
    completion: Option<js_sys::Function>,
    active: bool,
    performance: Performance,
    id: String,
    start_time: f64,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum TimingFunction {
    Default = 0,
    Linear = 1,
    EaseIn = 2,
    EaseOut = 3,
    EaseInOut = 4,
}

#[wasm_bindgen]
impl AnimationTransaction {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<AnimationTransaction, JsValue> {
        let window = window().ok_or_else(|| JsValue::from_str("No window"))?;
        let performance = window
            .performance()
            .ok_or_else(|| JsValue::from_str("No performance API"))?;

        Ok(AnimationTransaction {
            duration: 0.25,
            timing_function: TimingFunction::Default,
            disable_actions: false,
            completion: None,
            active: false,
            performance,
            id: generate_id(),
            start_time: 0.0,
        })
    }

    // ========================================================================
    // CONFIGURATION - Fluent API
    // ========================================================================

    #[wasm_bindgen(js_name = setDuration)]
    pub fn set_duration(mut self, duration: f64) -> Self {
        self.duration = duration.max(0.0);
        self
    }

    #[wasm_bindgen(js_name = setTimingFunction)]
    pub fn set_timing_function(mut self, timing: u8) -> Result<AnimationTransaction, JsValue> {
        self.timing_function = match timing {
            0 => TimingFunction::Default,
            1 => TimingFunction::Linear,
            2 => TimingFunction::EaseIn,
            3 => TimingFunction::EaseOut,
            4 => TimingFunction::EaseInOut,
            _ => return Err(JsValue::from_str("Invalid timing function")),
        };
        Ok(self)
    }

    #[wasm_bindgen(js_name = disableActions)]
    pub fn disable_actions(mut self) -> Self {
        self.disable_actions = true;
        self
    }

    #[wasm_bindgen(js_name = onComplete)]
    pub fn on_complete(mut self, callback: js_sys::Function) -> Self {
        self.completion = Some(callback);
        self
    }

    // ========================================================================
    // TRANSACTION LIFECYCLE
    // ========================================================================

    #[wasm_bindgen]
    pub fn begin(&mut self) {
        self.active = true;
        self.start_time = self.performance.now();
    }

    #[wasm_bindgen]
    pub fn commit(&mut self) -> Result<(), JsValue> {
        if !self.active {
            return Err(JsValue::from_str("Transaction not active"));
        }

        self.active = false;

        if let Some(ref callback) = self.completion {
            let _ = callback.call0(&JsValue::NULL);
        }

        Ok(())
    }

    // ========================================================================
    // QUERIES
    // ========================================================================

    #[wasm_bindgen(getter)]
    pub fn duration(&self) -> f64 {
        self.duration
    }

    #[wasm_bindgen(getter, js_name = timingFunction)]
    pub fn timing_function(&self) -> u8 {
        self.timing_function as u8
    }

    #[wasm_bindgen(getter, js_name = actionsDisabled)]
    pub fn actions_disabled(&self) -> bool {
        self.disable_actions
    }

    #[wasm_bindgen(getter, js_name = isActive)]
    pub fn is_active(&self) -> bool {
        self.active
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    #[wasm_bindgen(js_name = elapsedTime)]
    pub fn elapsed_time(&self) -> f64 {
        if self.active {
            (self.performance.now() - self.start_time) / 1000.0
        } else {
            0.0
        }
    }
}

fn generate_id() -> String {
    use std::fmt::Write;
    let mut id = String::with_capacity(36);
    let _ = write!(
        &mut id,
        "{:08x}-{:04x}-4{:03x}-{:04x}-{:012x}",
        rand_u32(),
        rand_u16(),
        rand_u16() & 0xfff,
        (rand_u16() & 0x3fff) | 0x8000,
        (rand_u32() as u64) << 32 | rand_u32() as u64
    );
    id
}

#[inline]
fn rand_u32() -> u32 {
    (js_sys::Math::random() * 4294967296.0) as u32
}

#[inline]
fn rand_u16() -> u16 {
    (js_sys::Math::random() * 65536.0) as u16
}


// ============================================================================
// TRANSACTION HELPERS - Static methods for global transactions
// ============================================================================

#[wasm_bindgen]
pub struct Transaction;

#[wasm_bindgen]
impl Transaction {
    /// Execute multiple animations in a single transaction
    #[wasm_bindgen]
    pub fn batch(duration: f64, callback: js_sys::Function) -> Result<(), JsValue> {
        let mut txn = AnimationTransaction::new()?;
        txn.duration = duration;
        txn.begin();
        
        let _ = callback.call0(&JsValue::NULL);
        
        txn.commit()?;
        Ok(())
    }

    /// Execute with completion callback
    #[wasm_bindgen(js_name = batchWithCompletion)]
    pub fn batch_with_completion(
        duration: f64,
        animation_block: js_sys::Function,
        completion_block: js_sys::Function,
    ) -> Result<(), JsValue> {
        let mut txn = AnimationTransaction::new()?;
        txn.duration = duration;
        txn.completion = Some(completion_block);
        txn.begin();
        
        let _ = animation_block.call0(&JsValue::NULL);
        
        txn.commit()?;
        Ok(())
    }
}
