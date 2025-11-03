# UI Animation API Guide


---

## Table of Contents
1. [Basic Animation](#basic-animation)
2. [Animation Scrubbing](#animation-scrubbing)
3. [Keyframe Animations](#keyframe-animations)
4. [Gesture Velocity](#gesture-velocity)
5. [Additive Animations](#additive-animations)
6. [Completion Callbacks](#completion-callbacks)
7. [Playback Control](#playback-control)
8. [Spring Physics](#spring-physics)

---

## Basic Animation

### Simple Property Animation
```javascript
new Animation(element)
    .smooth(400)
    .animate({ x: 200, opacity: 0.5 })
    .start();
```

### Available Timing Curves
```javascript
.smooth(duration)      // Apple's premium curve (recommended)
.snappy(duration)      // Quick, responsive
.bounce(duration)      // Overshoot effect
.ease_out(duration)    // Classic iOS
.ease_in(duration)     // Acceleration
.ease_in_out(duration) // S-curve
.linear(duration)      // Constant speed
```

### Animatable Properties
```javascript
{
    // Transform
    x: 200,              // translate X
    y: 100,              // translate Y
    z: 0,                // translate Z
    scale: 1.5,          // uniform scale
    scale_x: 2.0,        // scale X only
    scale_y: 0.5,        // scale Y only
    rotate: 180,         // rotation in degrees
    
    // Style
    opacity: 0.5,
    width: "300px",
    height: "200px",
    background_color: "#667eea",
    border_radius: "20px",
    
    // SVG
    stroke_dashoffset: 0,
    stroke_width: 3
}
```

---

## Animation Scrubbing

**UIViewPropertyAnimator equivalent** - manually control animation progress.

### Basic Scrubbing
```javascript
const anim = new Animation(element)
    .smooth(1000)
    .animate({ x: 200, rotate: 360 });

// Don't call .start() - control manually
anim.set_fraction_complete(0.5);  // Jump to 50%
```

### Interactive Slider
```javascript
const slider = document.getElementById('slider');
const anim = new Animation(element)
    .smooth(1000)
    .animate({ x: 300, scale: 1.5 });

slider.addEventListener('input', (e) => {
    const progress = e.target.value / 100;
    anim.set_fraction_complete(progress);
});
```

### Get Current Progress
```javascript
const progress = anim.get_fraction_complete(); // Returns 0.0 to 1.0
```

---

## Keyframe Animations

**UIView.animateKeyframes equivalent** - multi-step animations.

### Triangle Path
```javascript
new Animation(element)
    .smooth(1500)
    .add_keyframe({ time: 0.0, x: 0, y: 0 })
    .add_keyframe({ time: 0.33, x: 200, y: 0 })
    .add_keyframe({ time: 0.66, x: 100, y: 100 })
    .add_keyframe({ time: 1.0, x: 0, y: 0 })
    .start();
```

### Complex Animation
```javascript
new Animation(element)
    .smooth(2000)
    .add_keyframe({ 
        time: 0.0, 
        x: 0, 
        y: 0, 
        scale: 1, 
        opacity: 1 
    })
    .add_keyframe({ 
        time: 0.25, 
        x: 100, 
        y: -20, 
        scale: 1.5, 
        opacity: 0.5 
    })
    .add_keyframe({ 
        time: 0.75, 
        x: 200, 
        y: 50, 
        scale: 0.8, 
        opacity: 1 
    })
    .add_keyframe({ 
        time: 1.0, 
        x: 0, 
        y: 0, 
        scale: 1, 
        opacity: 1 
    })
    .start();
```

### Keyframe Timing
- `time`: 0.0 to 1.0 (relative to total duration)
- Easing curve applies **between** keyframes
- Automatically interpolates all properties

---

## Gesture Velocity

**iOS native behavior** - spring animations inherit swipe velocity.

### Basic Setup
```javascript
let isDragging = false;
let velocityX = 0;

element.addEventListener('mousedown', (e) => {
    isDragging = true;
    // Track velocity during drag
});

element.addEventListener('mouseup', () => {
    // Spring back with gesture velocity
    new Animation(element)
        .spring_bouncy()
        .with_velocity('x', velocityX / 100)  // Scale to reasonable range
        .with_velocity('y', velocityY / 100)
        .animate({ x: 0, y: 0 })
        .start();
});
```

### Velocity Calculation
```javascript
let lastX = 0, lastTime = 0;

document.addEventListener('mousemove', (e) => {
    if (!isDragging) return;
    
    const deltaTime = Date.now() - lastTime;
    if (deltaTime > 0) {
        // Velocity in pixels per second
        velocityX = (e.clientX - lastX) / deltaTime * 1000;
    }
    
    lastX = e.clientX;
    lastTime = Date.now();
});
```

### Per-Property Velocity
```javascript
new Animation(element)
    .spring_default()
    .with_velocity('x', 500)     // Fast horizontal
    .with_velocity('y', 100)     // Slow vertical
    .with_velocity('scale', 2)   // Quick scale change
    .animate({ x: 0, y: 0, scale: 1 })
    .start();
```

---

## Additive Animations

**UIKit default behavior** - multiple animations combine smoothly instead of canceling.

### Basic Additive
```javascript
// Animation 1: Move right
new Animation(element)
    .smooth(500)
    .additive()
    .animate({ x: 200 })
    .start();

// Animation 2: Scale up (doesn't cancel #1)
setTimeout(() => {
    new Animation(element)
        .snappy(300)
        .additive()
        .animate({ scale: 1.5 })
        .start();
}, 200);

// Result: Smooth diagonal motion + scale
```

### Layer Multiple Effects
```javascript
// Move
new Animation(element).smooth(600).additive()
    .animate({ x: 200 }).start();

// Rotate (combines with move)
new Animation(element).ease_out(800).additive()
    .animate({ rotate: 360 }).start();

// Fade (combines with both)
new Animation(element).linear(500).additive()
    .animate({ opacity: 0.5 }).start();
```

### Non-Additive (Override)
```javascript
// This REPLACES any existing animation
new Animation(element)
    .smooth(400)
    // Don't call .additive()
    .animate({ x: 200 })
    .start();
```

---

## Completion Callbacks

**Chain animations** like UIKit completion handlers.

### Basic Callback
```javascript
new Animation(element)
    .smooth(400)
    .animate({ x: 200 })
    .on_complete(() => {
        console.log('Animation finished!');
    })
    .start();
```

### Chaining Animations
```javascript
new Animation(element)
    .smooth(400)
    .animate({ x: 200 })
    .on_complete(() => {
        // Start next animation
        new Animation(element)
            .bounce(500)
            .animate({ scale: 1.5 })
            .on_complete(() => {
                // And another
                new Animation(element)
                    .smooth(300)
                    .animate({ x: 0, scale: 1 })
                    .start();
            })
            .start();
    })
    .start();
```

### Complex Sequence
```javascript
function animateSequence(element) {
    return new Promise(resolve => {
        new Animation(element)
            .smooth(400)
            .animate({ x: 200 })
            .on_complete(resolve)
            .start();
    });
}

// Use with async/await
async function run() {
    await animateSequence(element);
    console.log('Step 1 done');
    
    await animateSequence(element);
    console.log('Step 2 done');
}
```

---

## Playback Control

**UIViewPropertyAnimator equivalent** - full playback control.

### Play/Pause/Resume
```javascript
const anim = new Animation(element)
    .smooth(1000)
    .animate({ x: 200 });

anim.start();        // Begin animation
anim.pause();        // Pause at current position
anim.resume();       // Continue from where paused
anim.stop();         // Stop and mark as completed
```

### Reverse Mid-Flight
```javascript
const anim = new Animation(element)
    .smooth(1000)
    .animate({ x: 200 });

anim.start();

setTimeout(() => {
    anim.reverse();  // Smoothly reverse direction
}, 300);
```

### Check Animation State
```javascript
const state = anim.get_state();

// States:
// 0 = Idle
// 1 = Running
// 2 = Paused
// 3 = Completed

if (state === 1) {
    console.log('Animation is running');
}
```

### Delayed Start
```javascript
new Animation(element)
    .smooth(400)
    .set_delay(500)  // Wait 500ms before starting
    .animate({ x: 200 })
    .start();
```

---

## Spring Physics

**UISpringTimingParameters equivalent** - natural, physics-based motion.

### Preset Springs
```javascript
// Balanced (default)
new Animation(element)
    .spring_default()
    .animate({ x: 200 })
    .start();

// Bouncy (more oscillation)
new Animation(element)
    .spring_bouncy()
    .animate({ x: 200 })
    .start();

// Smooth (minimal bounce)
new Animation(element)
    .spring_smooth()
    .animate({ x: 200 })
    .start();
```

### Custom Spring
```javascript
new Animation(element)
    .spring(
        300,  // stiffness (higher = faster)
        30    // damping (higher = less bounce)
    )
    .animate({ x: 200 })
    .start();
```

### Spring Parameters Guide

**Stiffness** (typical: 200-500)
- 100: Very soft, slow response
- 300: Balanced, iOS default
- 500: Stiff, quick response

**Damping** (typical: 15-50)
- 10: Very bouncy (multiple overshoots)
- 30: Moderate bounce (one overshoot)
- 50: Critically damped (no overshoot)

### Spring vs Bezier

**Use Springs When:**
- User-initiated (tap, drag, swipe)
- Natural, organic feel needed
- Variable duration acceptable
- Velocity from gestures

**Use Bezier When:**
- Precise timing required
- Fixed duration needed
- System-initiated animations
- Simple fade/scale effects

---

## Complete Examples

### Interactive Card
```javascript
const card = document.querySelector('.card');

// Hover
card.addEventListener('mouseenter', () => {
    new Animation(card)
        .snappy(200)
        .animate({ scale: 1.05, y: -10 })
        .start();
});

card.addEventListener('mouseleave', () => {
    new Animation(card)
        .smooth(300)
        .animate({ scale: 1, y: 0 })
        .start();
});

// Click
card.addEventListener('mousedown', () => {
    new Animation(card)
        .snappy(100)
        .animate({ scale: 0.95 })
        .start();
});

card.addEventListener('mouseup', () => {
    new Animation(card)
        .spring_bouncy()
        .animate({ scale: 1.05 })
        .start();
});
```

### Modal Presentation
```javascript
function showModal(modal) {
    modal.style.display = 'block';
    
    // Backdrop fade in
    new Animation(modal.querySelector('.backdrop'))
        .linear(200)
        .animate({ opacity: 1 })
        .start();
    
    // Modal slide up with spring
    new Animation(modal.querySelector('.content'))
        .spring_smooth()
        .animate({ y: 0, opacity: 1 })
        .start();
}

function hideModal(modal) {
    new Animation(modal.querySelector('.content'))
        .smooth(250)
        .animate({ y: 100, opacity: 0 })
        .on_complete(() => {
            modal.style.display = 'none';
        })
        .start();
}
```

### SVG Logo Reveal
```javascript
const path = document.querySelector('svg path');
const pathLength = 1000;

// Set initial state
path.style.strokeDasharray = pathLength;
path.style.strokeDashoffset = pathLength;

// Animate reveal
new Animation(path)
    .ease_out(2000)
    .set_delay(500)
    .animate({ stroke_dashoffset: 0 })
    .on_complete(() => {
        // Fill after draw
        new Animation(path)
            .smooth(400)
            .animate({ fill_opacity: 1 })
            .start();
    })
    .start();
```

---

## Performance Tips

### ✅ DO
- Use `transform` properties (x, y, scale, rotate)
- Use `opacity`
- Batch property changes in single animation
- Use springs for natural motion
- Add `will-change: transform` to CSS

### ❌ DON'T
- Animate `width`/`height` (use `scale` instead)
- Animate `left`/`top` (use `x`/`y` instead)
- Create hundreds of simultaneous animations
- Animate shadows (pre-render as image)

### CSS Setup
```css
.animated-element {
    will-change: transform;
    backface-visibility: hidden;
    transform: translateZ(0); /* Force GPU layer */
}
```

---

## Browser Support

- **Chrome/Edge**: Full support
- **Firefox**: Full support
- **Safari**: Full support (native iOS feel)
- **Mobile**: Optimized for 120Hz displays

---
**Result**: Production-ready animations in the browser! 🎉