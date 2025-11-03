use wasm_bindgen::prelude::*;
use web_sys::{window, Performance};

/// Animation control and batching
#[wasm_bindgen]
pub struct AnimationTransaction {
    duration: f64,
    disable_actions: bool,
    completion_callback: Option<js_sys::Function>,
    is_active: bool,
    performance: Performance,
    transaction_id: String,
}

#[wasm_bindgen]
impl AnimationTransaction {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<AnimationTransaction, JsValue> {
        let window = window().ok_or_else(|| JsValue::from_str("No window available"))?;
        let performance = window
            .performance()
            .ok_or_else(|| JsValue::from_str("No performance API"))?;

        Ok(AnimationTransaction {
            duration: 0.25,
            disable_actions: false,
            completion_callback: None,
            is_active: false,
            performance,
            transaction_id: uuid_v4(),
        })
    }

    /// Set the duration for all animations in this transaction
    #[wasm_bindgen]
    pub fn set_duration(&mut self, duration: f64) {
        self.duration = duration;
    }

    /// Get the transaction duration
    #[wasm_bindgen]
    pub fn get_duration(&self) -> f64 {
        self.duration
    }

    /// Disable implicit animations for property changes
    #[wasm_bindgen]
    pub fn disable_actions(&mut self) {
        self.disable_actions = true;
    }

    /// Enable implicit animations for property changes
    #[wasm_bindgen]
    pub fn enable_actions(&mut self) {
        self.disable_actions = false;
    }

    /// Check if actions are disabled
    #[wasm_bindgen]
    pub fn are_actions_disabled(&self) -> bool {
        self.disable_actions
    }

    /// Set completion callback
    #[wasm_bindgen]
    pub fn set_completion_callback(&mut self, callback: js_sys::Function) {
        self.completion_callback = Some(callback);
    }

    /// Begin transaction
    #[wasm_bindgen]
    pub fn begin(&mut self) {
        self.is_active = true;
    }

    /// Commit transaction
    #[wasm_bindgen]
    pub fn commit(&mut self) -> Result<(), JsValue> {
        if !self.is_active {
            return Err(JsValue::from_str("Transaction not active"));
        }

        self.is_active = false;

        if let Some(ref callback) = self.completion_callback {
            let _ = callback.call0(&JsValue::NULL);
        }

        Ok(())
    }

    /// Get transaction ID
    #[wasm_bindgen]
    pub fn get_transaction_id(&self) -> String {
        self.transaction_id.clone()
    }

    /// Check if transaction is active
    #[wasm_bindgen]
    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

fn uuid_v4() -> String {
    use std::fmt::Write as _;
    let mut uuid = String::with_capacity(36);
    let _ = write!(
        &mut uuid,
        "{:08x}-{:04x}-4{:03x}-{:04x}-{:012x}",
        rand_u32(),
        rand_u16(),
        rand_u16() & 0xfff,
        (rand_u16() & 0x3fff) | 0x8000,
        rand_u32() as u64 * 0x100000000 + rand_u32() as u64
    );
    uuid
}

fn rand_u32() -> u32 {
    (js_sys::Math::random() * 4294967296.0) as u32
}

fn rand_u16() -> u16 {
    (js_sys::Math::random() * 65536.0) as u16
}