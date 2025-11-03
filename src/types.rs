use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PropertyType {
    X,
    Y,
    Z,
    Scale,
    Rotate,
    Opacity,
    Width,
    Height,
    BackgroundColor,
    StrokeDashOffset,
    StrokeWidth,
}

impl PropertyType {
    pub fn as_str(&self) -> &str {
        match self {
            PropertyType::X => "x",
            PropertyType::Y => "y",
            PropertyType::Z => "z",
            PropertyType::Scale => "scale",
            PropertyType::Rotate => "rotate",
            PropertyType::Opacity => "opacity",
            PropertyType::Width => "width",
            PropertyType::Height => "height",
            PropertyType::BackgroundColor => "background-color",
            PropertyType::StrokeDashOffset => "stroke-dashoffset",
            PropertyType::StrokeWidth => "stroke-width",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "x" => Some(PropertyType::X),
            "y" => Some(PropertyType::Y),
            "z" => Some(PropertyType::Z),
            "scale" => Some(PropertyType::Scale),
            "rotate" => Some(PropertyType::Rotate),
            "opacity" => Some(PropertyType::Opacity),
            "width" => Some(PropertyType::Width),
            "height" => Some(PropertyType::Height),
            "background-color" => Some(PropertyType::BackgroundColor),
            "stroke-dashoffset" => Some(PropertyType::StrokeDashOffset),
            "stroke-width" => Some(PropertyType::StrokeWidth),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum AnimatableValue {
    Number(f64),
    Color(f64, f64, f64, f64),
    Length(f64, LengthUnit),
}

#[derive(Clone, Debug)]
pub enum LengthUnit {
    Px,
    Percent,
}

#[derive(Clone, Debug)]
pub struct AnimationProperty {
    pub property_type: PropertyType,
    pub start: AnimatableValue,
    pub end: AnimatableValue,
    pub current: AnimatableValue,
}

#[derive(Clone)]
pub struct Keyframe {
    pub time: f64,
    pub properties: Vec<(PropertyType, AnimatableValue)>,
}

#[derive(Deserialize)]
pub struct KeyframeConfig {
    pub time: f64,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub scale: Option<f64>,
    pub opacity: Option<f64>,
    pub rotate: Option<f64>,
}

#[derive(Deserialize)]
pub struct AnimateConfig {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
    pub scale: Option<f64>,
    pub rotate: Option<f64>,
    pub opacity: Option<f64>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub background_color: Option<String>,
    pub stroke_dashoffset: Option<f64>,
    pub stroke_width: Option<f64>,
}

pub fn interpolate_value(start: &AnimatableValue, end: &AnimatableValue, t: f64) -> AnimatableValue {
    match (start, end) {
        (AnimatableValue::Number(s), AnimatableValue::Number(e)) => {
            AnimatableValue::Number(s + (e - s) * t)
        }
        (AnimatableValue::Color(r1, g1, b1, a1), AnimatableValue::Color(r2, g2, b2, a2)) => {
            AnimatableValue::Color(
                r1 + (r2 - r1) * t,
                g1 + (g2 - g1) * t,
                b1 + (b2 - b1) * t,
                a1 + (a2 - a1) * t,
            )
        }
        (AnimatableValue::Length(v1, u), AnimatableValue::Length(v2, _)) => {
            AnimatableValue::Length(v1 + (v2 - v1) * t, u.clone())
        }
        _ => start.clone(),
    }
}

pub fn extract_number(value: &AnimatableValue) -> f64 {
    match value {
        AnimatableValue::Number(n) => *n,
        AnimatableValue::Length(n, _) => *n,
        _ => 0.0,
    }
}

pub fn create_value_with_number(template: &AnimatableValue, num: f64) -> AnimatableValue {
    match template {
        AnimatableValue::Number(_) => AnimatableValue::Number(num),
        AnimatableValue::Length(_, u) => AnimatableValue::Length(num, u.clone()),
        AnimatableValue::Color(_, g, b, a) => AnimatableValue::Color(num, *g, *b, *a),
    }
}

pub fn format_value(value: &AnimatableValue) -> String {
    match value {
        AnimatableValue::Number(n) => n.to_string(),
        AnimatableValue::Length(n, u) => format!("{}{}", n, format_unit(u)),
        AnimatableValue::Color(r, g, b, a) => {
            format!("rgba({}, {}, {}, {})", r.round(), g.round(), b.round(), a)
        }
    }
}

fn format_unit(unit: &LengthUnit) -> &str {
    match unit {
        LengthUnit::Px => "px",
        LengthUnit::Percent => "%",
    }
}

pub fn parse_css_length(value: &str) -> Result<(f64, LengthUnit), String> {
    let value = value.trim();

    if value.ends_with("px") {
        let num = value[..value.len() - 2]
            .parse::<f64>()
            .map_err(|_| "Invalid px value".to_string())?;
        Ok((num, LengthUnit::Px))
    } else if value.ends_with('%') {
        let num = value[..value.len() - 1]
            .parse::<f64>()
            .map_err(|_| "Invalid % value".to_string())?;
        Ok((num, LengthUnit::Percent))
    } else {
        let num = value
            .parse::<f64>()
            .map_err(|_| "Invalid number".to_string())?;
        Ok((num, LengthUnit::Px))
    }
}

pub fn parse_css_color(value: &str) -> Result<(f64, f64, f64, f64), String> {
    if value.starts_with('#') {
        let hex = &value[1..];
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f64;
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f64;
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f64;
            return Ok((r, g, b, 1.0));
        }
    }
    Ok((0.0, 0.0, 0.0, 1.0))
}