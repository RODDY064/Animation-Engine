use wasm_bindgen::prelude::*;

/// Metal Acceleration - GPU-accelerated animation rendering hints
#[wasm_bindgen]
pub struct GPUAccelerator {
    use_gpu: bool,
    supported: bool,
    optimization_level: u8,
}

#[wasm_bindgen]
impl GPUAccelerator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GPUAccelerator {
        let supported = check_webgpu_support();
        GPUAccelerator {
            use_gpu: supported,
            supported,
            optimization_level: 2,
        }
    }

    /// Check if GPU acceleration is available
    #[wasm_bindgen]
    pub fn is_supported(&self) -> bool {
        self.supported
    }

    /// Enable GPU acceleration
    #[wasm_bindgen]
    pub fn enable(&mut self) -> Result<(), JsValue> {
        if !self.supported {
            return Err(JsValue::from_str("GPU acceleration not supported"));
        }
        self.use_gpu = true;
        Ok(())
    }

    /// Disable GPU acceleration
    #[wasm_bindgen]
    pub fn disable(&mut self) {
        self.use_gpu = false;
    }

    /// Check if GPU acceleration is enabled
    #[wasm_bindgen]
    pub fn is_enabled(&self) -> bool {
        self.use_gpu
    }

    /// Set optimization level (0-3, higher = more aggressive)
    #[wasm_bindgen]
    pub fn set_optimization_level(&mut self, level: u8) -> Result<(), JsValue> {
        if level > 3 {
            return Err(JsValue::from_str("Optimization level must be 0-3"));
        }
        self.optimization_level = level;
        Ok(())
    }

    /// Get optimization level
    #[wasm_bindgen]
    pub fn get_optimization_level(&self) -> u8 {
        self.optimization_level
    }

    /// Apply GPU-friendly CSS properties
    #[wasm_bindgen]
    pub fn apply_gpu_hints(&self, element: &web_sys::Element) -> Result<(), JsValue> {
        if let Ok(html_elem) = element.clone().dyn_into::<web_sys::HtmlElement>() {
            let style = html_elem.style();

            // Apply will-change for optimal GPU rendering
            if self.use_gpu {
                let _ = style.set_property("will-change", "transform, opacity");
                let _ = style.set_property("transform", "translateZ(0)");
                let _ = style.set_property("backface-visibility", "hidden");
                let _ = style.set_property("perspective", "1000px");
            }
        }
        Ok(())
    }

    /// Remove GPU hints
    #[wasm_bindgen]
    pub fn remove_gpu_hints(&self, element: &web_sys::Element) -> Result<(), JsValue> {
        if let Ok(html_elem) = element.clone().dyn_into::<web_sys::HtmlElement>() {
            let style = html_elem.style();
            let _ = style.remove_property("will-change");
            let _ = style.remove_property("backface-visibility");
        }
        Ok(())
    }
}

fn check_webgpu_support() -> bool {
    // Check for WebGPU support
    if let Some(window) = web_sys::window() {
        let navigator = window.navigator();
        return js_sys::Reflect::has(&navigator, &JsValue::from_str("gpu"))
            .unwrap_or(false);
    }
    false
}