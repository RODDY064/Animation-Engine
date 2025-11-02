#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};

wasm_bindgen_test_configure!(run_in_browser);
use anim::Animation;

// ============================================================================
// CUBIC BEZIER TESTS
// ============================================================================

#[wasm_bindgen_test]
fn test_cubic_bezier_endpoints() {
    // Test that bezier starts at 0 and ends at 1
    let bezier = anim::CubicBezier::ease_out();
    
    assert_eq!(bezier.solve(0.0), 0.0, "Bezier should start at 0");
    assert_eq!(bezier.solve(1.0), 1.0, "Bezier should end at 1");
}

#[wasm_bindgen_test]
fn test_cubic_bezier_monotonic() {
    // Test that ease-out is monotonically increasing
    let bezier = anim::CubicBezier::ease_out();
    
    let mut last = 0.0;
    for i in 0..100 {
        let t = i as f64 / 100.0;
        let value = bezier.solve(t);
        
        assert!(
            value >= last,
            "Bezier should be monotonically increasing: {} < {} at t={}",
            value, last, t
        );
        last = value;
    }
}

#[wasm_bindgen_test]
fn test_ease_out_profile() {
    // Ease-out should move faster at the start than at the end
    let bezier = anim::CubicBezier::ease_out();
    
    let start_delta = bezier.solve(0.1) - bezier.solve(0.0);
    let end_delta = bezier.solve(1.0) - bezier.solve(0.9);
    
    assert!(
        start_delta > end_delta,
        "Ease-out should decelerate: start_delta={}, end_delta={}",
        start_delta, end_delta
    );
}

#[wasm_bindgen_test]
fn test_cubic_bezier_presets() {
    // Test that iOS preset curves are valid
    let ease_in = anim::CubicBezier::ease_in();
    let ease_out = anim::CubicBezier::ease_out();
    let ease_in_out = anim::CubicBezier::ease_in_out();
    let fluid = anim::CubicBezier::fluid_ease_out();
    
    // All should produce valid values
    assert!(ease_in.solve(0.5) > 0.0 && ease_in.solve(0.5) < 1.0);
    assert!(ease_out.solve(0.5) > 0.0 && ease_out.solve(0.5) < 1.0);
    assert!(ease_in_out.solve(0.5) > 0.0 && ease_in_out.solve(0.5) < 1.0);
    assert!(fluid.solve(0.5) > 0.0 && fluid.solve(0.5) < 1.0);
}

// ============================================================================
// SPRING PHYSICS TESTS
// ============================================================================

#[wasm_bindgen_test]
fn test_spring_converges_to_target() {
    let mut spring = anim::Spring::new(300.0, 30.0);
    spring.reset(0.0);
    
    let target = 100.0;
    let delta_time = 1.0 / 60.0; // 60fps
    
    // Simulate 3 seconds (should be enough to settle)
    for _ in 0..180 {
        spring.update(target, delta_time);
    }
    
    // Should be very close to target
    assert!(
        (spring.current - target).abs() < 0.1,
        "Spring should converge to target: current={}, target={}",
        spring.current, target
    );
}

#[wasm_bindgen_test]
fn test_spring_damping() {
    // Higher damping should reduce oscillation
    let mut bouncy = anim::Spring::new(300.0, 10.0); // Low damping
    let mut smooth = anim::Spring::new(300.0, 50.0); // High damping
    
    bouncy.reset(0.0);
    smooth.reset(0.0);
    
    let target = 100.0;
    let delta_time = 1.0 / 60.0;
    
    let mut bouncy_overshoots = 0;
    let mut smooth_overshoots = 0;
    
    for _ in 0..120 {
        let bouncy_val = bouncy.update(target, delta_time);
        let smooth_val = smooth.update(target, delta_time);
        
        if bouncy_val > target {
            bouncy_overshoots += 1;
        }
        if smooth_val > target {
            smooth_overshoots += 1;
        }
    }
    
    assert!(
        bouncy_overshoots > smooth_overshoots,
        "Lower damping should cause more overshoot: bouncy={}, smooth={}",
        bouncy_overshoots, smooth_overshoots
    );
}

#[wasm_bindgen_test]
fn test_spring_presets() {
    // Test that preset springs produce valid behavior
    let default = anim::Spring::default();
    let bouncy = anim::Spring::bouncy();
    let smooth = anim::Spring::smooth();
    
    assert!(default.stiffness > 0.0);
    assert!(bouncy.stiffness > 0.0);
    assert!(smooth.stiffness > 0.0);
}

// ============================================================================
// ANIMATION INTEGRATION TESTS
// ============================================================================

#[wasm_bindgen_test]
fn test_animation_creation() {
    let window = window().expect("No window");
    let document = window.document().expect("No document");
    
    // Create test element
    let element = document
        .create_element("div")
        .expect("Failed to create element")
        .dyn_into::<HtmlElement>()
        .expect("Failed to cast to HtmlElement");
    
    // Should create animation without error
    let animation = Animation::new(element);
    assert!(animation.is_ok(), "Animation creation should succeed");
}

#[wasm_bindgen_test]
fn test_animation_cubic_setup() {
    let window = window().expect("No window");
    let document = window.document().expect("No document");
    
    let element = document
        .create_element("div")
        .expect("Failed to create element")
        .dyn_into::<HtmlElement>()
        .expect("Failed to cast to HtmlElement");
    
    let animation = Animation::new(element)
        .expect("Animation creation failed")
        .cubic(0.0, 0.0, 0.58, 1.0, 300.0)
        .to(Some(1.0), Some(200.0), None, None, None, None, None, None);
    
    // If we get here without panic, setup worked
    assert!(true);
}

#[wasm_bindgen_test]
fn test_animation_spring_setup() {
    let window = window().expect("No window");
    let document = window.document().expect("No document");
    
    let element = document
        .create_element("div")
        .expect("Failed to create element")
        .dyn_into::<HtmlElement>()
        .expect("Failed to cast to HtmlElement");
    
    let animation = Animation::new(element)
        .expect("Animation creation failed")
        .spring(300.0, 30.0)
        .to(None, Some(100.0), None, None, None, None, None, None);
    
    // If we get here without panic, setup worked
    assert!(true);
}

#[wasm_bindgen_test]
async fn test_animation_applies_transform() {
    let window = window().expect("No window");
    let document = window.document().expect("No document");
    
    let element = document
        .create_element("div")
        .expect("Failed to create element")
        .dyn_into::<HtmlElement>()
        .expect("Failed to cast to HtmlElement");
    
    // Add to document so styles work
    document.body()
        .expect("No body")
        .append_child(&element)
        .expect("Failed to append");
    
    let mut animation = Animation::new(element.clone())
        .expect("Animation creation failed")
        .cubic(0.0, 0.0, 1.0, 1.0, 100.0) // Linear for predictability
        .to(None, Some(200.0), None, None, None, None, None, None);
    
    animation.start().expect("Animation start failed");
    
    // Wait a frame for initial application
    wasm_bindgen_futures::JsFuture::from(
        js_sys::Promise::new(&mut |resolve, _| {
            window.request_animation_frame(&resolve).unwrap();
        })
    ).await.unwrap();
    
    // Check that transform was applied
    let transform = element.style().get_property_value("transform")
        .expect("Failed to get transform");
    
    assert!(
        !transform.is_empty(),
        "Transform should be applied to element"
    );
}

// ============================================================================
// PERFORMANCE TESTS
// ============================================================================

#[wasm_bindgen_test]
fn test_bezier_performance() {
    // Test that bezier calculation is fast enough
    let bezier = anim::CubicBezier::ease_out();
    
    let window = window().expect("No window");
    let performance = window.performance().expect("No performance");
    
    let start = performance.now();
    
    // Calculate 10,000 bezier values
    for i in 0..10_000 {
        let t = (i as f64) / 10_000.0;
        bezier.solve(t);
    }
    
    let elapsed = performance.now() - start;
    
    // Should complete in under 100ms (very generous)
    assert!(
        elapsed < 100.0,
        "Bezier calculations too slow: {}ms for 10k iterations",
        elapsed
    );
}

#[wasm_bindgen_test]
fn test_spring_performance() {
    let mut spring = anim::Spring::new(300.0, 30.0);
    spring.reset(0.0);
    
    let window = window().expect("No window");
    let performance = window.performance().expect("No performance");
    
    let start = performance.now();
    
    // Simulate 10 seconds at 60fps
    let delta_time = 1.0 / 60.0;
    for _ in 0..600 {
        spring.update(100.0, delta_time);
    }
    
    let elapsed = performance.now() - start;
    
    // Should complete in under 50ms
    assert!(
        elapsed < 50.0,
        "Spring physics too slow: {}ms for 600 iterations",
        elapsed
    );
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[wasm_bindgen_test]
fn test_bezier_handles_zero_time() {
    let bezier = anim::CubicBezier::ease_out();
    let result = bezier.solve(0.0);
    assert_eq!(result, 0.0, "Bezier at t=0 should return 0");
}

#[wasm_bindgen_test]
fn test_bezier_handles_one_time() {
    let bezier = anim::CubicBezier::ease_out();
    let result = bezier.solve(1.0);
    assert_eq!(result, 1.0, "Bezier at t=1 should return 1");
}

#[wasm_bindgen_test]
fn test_bezier_handles_negative_time() {
    let bezier = anim::CubicBezier::ease_out();
    let result = bezier.solve(-0.5);
    assert_eq!(result, 0.0, "Bezier at negative t should clamp to 0");
}

#[wasm_bindgen_test]
fn test_bezier_handles_overtime() {
    let bezier = anim::CubicBezier::ease_out();
    let result = bezier.solve(1.5);
    assert_eq!(result, 1.0, "Bezier at t>1 should clamp to 1");
}

#[wasm_bindgen_test]
fn test_spring_zero_velocity() {
    let mut spring = anim::Spring::new(300.0, 30.0);
    spring.reset(50.0);
    
    // Update toward same position
    let result = spring.update(50.0, 1.0 / 60.0);
    
    assert!(
        (result - 50.0).abs() < 0.01,
        "Spring with no force should stay at rest"
    );
}