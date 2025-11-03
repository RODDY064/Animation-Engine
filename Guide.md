### Interactive Modal with Spring Physics
```javascript
// Setup with bouncy spring for natural feel
const modal = new Animation(modalElement)
    .spring_smooth()
    .animate({ y: 0, opacity: 1 });

const gesture = new GestureController()
    .withSpring(Spring.bouncy());
gesture.sensitivity = 500;
gesture.completionThreshold = 0.4;

const choreographer# Apple Animation Engine - Integration Guide

Clean, powerful animations with Apple's design philosophy: **simplicity meets capability**.

---

## Core Concepts

### 1. **Animation** — Your Foundation
The base animation system with full property control.

### 2. **GestureController** — Touch-Driven Control
UIKit-inspired gesture handling for interactive animations.

### 3. **Choreographer** — Transition Coordination
Coordinates multiple animations for presentations and dismissals.

### 4. **Sequencer** — Timeline Management
SwiftUI-inspired declarative timeline with overlap control.

---

## Basic Usage

### Simple Animation
```rust
// In Rust
let element = document.get_element_by_id("box")?;
let animation = Animation::new(element)?
    .smooth(400.0)
    .animate(config)?
    .start()?;
```

```javascript
// In JavaScript
const element = document.getElementById('box');
const anim = new Animation(element)
    .smooth(400)
    .animate({ x: 100, opacity: 0.5 })
    .start();
```

---

## Gesture-Driven Animations

### Interactive Swipe Control
```javascript
// Create animation
const element = document.getElementById('card');
const animation = new Animation(element)
    .smooth(600)
    .animate({ y: -400, opacity: 0 })
    .start();

// Create gesture controller with spring physics
const gesture = new GestureController()
    .withSpring(Spring.smooth());
gesture.connectAnimation(animation);

// Configure behavior
gesture.sensitivity = 500;           // Pixels for full range (default: 500)
gesture.completionThreshold = 0.5;   // Complete if past 50%

// Wire up touch events
element.addEventListener('touchstart', (e) => {
    gesture.onTapDown(
        e.touches[0].clientX,
        e.touches[0].clientY,
        performance.now()
    );
});

element.addEventListener('touchmove', (e) => {
    gesture.onTapMove(
        e.touches[0].clientX,
        e.touches[0].clientY,
        performance.now()
    );
});

element.addEventListener('touchend', () => {
    gesture.onTapUp(); // Auto-completes or cancels
});

// Monitor gesture state
console.log(`Fraction: ${gesture.fraction}`);
console.log(`Velocity: ${gesture.velocityY}`);
console.log(`Tracking: ${gesture.isTracking}`);
```

### Advanced Gesture Control
```javascript
// Bouncy dismiss gesture
const bouncyGesture = new GestureController()
    .withSpring(Spring.bouncy());
bouncyGesture.sensitivity = 400;
bouncyGesture.completionThreshold = 0.4; // Easier to dismiss

// Snappy confirmation gesture
const snappyGesture = new GestureController()
    .withSpring(Spring.snappy());
snappyGesture.sensitivity = 300;
snappyGesture.completionThreshold = 0.6; // Harder to confirm

// Direct control (no spring smoothing)
const directGesture = new GestureController()
    .withoutSpring();
directGesture.sensitivity = 600;
```

### Press & Hover Effects
```javascript
const gesture = new GestureController();

button.addEventListener('mousedown', () => {
    const scale = gesture.onPress(true); // Returns 0.95
    // Apply scale to element
});

button.addEventListener('mouseup', () => {
    const scale = gesture.onPress(false); // Returns 1.0
});

button.addEventListener('mouseenter', () => {
    const scale = gesture.onHover(true); // Returns 1.05
});

button.addEventListener('mouseleave', () => {
    const scale = gesture.onHover(false); // Returns 1.0
});
```

---

## Choreographed Transitions

### Modal Presentation
```javascript
// Setup animations
const modal = new Animation(document.getElementById('modal'))
    .smooth(400)
    .animate({ y: 0, opacity: 1 });

const overlay = new Animation(document.getElementById('overlay'))
    .smooth(400)
    .animate({ opacity: 0.5 });

// Create choreographer
const choreographer = new Choreographer(0); // 0 = Present
choreographer.addAnimation(modal.start());
choreographer.addAnimation(overlay.start());

// For interactive drag-to-dismiss
choreographer.beginInteractive();

// During drag
function onDrag(fraction) {
    choreographer.updateInteractive(fraction);
}

// On release
function onRelease(velocity) {
    const completed = choreographer.finishInteractive(velocity);
    if (completed) {
        console.log('Modal presented');
    } else {
        console.log('Modal dismissed');
    }
}
```

### Navigation Push/Pop
```javascript
// Push transition
const pushChoreographer = new Choreographer(2); // 2 = Push

const newView = new Animation(document.getElementById('new-view'))
    .smooth(350)
    .animate({ x: 0, opacity: 1 });

const oldView = new Animation(document.getElementById('old-view'))
    .smooth(350)
    .animate({ x: -100, opacity: 0.3 });

pushChoreographer.addAnimation(newView.start());
pushChoreographer.addAnimation(oldView.start());
```

---

## Timeline Sequencing

### Sequential Animations
```javascript
const sequencer = new Sequencer();

// Animate one after another
const anim1 = new Animation(el1).smooth(300).animate({ y: 100 }).start();
const anim2 = new Animation(el2).smooth(300).animate({ y: 100 }).start();
const anim3 = new Animation(el3).smooth(300).animate({ y: 100 }).start();

sequencer.then(anim1);  // Starts immediately
sequencer.then(anim2);  // Starts after anim1 completes
sequencer.then(anim3);  // Starts after anim2 completes

sequencer.play();
```

### Parallel Animations
```javascript
const sequencer = new Sequencer();

sequencer.then(anim1);  // Starts first
sequencer.with(anim2);  // Starts WITH anim1 (parallel)
sequencer.with(anim3);  // Also starts WITH anim1

sequencer.play(); // All three animate together
```

### Overlapping Animations
```javascript
const sequencer = new Sequencer();

sequencer.then(anim1);         // Starts at 0ms
sequencer.overlap(anim2, 0.5); // Starts when anim1 is 50% done
sequencer.overlap(anim3, 0.8); // Starts when anim2 is 80% done

sequencer.play();
```

### Scrubbing Timeline
```javascript
// Scrub to 50% through entire sequence
sequencer.seekTo(0.5);

// Get total duration
const duration = sequencer.totalDuration;

// Check current position
const fraction = sequencer.fraction;
```

---

## Advanced Patterns

### Interactive Modal with Gesture
```javascript
// Setup
const modal = new Animation(modalElement)
    .spring_smooth()
    .animate({ y: 0, opacity: 1 });

const gesture = new GestureController();
const choreographer = new Choreographer(0); // Present

const handle = modal.start();
gesture.connectAnimation(handle);
choreographer.addAnimation(handle);

// Interactive presentation
choreographer.beginInteractive();

// Touch handling
modalElement.addEventListener('touchstart', (e) => {
    gesture.onTapDown(e.touches[0].clientX, e.touches[0].clientY, performance.now());
});

modalElement.addEventListener('touchmove', (e) => {
    gesture.onTapMove(e.touches[0].clientX, e.touches[0].clientY, performance.now());
    choreographer.updateInteractive(gesture.fraction);
});

modalElement.addEventListener('touchend', () => {
    gesture.onTapUp();
    choreographer.finishInteractive(gesture.velocity);
});
```

### Staggered Card Stack
```javascript
const sequencer = new Sequencer();
const cards = document.querySelectorAll('.card');

cards.forEach((card, i) => {
    const anim = new Animation(card)
        .smooth(400)
        .animate({ y: 0, opacity: 1 })
        .start();
    
    if (i === 0) {
        sequencer.then(anim);
    } else {
        sequencer.overlap(anim, 0.7); // Start at 70% of previous
    }
});

sequencer.play();
```

### Page Transition with Multiple Elements
```javascript
const choreographer = new Choreographer(2); // Push

// Background fades
const bgAnim = new Animation(background)
    .smooth(300)
    .animate({ opacity: 0 });

// Title slides
const titleAnim = new Animation(title)
    .snappy(350)
    .animate({ x: -50, opacity: 0 });

// Content follows
const contentAnim = new Animation(content)
    .smooth(400)
    .set_delay(50)
    .animate({ y: 20, opacity: 0 });

choreographer.addAnimation(bgAnim.start());
choreographer.addAnimation(titleAnim.start());
choreographer.addAnimation(contentAnim.start());

// Make it interactive
choreographer.beginInteractive();
```

---

## Physics Tuning

### Spring Presets
```javascript
// Create spring with preset
const spring = Spring.default();  // Balanced (300 stiffness, 30 damping)
const spring = Spring.bouncy();   // Playful (250, 15)
const spring = Spring.smooth();   // Polished (400, 40)
const spring = Spring.snappy();   // Quick (500, 35)
const spring = Spring.gentle();   // Subtle (200, 25)

// Custom spring
const spring = new Spring(350, 28); // stiffness, damping
```

### Gesture with Spring Physics
```javascript
const gesture = new GestureController()
    .withSpring(Spring.bouncy());

// Or without spring (direct control)
const gesture = new GestureController()
    .withoutSpring();

// Tune sensitivity
gesture.sensitivity = 600;           // Pixels to travel for full range
gesture.completionThreshold = 0.4;   // 0.0-1.0, when to complete vs cancel
```

### Spring Properties
```javascript
const spring = new Spring(300, 30);

// Read state
spring.position;  // Current value
spring.velocity;  // Current velocity

// Tune physics
spring.stiffness = 400;  // Higher = faster/harder
spring.damping = 35;     // Higher = less oscillation

// Manual control
spring.velocity = 100;   // Inject velocity
spring.reset(0.5);       // Jump to value

// Check if settled
if (spring.isAtRest(targetValue, 0.01)) {
    console.log('Spring settled');
}
```

### Real-time Spring Animation
```javascript
const spring = Spring.smooth();
spring.reset(0); // Start at 0

function animate() {
    const position = spring.update(1.0, deltaTime); // Move toward 1.0
    element.style.opacity = position;
    
    if (!spring.isAtRest(1.0, 0.001)) {
        requestAnimationFrame(animate);
    }
}

animate();
```

### Gesture Physics Deep Dive
```javascript
const gesture = new GestureController();

// Get detailed velocity information
console.log(gesture.velocity);   // Total velocity magnitude
console.log(gesture.velocityX);  // Horizontal component
console.log(gesture.velocityY);  // Vertical component

// Displacement vectors
console.log(gesture.displacement);   // Total distance (Pythagorean)
console.log(gesture.displacementX);  // Horizontal distance
console.log(gesture.displacementY);  // Vertical distance

// Access and modify spring
const spring = gesture.getSpring();
spring.damping = 20; // Make it bouncier
gesture.setSpring(spring);
```

---

## Properties Reference

### GestureController
```javascript
gesture.fraction      // Current animation fraction (0.0-1.0)
gesture.velocity      // Current gesture velocity
gesture.isTracking    // Whether gesture is active
gesture.displacement  // Distance moved from start
```

### Choreographer
```javascript
choreographer.fraction      // Overall transition fraction
choreographer.isInteractive // Whether in interactive mode
choreographer.isCancelled   // Whether transition was cancelled
choreographer.context       // 0=Present, 1=Dismiss, 2=Push, 3=Pop
```

### Sequencer
```javascript
sequencer.fraction       // Current timeline position (0.0-1.0)
sequencer.totalDuration  // Total duration in ms
sequencer.stepCount      // Number of animation steps
sequencer.isRunning      // Whether sequence is playing
```

---

## Best Practices

### ✅ Do
- Use gestures for dismiss interactions (cards, modals, sheets)
- Choreograph related animations together (present/dismiss pairs)
- Sequence multi-step animations (onboarding, tutorials)
- Tune physics for your specific use case

### ❌ Don't
- Don't update gestures more than 60fps
- Don't choreograph unrelated animations
- Don't use long sequences for simple cases
- Don't forget to handle cancelled states

---

## Performance Tips

1. **Batch Updates**: Update multiple properties in single animation
2. **Use `continue_animate`**: Preserve transform state between animations
3. **Limit Gesture Updates**: Throttle touch events to 60fps
4. **Prefer Transforms**: Use x/y/scale over width/height
5. **Spring Over Bezier**: Springs feel better for gestures

---

## Example: Complete Modal Implementation

```javascript
class InteractiveModal {
    constructor(element) {
        this.element = element;
        this.gesture = new GestureController();
        this.choreographer = new Choreographer(0); // Present
        this.animation = null;
        this.setupAnimation();
        this.attachListeners();
    }

    setupAnimation() {
        this.animation = new Animation(this.element)
            .spring_smooth()
            .animate({ y: 0, opacity: 1 })
            .start();
        
        this.gesture.connectAnimation(this.animation);
        this.choreographer.addAnimation(this.animation);
    }

    attachListeners() {
        this.element.addEventListener('touchstart', (e) => this.onTouchStart(e));
        this.element.addEventListener('touchmove', (e) => this.onTouchMove(e));
        this.element.addEventListener('touchend', () => this.onTouchEnd());
    }

    onTouchStart(e) {
        this.gesture.onTapDown(
            e.touches[0].clientX,
            e.touches[0].clientY,
            performance.now()
        );
        this.choreographer.beginInteractive();
    }

    onTouchMove(e) {
        this.gesture.onTapMove(
            e.touches[0].clientX,
            e.touches[0].clientY,
            performance.now()
        );
        this.choreographer.updateInteractive(this.gesture.fraction);
    }

    onTouchEnd() {
        this.gesture.onTapUp();
        const completed = this.choreographer.finishInteractive(this.gesture.velocity);
        
        if (!completed) {
            this.element.dispatchEvent(new CustomEvent('dismiss'));
        }
    }
}

// Usage
const modal = new InteractiveModal(document.getElementById('modal'));
modal.element.addEventListener('dismiss', () => {
    console.log('Modal dismissed!');
});
```

---

## Example: Complete Interactive Card

```javascript
class InteractiveCard {
    constructor(element) {
        this.element = element;
        this.gesture = new GestureController().withSpring(Spring.bouncy());
        this.particles = ParticlePresets.sparkle();
        this.animation = null;
        this.setupAnimation();
        this.attachListeners();
    }

    setupAnimation() {
        this.animation = new Animation(this.element)
            .spring_smooth()
            .animate({ y: -400, opacity: 0, scale: 0.8 })
            .start();
        
        this.gesture.connectAnimation(this.animation);
        this.gesture.sensitivity = 400;
        this.gesture.completionThreshold = 0.4;
    }

    attachListeners() {
        this.element.addEventListener('touchstart', (e) => this.onTouchStart(e));
        this.element.addEventListener('touchmove', (e) => this.onTouchMove(e));
        this.element.addEventListener('touchend', () => this.onTouchEnd());
        
        // Start particle animation loop
        this.animateParticles();
    }

    onTouchStart(e) {
        this.gesture.onTapDown(
            e.touches[0].clientX,
            e.touches[0].clientY,
            performance.now()
        );
    }

    onTouchMove(e) {
        this.gesture.onTapMove(
            e.touches[0].clientX,
            e.touches[0].clientY,
            performance.now()
        );
        
        // Emit particles while dragging fast
        if (Math.abs(this.gesture.velocityY) > 0.5) {
            const particle = this.createParticle();
            this.particles.emit(
                particle,
                e.touches[0].clientX,
                e.touches[0].clientY
            );
        }
    }

    onTouchEnd() {
        this.gesture.onTapUp();
        
        // Burst effect on successful dismiss
        if (this.gesture.fraction > this.gesture.completionThreshold) {
            const rect = this.element.getBoundingClientRect();
            const particle = this.createParticle();
            this.particles.emitBurst(
                particle,
                rect.left + rect.width / 2,
                rect.top + rect.height / 2,
                15
            );
            
            this.element.dispatchEvent(new CustomEvent('dismiss'));
        }
    }

    createParticle() {
        const particle = document.createElement('div');
        particle.className = 'sparkle';
        particle.style.cssText = `
            position: fixed;
            width: 8px;
            height: 8px;
            background: radial-gradient(circle, #fff, #667eea);
            border-radius: 50%;
            pointer-events: none;
            z-index: 9999;
        `;
        document.body.appendChild(particle);
        return particle;
    }

    animateParticles() {
        let lastTime = performance.now();
        
        const animate = (time) => {
            const dt = (time - lastTime) / 1000;
            lastTime = time;
            this.particles.update(dt);
            requestAnimationFrame(animate);
        };
        
        animate(performance.now());
    }
}

// Usage
const card = new InteractiveCard(document.getElementById('card'));

card.element.addEventListener('dismiss', () => {
    console.log('Card dismissed with style!');
});
```

---

## Advanced: Multi-Stage Transition System

```javascript
class TransitionManager {
    constructor() {
        this.choreographers = new Map();
        this.sequencer = new Sequencer();
    }
    
    // Present modal with complex choreography
    presentModal(modalElement) {
        return Transaction.batchWithCompletion(
            0.6,
            () => {
                const choreographer = new Choreographer(TransitionContext.Present);
                
                // Modal slide up
                const modalAnim = new Animation(modalElement)
                    .spring_smooth()
                    .animate({ y: 0, opacity: 1, scale: 1 });
                
                // Overlay fade
                const overlay = document.getElementById('overlay');
                const overlayAnim = new Animation(overlay)
                    .smooth(600)
                    .animate({ opacity: 0.6 });
                
                // Background blur
                const background = document.getElementById('background');
                const bgAnim = new Animation(background)
                    .smooth(600)
                    .animate({ blur: 10, scale: 0.95 });
                
                choreographer.addAnimation(modalAnim.start());
                choreographer.addAnimation(overlayAnim.start());
                choreographer.addAnimation(bgAnim.start());
                
                this.choreographers.set('modal', choreographer);
            },
            () => {
                console.log('Modal presented');
            }
        );
    }
    
    // Dismiss with gesture
    dismissModalInteractive(modalElement) {
        const choreographer = this.choreographers.get('modal');
        if (!choreographer) return;
        
        const gesture = new GestureController()
            .withSpring(Spring.bouncy());
        
        // Connect gesture to choreographer
        choreographer.beginInteractive();
        
        modalElement.addEventListener('touchmove', (e) => {
            gesture.onTapMove(
                e.touches[0].clientX,
                e.touches[0].clientY,
                performance.now()
            );
            choreographer.updateInteractive(1 - gesture.fraction);
        });
        
        modalElement.addEventListener('touchend', () => {
            const completed = choreographer.finishInteractive(gesture.velocity);
            
            if (completed) {
                console.log('Modal dismissed');
                this.choreographers.delete('modal');
            }
        });
    }
    
    // Sequential onboarding flow
    startOnboarding(steps) {
        const sequencer = new Sequencer();
        
        steps.forEach((step, i) => {
            const anim = new Animation(step.element)
                .smooth(400)
                .animate(step.config);
            
            if (i === 0) {
                sequencer.then(anim.start());
            } else {
                sequencer.overlap(anim.start(), 0.7); // Stagger
            }
        });
        
        sequencer.play();
        return sequencer;
    }
}

// Usage
const manager = new TransitionManager();

// Present modal
manager.presentModal(document.getElementById('modal'));

// Setup interactive dismissal
manager.dismissModalInteractive(document.getElementById('modal'));

// Run onboarding
const onboarding = manager.startOnboarding([
    { element: el1, config: { y: 0, opacity: 1 } },
    { element: el2, config: { y: 0, opacity: 1 } },
    { element: el3, config: { y: 0, opacity: 1 } }
]);

// Scrub through onboarding
scrubber.addEventListener('input', (e) => {
    onboarding.seekTo(e.target.value / 100);
});
```

---

This is the animation engine Apple would build for the web.