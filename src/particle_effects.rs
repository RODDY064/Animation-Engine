
use std::rc::Rc;
use web_sys::{Element, HtmlElement};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ParticleEmitter {
    particles: Vec<Particle>,
    velocity: (f64, f64),
    velocity_variance: f64,
    gravity: f64,
    lifetime: f64,
    lifetime_variance: f64,
    active: bool,
    max_particles: usize,
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
    scale: f64,
    rotation: f64,
    angular_velocity: f64,
}

#[wasm_bindgen]
impl ParticleEmitter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ParticleEmitter {
        ParticleEmitter {
            particles: Vec::with_capacity(100),
            velocity: (0.0, -100.0),
            velocity_variance: 50.0,
            gravity: 200.0,
            lifetime: 2.0,
            lifetime_variance: 0.5,
            active: false,
            max_particles: 100,
        }
    }

    /// Configure particle behavior
    #[wasm_bindgen(js_name = setVelocity)]
    pub fn set_velocity(&mut self, vx: f64, vy: f64) {
        self.velocity = (vx, vy);
    }

    #[wasm_bindgen(js_name = setVelocityVariance)]
    pub fn set_velocity_variance(&mut self, variance: f64) {
        self.velocity_variance = variance.max(0.0);
    }

    #[wasm_bindgen(js_name = setGravity)]
    pub fn set_gravity(&mut self, gravity: f64) {
        self.gravity = gravity;
    }

    #[wasm_bindgen(js_name = setLifetime)]
    pub fn set_lifetime(&mut self, lifetime: f64, variance: f64) {
        self.lifetime = lifetime.max(0.1);
        self.lifetime_variance = variance.max(0.0);
    }

    #[wasm_bindgen(js_name = setMaxParticles)]
    pub fn set_max_particles(&mut self, max: usize) {
        self.max_particles = max.clamp(1, 1000);
    }

    // ========================================================================
    // EMISSION CONTROL
    // ========================================================================

    #[wasm_bindgen]
    pub fn start(&mut self) {
        self.active = true;
    }

    #[wasm_bindgen]
    pub fn stop(&mut self) {
        self.active = false;
    }

    #[wasm_bindgen]
    pub fn clear(&mut self) {
        self.particles.clear();
    }

    /// Emit a single particle
    #[wasm_bindgen]
    pub fn emit(&mut self, element: Element, x: f64, y: f64) {
        if self.particles.len() >= self.max_particles {
            return;
        }

        let variance = self.velocity_variance;
        let vx = self.velocity.0 + (random() - 0.5) * variance * 2.0;
        let vy = self.velocity.1 + (random() - 0.5) * variance * 2.0;
        let life = self.lifetime + (random() - 0.5) * self.lifetime_variance * 2.0;

        self.particles.push(Particle {
            element: Rc::new(element),
            x,
            y,
            vx,
            vy,
            life: life.max(0.1),
            max_life: life.max(0.1),
            scale: 1.0,
            rotation: 0.0,
            angular_velocity: (random() - 0.5) * 360.0,
        });
    }

    /// Emit burst of particles
    #[wasm_bindgen(js_name = emitBurst)]
    pub fn emit_burst(&mut self, element: Element, x: f64, y: f64, count: usize) {
        for _ in 0..count {
            self.emit(element.clone(), x, y);
        }
    }

    // ========================================================================
    // UPDATE LOOP
    // ========================================================================

    #[wasm_bindgen]
    pub fn update(&mut self, delta_time: f64) -> Result<(), JsValue> {
        let dt = delta_time.min(0.1); // Cap to prevent huge jumps

        // Update particles
        for particle in &mut self.particles {
            particle.life -= dt;
            particle.vy += self.gravity * dt;
            particle.x += particle.vx * dt;
            particle.y += particle.vy * dt;
            particle.rotation += particle.angular_velocity * dt;

            // Fade out
            let life_fraction = (particle.life / particle.max_life).max(0.0);
            particle.scale = life_fraction;
        }

        // Remove dead particles
        self.particles.retain(|p| p.life > 0.0);

        // Apply visual updates
        for particle in &self.particles {
            if let Some(html) = particle.element.dyn_ref::<HtmlElement>() {
                let style = html.style();
                let _ = style.set_property(
                    "transform",
                    &format!(
                        "translate({}px, {}px) scale({}) rotate({}deg)",
                        particle.x, particle.y, particle.scale, particle.rotation
                    ),
                );
                let _ = style.set_property("opacity", &particle.scale.to_string());
            }
        }

        Ok(())
    }

    // ========================================================================
    // QUERIES
    // ========================================================================

    #[wasm_bindgen(getter, js_name = particleCount)]
    pub fn particle_count(&self) -> usize {
        self.particles.len()
    }

    #[wasm_bindgen(getter, js_name = isActive)]
    pub fn is_active(&self) -> bool {
        self.active
    }

    #[wasm_bindgen(getter, js_name = maxParticles)]
    pub fn max_particles(&self) -> usize {
        self.max_particles
    }
}

// Simple random number generator (0.0 - 1.0)
fn random() -> f64 {
    (js_sys::Math::random() * 1000.0).fract()
}


// ============================================================================
// PRESET PARTICLE EFFECTS
// ============================================================================

#[wasm_bindgen]
pub struct ParticlePresets;

#[wasm_bindgen]
impl ParticlePresets {
    /// Confetti explosion
    #[wasm_bindgen]
    pub fn confetti() -> ParticleEmitter {
        let mut emitter = ParticleEmitter::new();
        emitter.set_velocity(0.0, -300.0);
        emitter.set_velocity_variance(200.0);
        emitter.set_gravity(500.0);
        emitter.set_lifetime(3.0, 1.0);
        emitter.set_max_particles(50);
        emitter
    }

    /// Smoke/dust effect
    #[wasm_bindgen]
    pub fn smoke() -> ParticleEmitter {
        let mut emitter = ParticleEmitter::new();
        emitter.set_velocity(0.0, -50.0);
        emitter.set_velocity_variance(30.0);
        emitter.set_gravity(-20.0); // Float upward
        emitter.set_lifetime(2.0, 0.5);
        emitter.set_max_particles(30);
        emitter
    }

    /// Sparkle effect
    #[wasm_bindgen]
    pub fn sparkle() -> ParticleEmitter {
        let mut emitter = ParticleEmitter::new();
        emitter.set_velocity(0.0, 0.0);
        emitter.set_velocity_variance(100.0);
        emitter.set_gravity(0.0);
        emitter.set_lifetime(1.0, 0.3);
        emitter.set_max_particles(20);
        emitter
    }

    /// Explosion burst
    #[wasm_bindgen]
    pub fn explosion() -> ParticleEmitter {
        let mut emitter = ParticleEmitter::new();
        emitter.set_velocity(0.0, 0.0);
        emitter.set_velocity_variance(300.0);
        emitter.set_gravity(300.0);
        emitter.set_lifetime(1.5, 0.5);
        emitter.set_max_particles(40);
        emitter
    }
}