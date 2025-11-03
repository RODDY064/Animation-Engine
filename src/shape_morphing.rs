use wasm_bindgen::prelude::*;

/// Shape Morphing - SVG path interpolation for morphing animations
#[wasm_bindgen]
pub struct PathMorph {
    start_path: String,
    end_path: String,
    current_path: String,
    progress: f64,
}

#[wasm_bindgen]
impl PathMorph {
    #[wasm_bindgen(constructor)]
    pub fn new(start_path: String, end_path: String) -> Result<PathMorph, JsValue> {
        if start_path.is_empty() || end_path.is_empty() {
            return Err(JsValue::from_str("Paths cannot be empty"));
        }

        Ok(PathMorph {
            start_path,
            end_path,
            current_path: String::new(),
            progress: 0.0,
        })
    }

    /// Update the morph progress (0.0 to 1.0)
    #[wasm_bindgen]
    pub fn update_progress(&mut self, progress: f64) -> Result<String, JsValue> {
        self.progress = progress.clamp(0.0, 1.0);
        self.current_path = self.interpolate_path(self.progress)?;
        Ok(self.current_path.clone())
    }

    /// Get the current morphed path
    #[wasm_bindgen]
    pub fn get_current_path(&self) -> String {
        self.current_path.clone()
    }

    /// Get current progress
    #[wasm_bindgen]
    pub fn get_progress(&self) -> f64 {
        self.progress
    }

    /// Parse and interpolate SVG paths
    fn interpolate_path(&self, t: f64) -> Result<String, JsValue> {
        let start_commands = self.parse_path(&self.start_path)?;
        let end_commands = self.parse_path(&self.end_path)?;

        if start_commands.len() != end_commands.len() {
            return Err(JsValue::from_str(
                "Paths must have the same number of commands",
            ));
        }

        let mut result = String::new();

        for (start_cmd, end_cmd) in start_commands.iter().zip(end_commands.iter()) {
            match (start_cmd, end_cmd) {
                (PathCommand::Move(sx, sy), PathCommand::Move(ex, ey)) => {
                    let x = sx + (ex - sx) * t;
                    let y = sy + (ey - sy) * t;
                    result.push_str(&format!("M {} {} ", x, y));
                }
                (PathCommand::Line(sx, sy), PathCommand::Line(ex, ey)) => {
                    let x = sx + (ex - sx) * t;
                    let y = sy + (ey - sy) * t;
                    result.push_str(&format!("L {} {} ", x, y));
                }
                (
                    PathCommand::CubicBezier(sx1, sy1, sx2, sy2, sx, sy),
                    PathCommand::CubicBezier(ex1, ey1, ex2, ey2, ex, ey),
                ) => {
                    let x1 = sx1 + (ex1 - sx1) * t;
                    let y1 = sy1 + (ey1 - sy1) * t;
                    let x2 = sx2 + (ex2 - sx2) * t;
                    let y2 = sy2 + (ey2 - sy2) * t;
                    let x = sx + (ex - sx) * t;
                    let y = sy + (ey - sy) * t;
                    result.push_str(&format!("C {} {} {} {} {} {} ", x1, y1, x2, y2, x, y));
                }
                (PathCommand::Close, PathCommand::Close) => {
                    result.push_str("Z ");
                }
                _ => {}
            }
        }

        Ok(result.trim().to_string())
    }

    /// Parse SVG path string into commands
    fn parse_path(&self, path: &str) -> Result<Vec<PathCommand>, JsValue> {
        let mut commands = Vec::new();
        let path = path.trim();
        let mut chars = path.chars().peekable();
        let mut current_number = String::new();
        let mut numbers = Vec::new();

        while let Some(&ch) = chars.peek() {
            match ch {
                'M' | 'm' => {
                    chars.next();
                    numbers.clear();
                    self.collect_numbers(&mut chars, &mut current_number, &mut numbers);
                    if numbers.len() >= 2 {
                        commands.push(PathCommand::Move(numbers[0], numbers[1]));
                    }
                }
                'L' | 'l' => {
                    chars.next();
                    numbers.clear();
                    self.collect_numbers(&mut chars, &mut current_number, &mut numbers);
                    if numbers.len() >= 2 {
                        commands.push(PathCommand::Line(numbers[0], numbers[1]));
                    }
                }
                'C' | 'c' => {
                    chars.next();
                    numbers.clear();
                    self.collect_numbers(&mut chars, &mut current_number, &mut numbers);
                    if numbers.len() >= 6 {
                        commands.push(PathCommand::CubicBezier(
                            numbers[0], numbers[1], numbers[2], numbers[3], numbers[4], numbers[5],
                        ));
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
        &self,
        chars: &mut std::iter::Peekable<std::str::Chars>,
        current_number: &mut String,
        numbers: &mut Vec<f64>,
    ) {
        while let Some(&ch) = chars.peek() {
            match ch {
                '0'..='9' | '.' | '-' => {
                    current_number.push(ch);
                    chars.next();
                }
                ',' | ' ' => {
                    if !current_number.is_empty() {
                        if let Ok(num) = current_number.parse::<f64>() {
                            numbers.push(num);
                        }
                        current_number.clear();
                    }
                    chars.next();
                }
                'M' | 'L' | 'C' | 'Z' | 'm' | 'l' | 'c' | 'z' => break,
                _ => {
                    chars.next();
                }
            }
        }
        if !current_number.is_empty() {
            if let Ok(num) = current_number.parse::<f64>() {
                numbers.push(num);
            }
            current_number.clear();
        }
    }
}

#[derive(Debug, Clone)]
enum PathCommand {
    Move(f64, f64),
    Line(f64, f64),
    CubicBezier(f64, f64, f64, f64, f64, f64),
    Close,
}