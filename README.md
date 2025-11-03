# UI Animation Engine

A high-performance, Apple-inspired animation engine compiled to WebAssembly. Brings native iOS animation quality to the web with spring physics, gesture control, and GPU acceleration.

**Author:** RODDY064

---

## ✨ Features

- 🎯 **Apple-Quality Animations** - UIKit/SwiftUI-inspired API
- ⚡ **60-120 FPS** - GPU-accelerated transforms
- 🎢 **Real Spring Physics** - Natural, organic motion
- 👆 **Gesture Integration** - Touch velocity tracking & interactive control
- 🎬 **Choreography System** - Coordinate complex transitions
- 📊 **Timeline Sequencing** - Declarative animation composition
- 🎯 **Interactive Scrubbing** - Manual timeline control
- 🎨 **Full Property Support** - Transform, color, filter, SVG

---

## 🚀 Quick Start

```javascript
import init, { Animation } from 'animation-engine';

await init();

const element = document.getElementById('box');

// Simple animation
new Animation(element)
    .smooth(400)
    .animate({ x: 200, opacity: 0.5 })
    .start();
```

---

## 🎬 Core API

### Animation Basics
```javascript
// Choose your timing curve
.smooth(400)      // Apple's signature curve (recommended)
.snappy(300)      // Quick & responsive
.bounce(500)      // Playful overshoot
.ease_out(400)    // Classic iOS feel

// Animate properties
.animate({
    x: 200,           // Transform
    scale: 1.5,
    rotate: 360,
    opacity: 0.5,     // Visual
    blur: 10,         // Filters
    width: "300px"    // Layout
})

// Control playback
.start()
.pause()
.resume()
.reverse()
```

### Spring Physics
```javascript
// Built-in spring presets
new Animation(element)
    .spring_smooth()    // Polished (400 stiffness, 40 damping)
    .spring_bouncy()    // Playful (250, 15)
    .spring_default()   // Balanced (300, 30)
    .animate({ x: 200 })
    .start();

// Custom spring
const spring = new Spring(350, 28); // stiffness, damping
spring.update(targetValue, deltaTime);
```

---

## 👆 Gesture Control

### Interactive Animations
```javascript
import { GestureController } from 'animation-engine';

const gesture = new GestureController()
    .withSpring(Spring.smooth());
gesture.connectAnimation(animation);

// Touch events
element.addEventListener('touchstart', (e) => {
    gesture.onTapDown(e.touches[0].clientX, e.touches[0].clientY, e.timeStamp);
});

element.addEventListener('touchmove', (e) => {
    gesture.onTapMove(e.touches[0].clientX, e.touches[0].clientY, e.timeStamp);
});

element.addEventListener('touchend', () => {
    gesture.onTapUp(); // Auto-completes or cancels
});
```

### Configuration
```javascript
gesture.sensitivity = 500;           // Pixels for full range
gesture.completionThreshold = 0.5;   // When to complete vs cancel
```

---

## 🎭 Choreography

### Coordinate Transitions
```javascript
import { Choreographer, TransitionContext } from 'animation-engine';

const choreographer = new Choreographer(TransitionContext.Present);

// Add multiple animations
choreographer.addAnimation(modalAnimation.start());
choreographer.addAnimation(overlayAnimation.start());

// Interactive control
choreographer.beginInteractive();
choreographer.updateInteractive(progress);
choreographer.finishInteractive(velocity); // Smart completion
```

### Contexts
```javascript
TransitionContext.Present  // Modal presentation
TransitionContext.Dismiss  // Modal dismissal
TransitionContext.Push     // Navigation push
TransitionContext.Pop      // Navigation pop
```

---

## 📊 Timeline Sequencing

### Declarative Composition
```javascript
import { Sequencer } from 'animation-engine';

const sequencer = new Sequencer();

sequencer.then(anim1);         // Sequential - starts after previous
sequencer.with(anim2);         // Parallel - starts with previous
sequencer.overlap(anim3, 0.7); // Starts at 70% of previous

sequencer.play();
sequencer.seekTo(0.5); // Scrub to 50%
```

### Timeline Control
```javascript
sequencer.play();
sequencer.pause();
sequencer.stop();
sequencer.seekTo(fraction); // 0.0 - 1.0

// Query state
console.log(sequencer.totalDuration);
console.log(sequencer.fraction);
console.log(sequencer.isRunning);
```

---

## 🎯 Animatable Properties

```javascript
.animate({
    // Transform
    x, y, z: 200,
    scale, scale_x, scale_y: 1.5,
    rotate, rotate_x, rotate_y, rotate_z: 360,
    skew_x, skew_y: 10,
    
    // Visual
    opacity: 0.5,
    background_color: "#667eea",
    color: "#000",
    border_color: "rgb(100, 200, 255)",
    
    // Layout
    width: "300px",
    height: "200px",
    border_radius: "20px",
    
    // Filters
    blur: 10,
    brightness: 1.5,
    contrast: 1.2,
    saturate: 0.8,
    hue: 180,
    grayscale: 0.5,
    
    // Shadows
    shadow_offset_x: 5,
    shadow_offset_y: 10,
    shadow_blur: 20,
    shadow_color: "rgba(0,0,0,0.3)",
    
    // SVG
    stroke_dashoffset: 0,
    stroke_width: 3,
    fill_opacity: 1.0
})
```

---

## 🎮 Interactive Scrubbing

```javascript
const anim = new Animation(element)
    .smooth(1000)
    .animate({ x: 200, rotate: 360 });

// Manual control (don't call .start())
anim.set_fraction_complete(0.0);   // Start
anim.set_fraction_complete(0.5);   // 50%
anim.set_fraction_complete(1.0);   // End

// Query progress
const progress = anim.get_fraction_complete(); // 0.0 - 1.0
const state = anim.get_state(); // Idle, Running, Paused, Completed
```

---

## 🔗 Keyframe Animations

```javascript
new Animation(element)
    .smooth(2000)
    .add_keyframes([
        { time: 0.0,  x: 0,   y: 0,   scale: 1 },
        { time: 0.25, x: 100, y: 50,  scale: 1.2 },
        { time: 0.5,  x: 200, y: 0,   scale: 1.5 },
        { time: 0.75, x: 100, y: -50, scale: 1.2 },
        { time: 1.0,  x: 0,   y: 0,   scale: 1 }
    ])
    .start();
```

---

## 🎪 Advanced Features

### Continue Previous State
```javascript
// Preserve transform between animations
new Animation(element)
    .smooth(400)
    .continue_animate()  // Start from current position
    .animate({ x: 200 })
    .start();
```

### Additive Animations
```javascript
// Layer multiple animations
new Animation(element)
    .smooth(500)
    .additive()  // Doesn't cancel existing animations
    .animate({ x: 200 })
    .start();
```

### Repeating
```javascript
new Animation(element)
    .smooth(400)
    .repeat(5)           // Repeat 5 times
    .auto_reverse()      // Reverse on each repeat
    .animate({ x: 200 })
    .start();
```

### Callbacks
```javascript
new Animation(element)
    .smooth(400)
    .animate({ x: 200 })
    .on_complete(() => {
        console.log('Done!');
    })
    .start();
```

---

## 🎨 Shape Morphing

### SVG Path Interpolation
```javascript
import { PathMorph } from 'animation-engine';

const startPath = "M10 10 L90 10 L50 80 Z";
const endPath = "M10 80 L90 80 L50 10 Z";

const morph = new PathMorph(startPath, endPath);

// Manual control
const path = morph.updateProgress(0.5);
svgElement.setAttribute('d', path);

// With Animation
const anim = new Animation(element)
    .smooth(1000)
    .animate({ /* properties */ });

function updateMorph() {
    const path = morph.updateProgress(anim.get_fraction_complete());
    svgElement.setAttribute('d', path);
    if (anim.get_state() === 1) requestAnimationFrame(updateMorph);
}
anim.start();
updateMorph();
```

### Path Morphing Properties
```javascript
morph.progress = 0.5;           // Set directly
const current = morph.progress; // Get current
const path = morph.getPath();   // Get interpolated path
const pathAt = morph.getPathAt(0.75); // Get path at specific progress
```

---

## ✨ Particle System

### Basic Particle Emission
```javascript
import { ParticleEmitter } from 'animation-engine';

const emitter = new ParticleEmitter();

// Configure
emitter.setVelocity(0, -100);      // Initial velocity (vx, vy)
emitter.setVelocityVariance(50);   // Randomness
emitter.setGravity(200);            // Gravity force
emitter.setLifetime(2.0, 0.5);     // Life duration + variance
emitter.setMaxParticles(100);

// Emit particles
const particle = document.createElement('div');
particle.className = 'particle';
emitter.emit(particle, x, y);

// Update loop
function animate(time) {
    emitter.update(deltaTime);
    requestAnimationFrame(animate);
}
animate();
```

### Particle Presets
```javascript
import { ParticlePresets } from 'animation-engine';

// Built-in effects
const confetti = ParticlePresets.confetti();
const smoke = ParticlePresets.smoke();
const sparkle = ParticlePresets.sparkle();
const explosion = ParticlePresets.explosion();

// Use preset
button.addEventListener('click', (e) => {
    const particle = createParticle();
    confetti.emit(particle, e.clientX, e.clientY);
});
```

### Burst Effects
```javascript
// Emit multiple particles at once
emitter.emitBurst(particleElement, x, y, 20);

// Control emission
emitter.start();  // Enable emission
emitter.stop();   // Disable emission
emitter.clear();  // Remove all particles

// Query state
console.log(emitter.particleCount);
console.log(emitter.isActive);
console.log(emitter.maxParticles);
```

---

## 🔄 Animation Transactions

### Batch Multiple Animations
```javascript
import { Transaction } from 'animation-engine';

// Simple batch
Transaction.batch(0.5, () => {
    new Animation(el1).smooth(500).animate({ x: 200 }).start();
    new Animation(el2).smooth(500).animate({ y: 100 }).start();
    new Animation(el3).smooth(500).animate({ scale: 1.5 }).start();
});

// With completion
Transaction.batchWithCompletion(
    0.5,
    () => {
        // Animation block
        new Animation(el1).smooth(500).animate({ x: 200 }).start();
        new Animation(el2).smooth(500).animate({ y: 100 }).start();
    },
    () => {
        // Completion block
        console.log('All animations complete');
    }
);
```

### Manual Transaction Control
```javascript
const txn = new AnimationTransaction()
    .setDuration(0.5)
    .setTimingFunction(TimingFunction.EaseOut)
    .disableActions()
    .onComplete(() => console.log('Done'));

txn.begin();

// Execute animations within transaction
new Animation(el1).smooth(500).animate({ x: 200 }).start();
new Animation(el2).smooth(500).animate({ y: 100 }).start();

txn.commit();

// Query state
console.log(txn.duration);
console.log(txn.isActive);
console.log(txn.elapsedTime);
```

### Timing Functions
```javascript
TimingFunction.Default   // Apple's smooth curve
TimingFunction.Linear
TimingFunction.EaseIn
TimingFunction.EaseOut
TimingFunction.EaseInOut
```

---

## 💡 Complete Examples

### Interactive Modal
```javascript
const modal = new Animation(modalElement)
    .spring_smooth()
    .animate({ y: 0, opacity: 1 });

const gesture = new GestureController()
    .withSpring(Spring.bouncy());

gesture.connectAnimation(modal.start());

// Touch handlers
element.addEventListener('touchstart', (e) => {
    gesture.onTapDown(e.touches[0].clientX, e.touches[0].clientY, e.timeStamp);
});

element.addEventListener('touchmove', (e) => {
    gesture.onTapMove(e.touches[0].clientX, e.touches[0].clientY, e.timeStamp);
});

element.addEventListener('touchend', () => {
    gesture.onTapUp(); // Auto-completes or dismisses
});
```

### Staggered Sequence
```javascript
const sequencer = new Sequencer();
const cards = document.querySelectorAll('.card');

cards.forEach(card => {
    const anim = new Animation(card)
        .smooth(400)
        .animate({ y: 0, opacity: 1 })
        .start();
    
    sequencer.overlap(anim, 0.7); // Stagger at 70%
});

sequencer.play();
```

### Coordinated Transition
```javascript
const choreographer = new Choreographer(TransitionContext.Push);

const newView = new Animation(document.getElementById('new-view'))
    .smooth(350)
    .animate({ x: 0, opacity: 1 });

const oldView = new Animation(document.getElementById('old-view'))
    .smooth(350)
    .animate({ x: -100, opacity: 0.3 });

choreographer.addAnimation(newView.start());
choreographer.addAnimation(oldView.start());
```

---

## ⚡ Performance Tips

1. **Use transforms** - `x, y, scale, rotate` are GPU-accelerated
2. **Avoid layout properties** - `width, height` trigger reflow
3. **Enable will-change** - `will-change: transform` in CSS
4. **Batch updates** - Animate multiple properties together
5. **Use spring physics** - Better than bezier curves for gestures
6. **Throttle gesture updates** - Limit to 60fps

```css
.animated-element {
    will-change: transform, opacity;
    transform: translateZ(0); /* GPU layer */
}
```

---

## 📚 API Reference

### Animation
```javascript
new Animation(element)
    // Timing
    .smooth(duration)
    .snappy(duration)
    .bounce(duration)
    .ease_out(duration)
    .spring_smooth()
    
    // Properties
    .animate(config)
    .add_keyframe(config)
    
    // Options
    .repeat(count)
    .auto_reverse()
    .set_delay(ms)
    .additive()
    .continue_animate()
    
    // Control
    .start()
    .pause()
    .resume()
    .stop()
    .reverse()
    
    // Scrubbing
    .set_fraction_complete(fraction)
    .get_fraction_complete()
    .get_state()
    
    // Callbacks
    .on_complete(fn)
```

### GestureController
```javascript
new GestureController()
    .withSpring(spring)
    .connectAnimation(animation)
    
    .onTapDown(x, y, timestamp)
    .onTapMove(x, y, timestamp)
    .onTapUp()
    .onPress(pressed)
    .onHover(hovering)
    
    // Properties
    .fraction
    .velocity, .velocityX, .velocityY
    .displacement, .displacementX, .displacementY
    .isTracking
    .sensitivity
    .completionThreshold
```

### Spring
```javascript
Spring.default()   // (300, 30)
Spring.smooth()    // (400, 40)
Spring.bouncy()    // (250, 15)
Spring.snappy()    // (500, 35)
Spring.gentle()    // (200, 25)

new Spring(stiffness, damping)
    .update(target, deltaTime)
    .reset(value)
    .isAtRest(target, threshold)
    
    // Properties
    .position
    .velocity
    .stiffness
    .damping
```

### Choreographer
```javascript
new Choreographer(context)
    .addAnimation(animation)
    .beginInteractive()
    .updateInteractive(fraction)
    .finishInteractive(velocity)
    .cancelInteractive()
    
    // Properties
    .fraction
    .isInteractive
    .isCancelled
```

### Sequencer
```javascript
new Sequencer()
    .then(animation)
    .with(animation)
    .overlap(animation, at)
    .addStep(animation, overlap)
    
    .play()
    .pause()
    .stop()
    .seekTo(fraction)
    
    // Properties
    .totalDuration
    .stepCount
    .fraction
    .isRunning
```

### PathMorph
```javascript
new PathMorph(startPath, endPath)
    .updateProgress(fraction)
    .getPath()
    .getPathAt(fraction)
    
    // Properties
    .progress
```

### ParticleEmitter
```javascript
new ParticleEmitter()
    .setVelocity(vx, vy)
    .setVelocityVariance(variance)
    .setGravity(gravity)
    .setLifetime(duration, variance)
    .setMaxParticles(max)
    
    .emit(element, x, y)
    .emitBurst(element, x, y, count)
    .update(deltaTime)
    
    .start()
    .stop()
    .clear()
    
    // Properties
    .particleCount
    .isActive
    .maxParticles

// Presets
ParticlePresets.confetti()
ParticlePresets.smoke()
ParticlePresets.sparkle()
ParticlePresets.explosion()
```

---

## 🎨 Design Philosophy

This engine follows **Apple's animation principles**:

1. **Clarity** - Animations should clarify, not confuse
2. **Physicality** - Natural motion feels intuitive
3. **Responsiveness** - Immediate feedback to user input
4. **Delight** - Subtle moments that bring joy
5. **Consistency** - Predictable, learnable patterns

---

## 📄 License

MIT License - See LICENSE file for details

---

**Built with Rust + WebAssembly for maximum performance** 🦀⚡