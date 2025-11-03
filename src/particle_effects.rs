use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement};

/// Particle Effects - Multiple element animations with particle system
#[wasm_bindgen]
pub struct ParticleEmitter {
    particles: Vec<Particle>,
    #[allow(dead_code)]
    emit_rate: f64,
    #[allow(dead_code)]
    lifetime: f64,
    velocity_x: f64,
    velocity_y: f64,
    gravity: f64,
    is_active: bool,
}

#[derive(Clone)]
struct Particle {
    element: Rc<Element>,
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    life: f64,
    max_life: f64,
    opacity: f64,
}

#[wasm_bindgen]
impl ParticleEmitter {
    #[wasm_bindgen(constructor)]
    pub fn new(emit_rate: f64, lifetime: f64) -> ParticleEmitter {
        ParticleEmitter {
            particles: Vec::new(),
            emit_rate,
            lifetime,
            velocity_x: 0.0,
            velocity_y: 0.0,
            gravity: 9.81,
            is_active: false,
        }
    }

    /// Set initial velocity for particles
    #[wasm_bindgen]
    pub fn set_velocity(&mut self, vx: f64, vy: f64) {
        self.velocity_x = vx;
        self.velocity_y = vy;
    }

    /// Set gravity effect on particles
    #[wasm_bindgen]
    pub fn set_gravity(&mut self, gravity: f64) {
        self.gravity = gravity;
    }

    /// Start emitting particles
    #[wasm_bindgen]
    pub fn start(&mut self) {
        self.is_active = true;
    }

    /// Stop emitting particles
    #[wasm_bindgen]
    pub fn stop(&mut self) {
        self.is_active = false;
    }

    /// Update particles (called each frame)
    #[wasm_bindgen]
    pub fn update(&mut self, delta_time: f64) -> Result<(), JsValue> {
        // Update existing particles
        self.particles.iter_mut().for_each(|particle| {
            particle.life -= delta_time;
            particle.vy += self.gravity * delta_time;
            particle.x += particle.vx * delta_time;
            particle.y += particle.vy * delta_time;
            particle.opacity = (particle.life / particle.max_life).max(0.0);
        });

        // Remove dead particles
        self.particles.retain(|p| p.life > 0.0);

        // Apply visual updates
        for particle in &self.particles {
            if let Some(html_elem) = particle.element.dyn_ref::<HtmlElement>() {
                let style = html_elem.style();
                let _ = style.set_property(
                    "transform",
                    &format!("translate({}px, {}px)", particle.x, particle.y),
                );
                let _ = style.set_property("opacity", &particle.opacity.to_string());
            }
        }

        Ok(())
    }

    /// Get number of active particles
    #[wasm_bindgen]
    pub fn get_particle_count(&self) -> usize {
        self.particles.len()
    }

    /// Check if emitter is active
    #[wasm_bindgen]
    pub fn is_emitting(&self) -> bool {
        self.is_active
    }
}