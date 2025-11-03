# UI Animation Engine - WebAssembly

A high-performance, UIKit-inspired animation engine compiled to WebAssembly. Brings native iOS animation feel to web applications with GPU acceleration, spring physics, and gesture integration.

**Author:** RODDY064

---

## ✨ Features

- 🎯 **UIKit-Compatible API** - Familiar to iOS developers
- ⚡ **60-120 FPS Performance** - GPU-accelerated transforms
- 🎢 **Spring Physics** - Natural, organic motion
- 👆 **Gesture Integration** - Touch velocity tracking
- 🔄 **Animation Groups** - Synchronized animations
- 🎨 **Shape Morphing** - SVG path interpolation
- ✨ **Particle Effects** - Multi-element animations
- 🎯 **Keyframe Support** - Complex multi-step sequences
- 🎪 **Advanced Controls** - Pause, resume, reverse, scrub
- 🌊 **Additive Animations** - Layer effects smoothly

---

<!-- ## 📦 Installation

```bash
npm install animation-engine
# or
yarn add animation-engine
``` -->

---

## 🚀 Quick Start

### Basic Animation

```javascript
import init, { Animation } from 'animation-engine';

await init();

const element = document.getElementById('box');
new Animation(element)
    .smooth(400)
    .animate({ x: 200, opacity: 0.5 })
    .start();
```

### With HTML

```html
<!DOCTYPE html>
<html>
<head>
    <style>
        #box {
            width: 100px;
            height: 100px;
            background: #667eea;
            will-change: transform;
        }
    </style>
</head>
<body>
    <div id="box"></div>
    
    <script type="module">
        import init, { Animation } from './pkg/animation_engine.js';
        
        async function main() {
            await init();
            
            const box = document.getElementById('box');
            
            new Animation(box)
                .smooth(400)
                .animate({ x: 200, opacity: 0.5 })
                .start();
        }
        
        main();
    </script>
</body>
</html>
```

### Multiple Elements

```javascript
const elements = document.querySelectorAll('.box');

elements.forEach((element, index) => {
    new Animation(element)
        .smooth(400)
        .set_delay(index * 100)  // Stagger animations
        .animate({ x: 200, scale: 1.2 })
        .start();
});
```

---

## 📚 Table of Contents

1. [Basic Animation](#basic-animation)
2. [Timing Curves](#timing-curves)
3. [Animatable Properties](#animatable-properties)
4. [Animation Scrubbing](#animation-scrubbing)
5. [Keyframe Animations](#keyframe-animations)
6. [Gesture Velocity](#gesture-velocity)
7. [Additive Animations](#additive-animations)
8. [Completion Callbacks](#completion-callbacks)
9. [Playback Control](#playback-control)
10. [Spring Physics](#spring-physics)
11. [Advanced Features](#advanced-features)
12. [Animation Groups](#animation-groups)
13. [Shape Morphing](#shape-morphing)
14. [Conditional Animations](#conditional-animations)
15. [Transactions](#transactions)
16. [GPU Acceleration](#gpu-acceleration)
17. [Complete Examples](#complete-examples)
18. [Performance Tips](#performance-tips)
19. [API Reference](#api-reference)

---

## 🎬 Basic Animation

### Simple Property Animation

```javascript
new Animation(element)
    .smooth(400)
    .animate({ x: 200, opacity: 0.5 })
    .start();
```

### Chaining Properties

```javascript
new Animation(element)
    .smooth(600)
    .animate({
        x: 200,
        y: 100,
        scale: 1.5,
        opacity: 0.8
    })
    .start();
```

### Animation Flow

```javascript
new Animation(element)
    .smooth(400)                    // Step 1: Choose timing
    .animate({ x: 200 })            // Step 2: Define properties
    .start();                        // Step 3: Start animation
```

---

## 🎯 Timing Curves

Choose the right timing for your animation's feel and purpose.

### Available Curves

```javascript
.smooth(duration)      // Apple's premium curve (recommended)
.snappy(duration)      // Quick, responsive, punchy
.bounce(duration)      // Overshoot effect, playful
.ease_out(duration)    // Classic iOS, fast to slow
.ease_in(duration)     // Slow to fast, acceleration
.ease_in_out(duration) // S-curve, smooth both ends
.linear(duration)      // Constant speed, mechanical
.cubic(x1, y1, x2, y2, duration)  // Custom Bezier curve
```

### Duration Guidelines

```javascript
// Quick feedback (hover, tap)
.smooth(200)
.snappy(150)

// Standard transitions
.smooth(400)
.snappy(300)

// Hero animations
.smooth(800)
.ease_out(1200)

// Long sequences
.smooth(1500)
.bounce(2000)
```

### Custom Bezier Curves

```javascript
// Define custom easing
new Animation(element)
    .cubic(0.42, 0, 0.58, 1, 400)  // ease-in-out
    .animate({ x: 200 })
    .start();

// Common Bezier values
// ease-out: (0, 0, 0.58, 1)
// ease-in: (0.42, 0, 1, 1)
// ease-in-out: (0.42, 0, 0.58, 1)
```

---

## 🎨 Animatable Properties

### Transform Properties

```javascript
new Animation(element)
    .smooth(400)
    .animate({
        // Translation
        x: 200,              // translateX in pixels
        y: 100,              // translateY in pixels
        z: 0,                // translateZ in pixels (3D)
        
        // Scale
        scale: 1.5,          // uniform scale
        scale_x: 2.0,        // scale X only
        scale_y: 0.5,        // scale Y only
        
        // Rotation
        rotate: 180,         // rotate in degrees
        rotate_x: 45,        // rotateX in degrees (3D)
        rotate_y: 30,        // rotateY in degrees (3D)
        rotate_z: 90,        // rotateZ in degrees
        
        // Skew
        skew_x: 10,          // skewX in degrees
        skew_y: 5,           // skewY in degrees
    })
    .start();
```

### Visual Properties

```javascript
new Animation(element)
    .smooth(400)
    .animate({
        // Opacity & Visibility
        opacity: 0.5,                    // 0 to 1
        visibility: "hidden",            // "visible", "hidden", "collapse"
        
        // Colors
        background_color: "#667eea",
        color: "#000",
        border_color: "rgb(100, 200, 255)",
        
        // Sizing
        width: "300px",
        height: "200px",
        min_width: "100px",
        max_height: "500px",
        
        // Border & Radius
        border_radius: "20px",
        border_width: "2px",
    })
    .start();
```

### Filter Effects

```javascript
new Animation(element)
    .smooth(600)
    .animate({
        blur: 10,              // pixels (0-50)
        brightness: 1.5,       // 0-2
        contrast: 1.2,         // 0-2
        saturate: 0.8,         // 0-2
        hue: 180,              // degrees (0-360)
        grayscale: 0.5,        // 0-1
        invert: 0.3,           // 0-1
        sepia: 0.7,            // 0-1
    })
    .start();
```

### Advanced Transform Properties

```javascript
new Animation(element)
    .smooth(400)
    .animate({
        // Transform Origin
        transform_origin_x: "50%",
        transform_origin_y: "50%",
        transform_origin_z: "0",
        
        // Perspective
        perspective: 1000,
        perspective_origin_x: "50%",
        perspective_origin_y: "50%",
    })
    .start();
```

### Shadow Properties

```javascript
new Animation(element)
    .smooth(400)
    .animate({
        shadow_offset_x: 5,
        shadow_offset_y: 10,
        shadow_blur: 20,
        shadow_spread: 2,
        shadow_color: "rgba(0, 0, 0, 0.3)",
    })
    .start();
```

### SVG Properties

```javascript
new Animation(svgPath)
    .smooth(2000)
    .animate({
        stroke_dashoffset: 0,
        stroke_width: 3,
        fill_opacity: 1.0,
        stroke_opacity: 0.8,
    })
    .start();
```

---

## 🎮 Animation Scrubbing

Manually control animation progress like UIViewPropertyAnimator.

### Basic Scrubbing

```javascript
const anim = new Animation(element)
    .smooth(1000)
    .animate({ x: 200, rotate: 360 });

// Don't call .start() - control manually
anim.set_fraction_complete(0.0);   // Start
anim.set_fraction_complete(0.5);   // Jump to 50%
anim.set_fraction_complete(1.0);   // End
```

### Interactive Slider

```javascript
const slider = document.getElementById('progress-slider');
const element = document.getElementById('box');

const anim = new Animation(element)
    .smooth(1000)
    .animate({ x: 300, scale: 1.5 });

slider.addEventListener('input', (e) => {
    const progress = e.target.value / 100;
    anim.set_fraction_complete(progress);
});
```

### Timeline Scrubber

```javascript
const timeline = document.getElementById('timeline');
const element = document.getElementById('box');

const anim = new Animation(element)
    .smooth(5000)
    .animate({ x: 200, y: 100, scale: 2 });

document.addEventListener('mousemove', (e) => {
    const rect = timeline.getBoundingClientRect();
    const progress = (e.clientX - rect.left) / rect.width;
    anim.set_fraction_complete(Math.max(0, Math.min(1, progress)));
});
```

### Get Current Progress

```javascript
const progress = anim.get_fraction_complete();  // 0.0 to 1.0
console.log(`Animation is ${progress * 100}% complete`);

const state = anim.get_state();  
// 0 = Idle
// 1 = Running
// 2 = Paused
// 3 = Completed

if (state === 1) {
    console.log('Animation is currently running');
}
```

---

## 🔗 Keyframe Animations

Multi-step animations with precise control over each phase.

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

### Complex Multi-Property Animation

```javascript
new Animation(element)
    .smooth(2000)
    .add_keyframe({ 
        time: 0.0, 
        x: 0, 
        y: 0, 
        scale: 1, 
        opacity: 1,
        rotate: 0
    })
    .add_keyframe({ 
        time: 0.25, 
        x: 100, 
        y: -20, 
        scale: 1.5, 
        opacity: 0.5,
        rotate: 90
    })
    .add_keyframe({ 
        time: 0.5, 
        x: 200, 
        y: 0, 
        scale: 2, 
        opacity: 1,
        rotate: 180
    })
    .add_keyframe({ 
        time: 0.75, 
        x: 200, 
        y: 50, 
        scale: 0.8, 
        opacity: 1,
        rotate: 270
    })
    .add_keyframe({ 
        time: 1.0, 
        x: 0, 
        y: 0, 
        scale: 1, 
        opacity: 1,
        rotate: 360
    })
    .start();
```

### Keyframe Timing

```javascript
// time: 0.0 to 1.0 (relative to total duration)
// Each keyframe triggers at (time * duration)

new Animation(element)
    .smooth(2000)  // 2000ms total
    .add_keyframe({ time: 0.0, x: 0 })      // at 0ms
    .add_keyframe({ time: 0.25, x: 100 })   // at 500ms
    .add_keyframe({ time: 0.5, x: 200 })    // at 1000ms
    .add_keyframe({ time: 0.75, x: 100 })   // at 1500ms
    .add_keyframe({ time: 1.0, x: 0 })      // at 2000ms
    .start();
```

### Easing Between Keyframes

```javascript
// Easing applies BETWEEN keyframes, not individual keyframes
new Animation(element)
    .ease_out(1000)  // ease-out applies between all keyframes
    .add_keyframe({ time: 0.0, x: 0, y: 0 })
    .add_keyframe({ time: 0.5, x: 200, y: 200 })
    .add_keyframe({ time: 1.0, x: 0, y: 0 })
    .start();
```

### Batch Keyframes

```javascript
new Animation(element)
    .smooth(3000)
    .add_keyframes([
        { time: 0.0, x: 0, y: 0, scale: 1 },
        { time: 0.25, x: 100, y: 50, scale: 1.2 },
        { time: 0.5, x: 200, y: 0, scale: 1.4 },
        { time: 0.75, x: 100, y: -50, scale: 1.2 },
        { time: 1.0, x: 0, y: 0, scale: 1 },
    ])
    .start();
```

---

## 👆 Gesture Velocity

Track touch velocity and apply it to spring animations.

### Basic Gesture Handling

```javascript
import init, { Animation, GestureRecognizer } from 'animation-engine';

await init();

const gestureRecognizer = new GestureRecognizer();
const element = document.getElementById('draggable');

element.addEventListener('touchstart', (e) => {
    const touch = e.touches[0];
    gestureRecognizer.on_touch_start(
        touch.clientX, 
        touch.clientY, 
        e.timeStamp
    );
});

element.addEventListener('touchmove', (e) => {
    const touch = e.touches[0];
    gestureRecognizer.on_touch_move(
        touch.clientX, 
        touch.clientY, 
        e.timeStamp
    );
    
    const dx = gestureRecognizer.get_displacement_x();
    const dy = gestureRecognizer.get_displacement_y();
    
    // Update element position in real-time
    element.style.transform = `translate(${dx}px, ${dy}px)`;
});

element.addEventListener('touchend', (e) => {
    gestureRecognizer.on_touch_end();
    
    // Get velocity from gesture
    const vx = gestureRecognizer.get_velocity_x();
    const vy = gestureRecognizer.get_velocity_y();
    
    // Animate to rest with velocity
    new Animation(element)
        .spring_bouncy()
        .with_velocity('x', vx)
        .with_velocity('y', vy)
        .animate({ x: 0, y: 0 })
        .start();
});
```

### Mouse Gesture Tracking

```javascript
const gestureRecognizer = new GestureRecognizer();
const element = document.getElementById('draggable');

element.addEventListener('mousedown', (e) => {
    gestureRecognizer.on_touch_start(e.clientX, e.clientY, e.timeStamp);
});

document.addEventListener('mousemove', (e) => {
    if (gestureRecognizer.is_gesture_active()) {
        gestureRecognizer.on_touch_move(e.clientX, e.clientY, e.timeStamp);
        
        const dx = gestureRecognizer.get_displacement_x();
        const dy = gestureRecognizer.get_displacement_y();
        
        element.style.transform = `translate(${dx}px, ${dy}px)`;
    }
});

document.addEventListener('mouseup', (e) => {
    gestureRecognizer.on_touch_end();
    
    const vx = gestureRecognizer.get_velocity_x();
    const vy = gestureRecognizer.get_velocity_y();
    const distance = gestureRecognizer.get_distance();
    
    new Animation(element)
        .spring_smooth()
        .with_velocity('x', vx)
        .with_velocity('y', vy)
        .animate({ x: 0, y: 0 })
        .start();
});
```

### Per-Property Velocity

```javascript
new Animation(element)
    .spring_default()
    .with_velocity('x', 500)     // Fast horizontal motion
    .with_velocity('y', 100)     // Slow vertical motion
    .with_velocity('scale', 2)   // Quick scale change
    .animate({ x: 0, y: 0, scale: 1 })
    .start();
```

### Gesture Metrics

```javascript
const gestureRecognizer = new GestureRecognizer();

// Get displacement
const dx = gestureRecognizer.get_displacement_x();
const dy = gestureRecognizer.get_displacement_y();

// Get velocity (pixels per second)
const vx = gestureRecognizer.get_velocity_x();
const vy = gestureRecognizer.get_velocity_y();

// Get total distance
const distance = gestureRecognizer.get_distance();

// Check if gesture active
if (gestureRecognizer.is_gesture_active()) {
    console.log('Gesture in progress');
}
```

---

## 🔄 Additive Animations

Layer multiple animations smoothly without canceling.

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

// Result: Smooth diagonal motion + scale combined
```

### Layer Multiple Effects

```javascript
// Start motion
new Animation(element)
    .smooth(600)
    .additive()
    .animate({ x: 200 })
    .start();

// Add rotation while moving
setTimeout(() => {
    new Animation(element)
        .ease_out(800)
        .additive()
        .animate({ rotate: 360 })
        .start();
}, 100);

// Add fade while moving and rotating
setTimeout(() => {
    new Animation(element)
        .linear(500)
        .additive()
        .animate({ opacity: 0.5 })
        .start();
}, 200);

// Result: All three animations combine smoothly
```

### Complex Stacking

```javascript
// Base animation
new Animation(element)
    .smooth(400)
    .additive()
    .animate({ x: 200, y: 100 })
    .start();

// Layer 1
setTimeout(() => {
    new Animation(element)
        .snappy(300)
        .additive()
        .animate({ scale: 1.5 })
        .start();
}, 100);

// Layer 2
setTimeout(() => {
    new Animation(element)
        .bounce(500)
        .additive()
        .animate({ rotate: 45 })
        .start();
}, 200);

// Layer 3
setTimeout(() => {
    new Animation(element)
        .ease_in(600)
        .additive()
        .animate({ background_color: "#ff6b6b" })
        .start();
}, 300);
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

## ✅ Completion Callbacks

Chain and sequence animations with callbacks.

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

### Sequential Chaining

```javascript
new Animation(element)
    .smooth(400)
    .animate({ x: 200 })
    .on_complete(() => {
        new Animation(element)
            .bounce(500)
            .animate({ scale: 1.5 })
            .on_complete(() => {
                new Animation(element)
                    .smooth(300)
                    .animate({ x: 0, scale: 1 })
                    .start();
            })
            .start();
    })
    .start();
```

### Promise Wrapper

```javascript
function animateElement(element, config, duration) {
    return new Promise(resolve => {
        new Animation(element)
            .smooth(duration)
            .animate(config)
            .on_complete(resolve)
            .start();
    });
}

// Use with async/await
async function runSequence() {
    await animateElement(element, { x: 200 }, 400);
    console.log('Step 1 complete');
    
    await animateElement(element, { scale: 1.5 }, 300);
    console.log('Step 2 complete');
    
    await animateElement(element, { x: 0, scale: 1 }, 400);
    console.log('Step 3 complete');
}
```

### Array of Sequences

```javascript
const elements = document.querySelectorAll('.item');

async function animateAll() {
    for (const element of elements) {
        await new Promise(resolve => {
            new Animation(element)
                .smooth(300)
                .set_delay(100)
                .animate({ x: 100, opacity: 0.5 })
                .on_complete(resolve)
                .start();
        });
    }
    console.log('All animations complete');
}

animateAll();
```

---

## ⏯️ Playback Control

Full animation playback control.

### Play/Pause/Resume

```javascript
const handle = new Animation(element)
    .smooth(1000)
    .animate({ x: 200 })
    .start();  // Returns AnimationHandle

// Control playback
handle.pause();   // Pause at current position
handle.resume();  // Continue from where paused
handle.stop();    // Stop and mark as completed
handle.reverse(); // Smoothly reverse direction
```

### Animation State

```javascript
const handle = new Animation(element)
    .smooth(1000)
    .animate({ x: 200 })
    .start();

const state = handle.get_state();

// States:
// 0 = Idle (not started)
// 1 = Running (in progress)
// 2 = Paused (paused)
// 3 = Completed (finished)

if (state === 1) {
    console.log('Animation is running');
} else if (state === 2) {
    console.log('Animation is paused');
}
```

### Get/Set Progress

```javascript
const handle = new Animation(element)
    .smooth(1000)
    .animate({ x: 200 })
    .start();

// Get current progress (0.0 to 1.0)
const progress = handle.get_fraction_complete();

// Set progress
handle.set_fraction_complete(0.5);  // Jump to 50%
```

### Delayed Start

```javascript
new Animation(element)
    .smooth(400)
    .set_delay(500)  // Wait 500ms before starting
    .animate({ x: 200 })
    .start();
```

### Repeat with Auto-Reverse

```javascript
new Animation(element)
    .smooth(400)
    .repeat(3)       // Repeat 3 times
    .auto_reverse()  // Alternate direction each repeat
    .animate({ x: 200 })
    .start();

// Animation plays: forward -> reverse -> forward
```

### Infinite Repeat

```javascript
new Animation(element)
    .smooth(400)
    .repeat(-1)      // Negative = infinite
    .auto_reverse()
    .animate({ x: 200 })
    .start();
```

---

## 🌊 Spring Physics

Natural, physics-based motion.

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

### Custom Spring Parameters

```javascript
new Animation(element)
    .spring(300, 30)  // stiffness, damping
    .animate({ x: 200 })
    .start();
```

### Spring Parameter Guide

**Stiffness** (typical: 200-500)
```javascript
// Soft (slow response)
.spring(100, 30)

// Balanced (iOS default)
.spring(300, 30)

// Stiff (quick response)
.spring(500, 30)
```

**Damping** (typical: 15-50)
```javascript
// Bouncy (multiple overshoots)
.spring(300, 10)

// Moderate bounce (one overshoot)
.spring(300, 30)

// No overshoot (critically damped)
.spring(300, 50)
```

### Spring vs Bezier

```javascript
// USE SPRING FOR:
// - User-initiated interactions (tap, drag, swipe)
// - Natural, organic feel
// - Variable duration acceptable
// - Velocity from gestures
new Animation(element)
    .spring_bouncy()
    .with_velocity('x', gestureVelocity)
    .animate({ x: 0 })
    .start();

// USE BEZIER FOR:
// - Precise timing required
// - Fixed duration needed
// - System-initiated animations
// - Simple fade/scale effects
new Animation(element)
    .smooth(400)  // Fixed duration
    .animate({ opacity: 0 })
    .start();
```

---

## 🎨 Advanced Features

### Animation Groups (CAAnimationGroup)

Group multiple animations to control them together.

```javascript
import init, { Animation, AnimationGroup } from 'animation-engine';

await init();

const group = new AnimationGroup("my-animations");

// Create animations
const box1 = document.getElementById('box1');
const box2 = document.getElementById('box2');

const anim1 = new Animation(box1)
    .smooth(1000)
    .animate({ x: 200 })
    .start();

const anim2 = new Animation(box2)
    .snappy(1000)
    .animate({ scale: 1.5 })
    .start();

// Add to group
group.add_animation(anim1);
group.add_animation(anim2);

// Control all together
group.play();      // Start all
group.pause();     // Pause all
group.resume();    // Resume all
group.reverse();   // Reverse all
group.stop();      // Stop all

// Get info
const count = group.get_animation_count();
const isPlaying = group.is_playing_group();
const id = group.get_group_id();
```

### Shape Morphing

Animate between SVG paths smoothly.

```javascript
import init, { PathMorph } from 'animation-engine';

await init();

const morph = new PathMorph(
    "M100 100 L150 200 L50 200 Z",  // Start shape
    "M100 50 L170 100 L130 200 Z"   // End shape
);

const svgPath = document.querySelector('svg path');
let progress = 0;

const interval = setInterval(() => {
    progress += 0.016;  // ~60fps
    
    const newPath = morph.update_progress(progress);
    svgPath.setAttribute('d', newPath);
    
    if (progress >= 1.0) {
        clearInterval(interval);
    }
}, 16);

// Get current state
const currentPath = morph.get_current_path();
const currentProgress = morph.get_progress();
```

### Conditional Animations

### If/Else Animation

```javascript
new Animation(element)
    .animate_if(
        isLoggedIn,  // condition
        { x: 200, opacity: 1 },      // true case
        { x: 0, opacity: 0 }         // false case
    )
    .smooth(400)
    .start();
```

### Switch/Case Animation

```javascript
new Animation(element)
    .animate_match(
        userState,  // value to match
        {
            "0": { x: 0, y: 0 },           // idle
            "1": { x: 100, y: 50 },        // active
            "2": { x: 0, y: -100 }         // disabled
        }
    )
    .smooth(300)
    .start();

// Also works with string values
new Animation(element)
    .animate_match(
        theme,
        {
            "light": { opacity: 1, background_color: "#fff" },
            "dark": { opacity: 0.8, background_color: "#000" }
        }
    )
    .smooth(300)
    .start();
```

### Ternary Animation

```javascript
new Animation(element)
    .animate_ternary(
        isDark,  // condition
        1.0,     // true value
        0.5,     // false value
        "opacity"  // property
    )
    .smooth(300)
    .start();
```

---

## 💼 Transactions (CATransaction)

Batch animations with unified timing.

```javascript
import init, { Animation, AnimationTransaction } from 'animation-engine';

await init();

const transaction = new AnimationTransaction();
transaction.set_duration(500);
transaction.begin();

const box1 = document.getElementById('box1');
const box2 = document.getElementById('box2');

// Create animations with transaction duration
new Animation(box1)
    .smooth(transaction.get_duration())
    .animate({ x: 200 })
    .start();

new Animation(box2)
    .smooth(transaction.get_duration())
    .animate({ scale: 1.5 })
    .start();

// Set completion
transaction.set_completion_callback(() => {
    console.log('Transaction complete');
});

// Finalize
transaction.commit();

// Other transaction methods
transaction.disable_actions();  // Disable implicit animations
transaction.enable_actions();   // Re-enable
const disabled = transaction.are_actions_disabled();
const id = transaction.get_transaction_id();
const active = transaction.is_active();
```

---

## ⚡ GPU Acceleration

Optimize for high-performance animations.

```javascript
import init, { Animation, GPUAccelerator } from 'animation-engine';

await init();

const accelerator = new GPUAccelerator();

// Check support
if (accelerator.is_supported()) {
    console.log('GPU acceleration available');
    
    accelerator.enable();
    accelerator.set_optimization_level(3);  // 0-3, higher = more aggressive
    
    const element = document.getElementById('fast-animation');
    
    // Apply GPU hints
    accelerator.apply_gpu_hints(element);
    
    // Run animation
    new Animation(element)
        .smooth(1000)
        .animate({ x: 500, rotate: 360 })
        .start();
    
    // Clean up when done
    setTimeout(() => {
        accelerator.remove_gpu_hints(element);
    }, 1000);
} else {
    console.log('GPU acceleration not supported, using CPU');
}

// Check state
const enabled = accelerator.is_enabled();
const level = accelerator.get_optimization_level();
```

---

## 🎪 Complete Examples

### Interactive Card Component

```javascript
const card = document.querySelector('.card');

// Hover enter
card.addEventListener('mouseenter', () => {
    new Animation(card)
        .snappy(200)
        .additive()
        .animate({ scale: 1.05, y: -10 })
        .start();
});

// Hover exit
card.addEventListener('mouseleave', () => {
    new Animation(card)
        .smooth(300)
        .additive()
        .animate({ scale: 1, y: 0 })
        .start();
});

// Mouse down (pressed)
card.addEventListener('mousedown', () => {
    new Animation(card)
        .snappy(100)
        .additive()
        .animate({ scale: 0.95 })
        .start();
});

// Mouse up (released)
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
    
    const backdrop = modal.querySelector('.backdrop');
    const content = modal.querySelector('.content');
    
    // Fade in backdrop
    new Animation(backdrop)
        .linear(200)
        .animate({ opacity: 1 })
        .start();
    
    // Slide up content with spring
    new Animation(content)
        .spring_smooth()
        .animate({ y: 0, opacity: 1 })
        .start();
}

function hideModal(modal) {
    const content = modal.querySelector('.content');
    
    new Animation(content)
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
const pathLength = path.getTotalLength();

// Initial state
path.style.strokeDasharray = pathLength;
path.style.strokeDashoffset = pathLength;

// Animate draw
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

### Page Transition

```javascript
async function transitionToPage(nextPage) {
    const currentPage = document.querySelector('.page.active');
    
    // Fade out current
    await new Promise(resolve => {
        new Animation(currentPage)
            .smooth(300)
            .animate({ opacity: 0 })
            .on_complete(resolve)
            .start();
    });
    
    // Swap pages
    currentPage.classList.remove('active');
    nextPage.classList.add('active');
    nextPage.style.opacity = '0';
    
    // Fade in new
    new Animation(nextPage)
        .smooth(300)
        .animate({ opacity: 1 })
        .start();
}
```

### Draggable Card with Physics

```javascript
const card = document.querySelector('.card');
const gestureRecognizer = new GestureRecognizer();
let isDragging = false;

card.addEventListener('mousedown', (e) => {
    isDragging = true;
    gestureRecognizer.on_touch_start(e.clientX, e.clientY, e.timeStamp);
});

document.addEventListener('mousemove', (e) => {
    if (!isDragging) return;
    
    gestureRecognizer.on_touch_move(e.clientX, e.clientY, e.timeStamp);
    
    const dx = gestureRecognizer.get_displacement_x();
    const dy = gestureRecognizer.get_displacement_y();
    
    card.style.transform = `translate(${dx}px, ${dy}px) rotate(${dx * 0.1}deg)`;
});

document.addEventListener('mouseup', (e) => {
    if (!isDragging) return;
    isDragging = false;
    
    gestureRecognizer.on_touch_end();
    
    const vx = gestureRecognizer.get_velocity_x();
    const vy = gestureRecognizer.get_velocity_y();
    
    new Animation(card)
        .spring_bouncy()
        .with_velocity('x', vx)
        .with_velocity('y', vy)
        .animate({ x: 0, y: 0, rotate: 0 })
        .start();
});
```

### Loader Animation

```javascript
function createLoaderAnimation() {
    const loader = document.querySelector('.loader');
    
    return new Animation(loader)
        .linear(1000)
        .repeat(-1)  // Infinite
        .animate({ rotate: 360 })
        .start();
}

// Use it
const loaderHandle = createLoaderAnimation();

// Stop when ready
setTimeout(() => {
    loaderHandle.stop();
}, 5000);
```

### Staggered List Items

```javascript
const items = document.querySelectorAll('.list-item');

items.forEach((item, index) => {
    new Animation(item)
        .smooth(600)
        .set_delay(index * 100)  // Stagger by 100ms
        .animate({ x: 0, opacity: 1 })
        .start();
});
```

---

## ⚡ Performance Tips

### ✅ DO Use

```javascript
// Transform properties (GPU accelerated)
.animate({ x: 200, y: 100, scale: 1.5, rotate: 45 })

// Opacity (very performant)
.animate({ opacity: 0.5 })

// Batch properties in one animation
new Animation(element)
    .smooth(400)
    .animate({ x: 200, y: 100, scale: 1.5 })
    .start();

// Springs for natural motion
.spring_bouncy()

// will-change CSS
```

### ❌ DON'T Use

```javascript
// Layout properties (cause reflow)
.animate({ width: 300, height: 200 })

// Position properties
.animate({ left: 200, top: 100 })  // Use x, y instead

// Hundreds of simultaneous animations
for (let i = 0; i < 1000; i++) {
    new Animation(elements[i]).animate({...}).start();
}

// Shadow animations
.animate({ box_shadow: "..." })  // Pre-render instead

// Blocking operations during animation
```

### CSS Optimization

```css
.animated-element {
    /* Enable GPU layer */
    will-change: transform;
    
    /* Prevent layout thrashing */
    backface-visibility: hidden;
    
    /* Force GPU rendering */
    transform: translateZ(0);
    
    /* Contain layout calculations */
    contain: layout style paint;
    
    /* Modern browsers: optimize rendering */
    transform-origin: center;
    box-sizing: border-box;
}
```

### Performance Monitoring

```javascript
const startTime = performance.now();

const handle = new Animation(element)
    .smooth(1000)
    .animate({ x: 500, scale: 2 })
    .on_complete(() => {
        const endTime = performance.now();
        console.log(`Animation took ${endTime - startTime}ms`);
    })
    .start();
```

---

## 🌐 Browser Support

| Browser | Version | Support |
|---------|---------|---------|
| Chrome | v90+ | ✅ Full |
| Edge | v90+ | ✅ Full |
| Firefox | v88+ | ✅ Full |
| Safari | v14+ | ✅ Full |
| iOS Safari | v14+ | ✅ Full |
| Chrome Mobile | Latest | ✅ Full |
| Firefox Android | Latest | ✅ Full |

---

## 📖 API Reference

### Animation Class

```javascript
new Animation(element)
    .smooth(duration)
    .snappy(duration)
    .bounce(duration)
    .ease_out(duration)
    .ease_in(duration)
    .ease_in_out(duration)
    .linear(duration)
    .cubic(x1, y1, x2, y2, duration)
    .spring(stiffness, damping)
    .spring_default()
    .spring_bouncy()
    .spring_smooth()
    .animate(config)
    .add_keyframe(config)
    .add_keyframes(configs)
    .set_delay(delay)
    .set_transform_origin(x, y, z)
    .add_shadow_layer(offsetX, offsetY, blur, spread, color, inset)
    .repeat(count)
    .auto_reverse()
    .additive()
    .on_complete(callback)
    .with_velocity(property, velocity)
    .animate_if(condition, trueConfig, falseConfig)
    .animate_match(value, cases)
    .animate_ternary(condition, trueVal, falseVal, property)
    .start()  // Returns AnimationHandle
```

### AnimationHandle Class

```javascript
const handle = animation.start();

handle.pause()
handle.resume()
handle.stop()
handle.reverse()
handle.set_fraction_complete(fraction)
handle.get_fraction_complete()
handle.get_state()  // 0=Idle, 1=Running, 2=Paused, 3=Completed
```

### AnimationGroup Class

```javascript
const group = new AnimationGroup(id);

group.add_animation(handle)
group.play()
group.pause()
group.resume()
group.stop()
group.reverse()
group.get_animation_count()
group.is_playing_group()
group.get_group_id()
```

### GestureRecognizer Class

```javascript
const gesture = new GestureRecognizer();

gesture.on_touch_start(x, y, timestamp)
gesture.on_touch_move(x, y, timestamp)
gesture.on_touch_end()
gesture.get_displacement_x()
gesture.get_displacement_y()
gesture.get_velocity_x()
gesture.get_velocity_y()
gesture.get_distance()
gesture.is_gesture_active()
```

### PathMorph Class

```javascript
const morph = new PathMorph(startPath, endPath);

morph.update_progress(progress)  // 0.0 to 1.0
morph.get_current_path()
morph.get_progress()
```

### AnimationTransaction Class

```javascript
const transaction = new AnimationTransaction();

transaction.set_duration(duration)
transaction.get_duration()
transaction.disable_actions()
transaction.enable_actions()
transaction.are_actions_disabled()
transaction.set_completion_callback(callback)
transaction.begin()
transaction.commit()
transaction.get_transaction_id()
transaction.is_active()
```

### GPUAccelerator Class

```javascript
const accelerator = new GPUAccelerator();

accelerator.is_supported()
accelerator.enable()
accelerator.disable()
accelerator.is_enabled()
accelerator.set_optimization_level(level)  // 0-3
accelerator.get_optimization_level()
accelerator.apply_gpu_hints(element)
accelerator.remove_gpu_hints(element)
```

---

## 🤝 Contributing

Contributions welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## 📄 License

MIT © 2025 RODDY064

---

## 📚 Additional Resources

- [Apple UIView Animation Docs](https://developer.apple.com/documentation/uikit/uiview)
- [Web Animations API](https://www.w3.org/TR/web-animations-1/)
- [WebAssembly](https://webassembly.org/)
- [Cubic Bezier Generator](https://cubic-bezier.com/)

---

## 🎯 Roadmap

- [ ] Touch event advanced gestures (pan, pinch, rotate)
- [ ] Constraint-based animations
- [ ] Custom animation types
- [ ] Timeline/sequencer UI
- [ ] Snapshot and playback
- [ ] Physics-based collision detection
- [ ] Advanced particle systems

---

**Built with ❤️ for developers who love smooth animations**

**2025 - RODDY064**