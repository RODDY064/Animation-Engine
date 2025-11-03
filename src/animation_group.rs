use wasm_bindgen::prelude::*;

/// Manages grouped simultaneous animations
#[wasm_bindgen]
pub struct AnimationGroup {
    animations: Vec<crate::AnimationHandle>,
    group_id: String,
    is_playing: bool,
}

#[wasm_bindgen]
impl AnimationGroup {
    #[wasm_bindgen(constructor)]
    pub fn new(group_id: String) -> AnimationGroup {
        AnimationGroup {
            animations: Vec::new(),
            group_id,
            is_playing: false,
        }
    }

    /// Add a pre-created animation to the group
    #[wasm_bindgen]
    pub fn add_animation(&mut self, animation: crate::AnimationHandle) {
        self.animations.push(animation);
    }

    /// Play all animations simultaneously
    #[wasm_bindgen]
    pub fn play(&mut self) -> Result<(), JsValue> {
        if self.animations.is_empty() {
            return Err(JsValue::from_str("No animations in group"));
        }

        self.is_playing = true;

        for anim_handle in self.animations.iter() {
            anim_handle.resume()?;
        }

        Ok(())
    }

    /// Pause all animations in the group
    #[wasm_bindgen]
    pub fn pause(&mut self) -> Result<(), JsValue> {
        for anim_handle in self.animations.iter() {
            anim_handle.pause()?;
        }
        self.is_playing = false;
        Ok(())
    }

    /// Resume all animations in the group
    #[wasm_bindgen]
    pub fn resume(&mut self) -> Result<(), JsValue> {
        for anim_handle in self.animations.iter() {
            anim_handle.resume()?;
        }
        self.is_playing = true;
        Ok(())
    }

    /// Stop all animations in the group
    #[wasm_bindgen]
    pub fn stop(&mut self) -> Result<(), JsValue> {
        for anim_handle in self.animations.iter() {
            anim_handle.stop()?;
        }
        self.is_playing = false;
        Ok(())
    }

    /// Reverse all animations in the group
    #[wasm_bindgen]
    pub fn reverse(&mut self) -> Result<(), JsValue> {
        for anim_handle in self.animations.iter() {
            anim_handle.reverse()?;
        }
        Ok(())
    }

    /// Get the number of animations in the group
    #[wasm_bindgen]
    pub fn get_animation_count(&self) -> usize {
        self.animations.len()
    }

    /// Check if group is currently playing
    #[wasm_bindgen]
    pub fn is_playing_group(&self) -> bool {
        self.is_playing
    }

    /// Get group ID
    #[wasm_bindgen]
    pub fn get_group_id(&self) -> String {
        self.group_id.clone()
    }
}