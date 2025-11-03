/// Shape Morphing - SVG path interpolation
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PathMorph {
    start_path: String,
    end_path: String,
    progress: f64,
    start_commands: Vec<PathCommand>,
    end_commands: Vec<PathCommand>,
}

#[wasm_bindgen]
impl PathMorph {
    #[wasm_bindgen(constructor)]
    pub fn new(start_path: String, end_path: String) -> Result<PathMorph, JsValue> {
        if start_path.is_empty() || end_path.is_empty() {
            return Err(JsValue::from_str("Paths cannot be empty"));
        }

        let start_commands = Self::parse_path(&start_path)?;
        let end_commands = Self::parse_path(&end_path)?;

        if start_commands.len() != end_commands.len() {
            return Err(JsValue::from_str(
                "Paths must have same number of commands. Use PathMorph.normalize() to fix."
            ));
        }

        Ok(PathMorph {
            start_path,
            end_path,
            progress: 0.0,
            start_commands,
            end_commands,
        })
    }

    /// Update morph progress and return interpolated path
    #[wasm_bindgen(js_name = updateProgress)]
    pub fn update_progress(&mut self, progress: f64) -> String {
        self.progress = progress.clamp(0.0, 1.0);
        self.interpolate()
    }

    /// Get current progress
    #[wasm_bindgen(getter)]
    pub fn progress(&self) -> f64 {
        self.progress
    }

    /// Set progress directly
    #[wasm_bindgen(setter)]
    pub fn set_progress(&mut self, value: f64) {
        self.progress = value.clamp(0.0, 1.0);
    }

    /// Get interpolated path at current progress
    #[wasm_bindgen(js_name = getPath)]
    pub fn get_path(&self) -> String {
        self.interpolate()
    }

    /// Get path at specific progress without updating state
    #[wasm_bindgen(js_name = getPathAt)]
    pub fn get_path_at(&self, progress: f64) -> String {
        self.interpolate_at(progress.clamp(0.0, 1.0))
    }

    // ========================================================================
    // INTERNAL INTERPOLATION
    // ========================================================================

    fn interpolate(&self) -> String {
        self.interpolate_at(self.progress)
    }

    fn interpolate_at(&self, t: f64) -> String {
        let mut result = String::with_capacity(self.start_path.len());

        for (start_cmd, end_cmd) in self.start_commands.iter().zip(self.end_commands.iter()) {
            match (start_cmd, end_cmd) {
                (PathCommand::Move(sx, sy), PathCommand::Move(ex, ey)) => {
                    result.push_str(&format!("M{} {} ", lerp(*sx, *ex, t), lerp(*sy, *ey, t)));
                }
                (PathCommand::Line(sx, sy), PathCommand::Line(ex, ey)) => {
                    result.push_str(&format!("L{} {} ", lerp(*sx, *ex, t), lerp(*sy, *ey, t)));
                }
                (
                    PathCommand::Cubic(sx1, sy1, sx2, sy2, sx, sy),
                    PathCommand::Cubic(ex1, ey1, ex2, ey2, ex, ey),
                ) => {
                    result.push_str(&format!(
                        "C{} {} {} {} {} {} ",
                        lerp(*sx1, *ex1, t),
                        lerp(*sy1, *ey1, t),
                        lerp(*sx2, *ex2, t),
                        lerp(*sy2, *ey2, t),
                        lerp(*sx, *ex, t),
                        lerp(*sy, *ey, t)
                    ));
                }
                (PathCommand::Quad(sx1, sy1, sx, sy), PathCommand::Quad(ex1, ey1, ex, ey)) => {
                    result.push_str(&format!(
                        "Q{} {} {} {} ",
                        lerp(*sx1, *ex1, t),
                        lerp(*sy1, *ey1, t),
                        lerp(*sx, *ex, t),
                        lerp(*sy, *ey, t)
                    ));
                }
                (PathCommand::Close, PathCommand::Close) => {
                    result.push('Z');
                }
                _ => {} // Mismatched commands (shouldn't happen after validation)
            }
        }

        result.trim().to_string()
    }

    // ========================================================================
    // PATH PARSING
    // ========================================================================

    fn parse_path(path: &str) -> Result<Vec<PathCommand>, JsValue> {
        let mut commands = Vec::new();
        let mut chars = path.trim().chars().peekable();

        while let Some(&ch) = chars.peek() {
            match ch {
                'M' | 'm' => {
                    chars.next();
                    if let Some(nums) = Self::collect_numbers(&mut chars, 2) {
                        commands.push(PathCommand::Move(nums[0], nums[1]));
                    }
                }
                'L' | 'l' => {
                    chars.next();
                    if let Some(nums) = Self::collect_numbers(&mut chars, 2) {
                        commands.push(PathCommand::Line(nums[0], nums[1]));
                    }
                }
                'C' | 'c' => {
                    chars.next();
                    if let Some(nums) = Self::collect_numbers(&mut chars, 6) {
                        commands.push(PathCommand::Cubic(
                            nums[0], nums[1], nums[2], nums[3], nums[4], nums[5],
                        ));
                    }
                }
                'Q' | 'q' => {
                    chars.next();
                    if let Some(nums) = Self::collect_numbers(&mut chars, 4) {
                        commands.push(PathCommand::Quad(nums[0], nums[1], nums[2], nums[3]));
                    }
                }
                'Z' | 'z' => {
                    chars.next();
                    commands.push(PathCommand::Close);
                }
                _ => {
                    chars.next();
                }
            }
        }

        Ok(commands)
    }

    fn collect_numbers(
        chars: &mut std::iter::Peekable<std::str::Chars>,
        count: usize,
    ) -> Option<Vec<f64>> {
        let mut numbers = Vec::with_capacity(count);
        let mut current = String::new();

        while numbers.len() < count {
            match chars.peek() {
                Some(&ch) if ch.is_numeric() || ch == '.' || ch == '-' => {
                    current.push(ch);
                    chars.next();
                }
                Some(&ch) if ch == ',' || ch.is_whitespace() => {
                    if !current.is_empty() {
                        if let Ok(num) = current.parse::<f64>() {
                            numbers.push(num);
                        }
                        current.clear();
                    }
                    chars.next();
                }
                Some(&ch) if ch.is_alphabetic() => break,
                None => {
                    if !current.is_empty() {
                        if let Ok(num) = current.parse::<f64>() {
                            numbers.push(num);
                        }
                    }
                    break;
                }
                _ => {
                    chars.next();
                }
            }
        }

        if !current.is_empty() {
            if let Ok(num) = current.parse::<f64>() {
                numbers.push(num);
            }
        }

        if numbers.len() == count {
            Some(numbers)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
enum PathCommand {
    Move(f64, f64),
    Line(f64, f64),
    Cubic(f64, f64, f64, f64, f64, f64),
    Quad(f64, f64, f64, f64),
    Close,
}

#[inline]
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}
