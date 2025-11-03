use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum PropertyType {
    // Transform
    X,
    Y,
    Z,
    Scale,
    ScaleX,
    ScaleY,
    Rotate,
    RotateX,
    RotateY,
    RotateZ,
    SkewX,
    SkewY,

    // Layout (Size)
    Width,
    Height,
    MinWidth,
    MinHeight,
    MaxWidth,
    MaxHeight,

    // Visual
    Opacity,
    BackgroundColor,
    Color,
    BorderColor,
    BorderRadius,
    BorderWidth,
    Visibility,  // ✨ NEW

    // Shadows & Effects
    ShadowOffsetX,
    ShadowOffsetY,
    ShadowBlur,
    ShadowSpread,
    ShadowColor,

    // Filters
    Blur,
    Brightness,
    Contrast,
    Saturate,
    Hue,
    Grayscale,
    Invert,
    Sepia,
    #[allow(dead_code)]
    Dropoff,

    // SVG
    StrokeDashOffset,
    #[allow(dead_code)]
    StrokeDashArray,
    StrokeWidth,
    FillOpacity,
    StrokeOpacity,

    // Advanced
    TransformOriginX,
    TransformOriginY,
    TransformOriginZ,
    Perspective,
    PerspectiveOriginX,
    PerspectiveOriginY,
    #[allow(dead_code)]
    BackfaceVisibility,
    #[allow(dead_code)]
    BackgroundBlur,
    #[allow(dead_code)]
    Inset,
}

impl PropertyType {
    pub fn as_str(&self) -> &str {
        match self {
            PropertyType::X => "x",
            PropertyType::Y => "y",
            PropertyType::Z => "z",
            PropertyType::Scale => "scale",
            PropertyType::ScaleX => "scaleX",
            PropertyType::ScaleY => "scaleY",
            PropertyType::Rotate => "rotate",
            PropertyType::RotateX => "rotateX",
            PropertyType::RotateY => "rotateY",
            PropertyType::RotateZ => "rotateZ",
            PropertyType::SkewX => "skewX",
            PropertyType::SkewY => "skewY",
            PropertyType::Width => "width",
            PropertyType::Height => "height",
            PropertyType::MinWidth => "minWidth",
            PropertyType::MinHeight => "minHeight",
            PropertyType::MaxWidth => "maxWidth",
            PropertyType::MaxHeight => "maxHeight",
            PropertyType::Opacity => "opacity",
            PropertyType::BackgroundColor => "backgroundColor",
            PropertyType::Color => "color",
            PropertyType::BorderColor => "borderColor",
            PropertyType::BorderRadius => "borderRadius",
            PropertyType::BorderWidth => "borderWidth",
            PropertyType::Visibility => "visibility",
            PropertyType::ShadowOffsetX => "shadowOffsetX",
            PropertyType::ShadowOffsetY => "shadowOffsetY",
            PropertyType::ShadowBlur => "shadowBlur",
            PropertyType::ShadowSpread => "shadowSpread",
            PropertyType::ShadowColor => "shadowColor",
            PropertyType::Blur => "blur",
            PropertyType::Brightness => "brightness",
            PropertyType::Contrast => "contrast",
            PropertyType::Saturate => "saturate",
            PropertyType::Hue => "hue",
            PropertyType::Grayscale => "grayscale",
            PropertyType::Invert => "invert",
            PropertyType::Sepia => "sepia",
            PropertyType::Dropoff => "dropoff",
            PropertyType::StrokeDashOffset => "strokeDashOffset",
            PropertyType::StrokeDashArray => "strokeDashArray",
            PropertyType::StrokeWidth => "strokeWidth",
            PropertyType::FillOpacity => "fillOpacity",
            PropertyType::StrokeOpacity => "strokeOpacity",
            PropertyType::TransformOriginX => "transformOriginX",
            PropertyType::TransformOriginY => "transformOriginY",
            PropertyType::TransformOriginZ => "transformOriginZ",
            PropertyType::Perspective => "perspective",
            PropertyType::PerspectiveOriginX => "perspectiveOriginX",
            PropertyType::PerspectiveOriginY => "perspectiveOriginY",
            PropertyType::BackfaceVisibility => "backfaceVisibility",
            PropertyType::BackgroundBlur => "backgroundBlur",
            PropertyType::Inset => "inset",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "x" => Some(PropertyType::X),
            "y" => Some(PropertyType::Y),
            "z" => Some(PropertyType::Z),
            "scale" => Some(PropertyType::Scale),
            "scaleX" | "scale_x" => Some(PropertyType::ScaleX),
            "scaleY" | "scale_y" => Some(PropertyType::ScaleY),
            "rotate" => Some(PropertyType::Rotate),
            "rotateX" | "rotate_x" => Some(PropertyType::RotateX),
            "rotateY" | "rotate_y" => Some(PropertyType::RotateY),
            "rotateZ" | "rotate_z" => Some(PropertyType::RotateZ),
            "skewX" | "skew_x" => Some(PropertyType::SkewX),
            "skewY" | "skew_y" => Some(PropertyType::SkewY),
            "width" => Some(PropertyType::Width),
            "height" => Some(PropertyType::Height),
            "minWidth" | "min_width" => Some(PropertyType::MinWidth),
            "minHeight" | "min_height" => Some(PropertyType::MinHeight),
            "maxWidth" | "max_width" => Some(PropertyType::MaxWidth),
            "maxHeight" | "max_height" => Some(PropertyType::MaxHeight),
            "opacity" => Some(PropertyType::Opacity),
            "backgroundColor" | "background_color" => Some(PropertyType::BackgroundColor),
            "color" => Some(PropertyType::Color),
            "borderColor" | "border_color" => Some(PropertyType::BorderColor),
            "borderRadius" | "border_radius" => Some(PropertyType::BorderRadius),
            "borderWidth" | "border_width" => Some(PropertyType::BorderWidth),
            "visibility" => Some(PropertyType::Visibility),
            "blur" => Some(PropertyType::Blur),
            "brightness" => Some(PropertyType::Brightness),
            "contrast" => Some(PropertyType::Contrast),
            "saturate" => Some(PropertyType::Saturate),
            "hue" => Some(PropertyType::Hue),
            "grayscale" => Some(PropertyType::Grayscale),
            "invert" => Some(PropertyType::Invert),
            "sepia" => Some(PropertyType::Sepia),
            "transformOriginX" | "transform_origin_x" => Some(PropertyType::TransformOriginX),
            "transformOriginY" | "transform_origin_y" => Some(PropertyType::TransformOriginY),
            "transformOriginZ" | "transform_origin_z" => Some(PropertyType::TransformOriginZ),
            "perspective" => Some(PropertyType::Perspective),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum AnimatableValue {
    Number(f64),
    Color(f64, f64, f64, f64),
    Length(f64, LengthUnit),
    Shadow(ShadowValue),
    Visibility(VisibilityValue),  // ✨ NEW
}

#[derive(Clone, Debug, PartialEq)]
pub enum VisibilityValue {
    Visible,
    Hidden,
    Collapse,
}

impl VisibilityValue {
    pub fn as_str(&self) -> &str {
        match self {
            VisibilityValue::Visible => "visible",
            VisibilityValue::Hidden => "hidden",
            VisibilityValue::Collapse => "collapse",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "hidden" => VisibilityValue::Hidden,
            "collapse" => VisibilityValue::Collapse,
            _ => VisibilityValue::Visible,
        }
    }

    /// Convert to numeric representation for interpolation
    pub fn to_number(&self) -> f64 {
        match self {
            VisibilityValue::Visible => 1.0,
            VisibilityValue::Hidden => 0.0,
            VisibilityValue::Collapse => -1.0,
        }
    }

    /// Convert from numeric representation
    pub fn from_number(n: f64) -> Self {
        if n > 0.5 {
            VisibilityValue::Visible
        } else if n < -0.5 {
            VisibilityValue::Collapse
        } else {
            VisibilityValue::Hidden
        }
    }
}

#[derive(Clone, Debug)]
pub struct ShadowValue {
    pub offset_x: f64,
    pub offset_y: f64,
    pub blur: f64,
    pub spread: f64,
    pub color: (f64, f64, f64, f64),
    pub inset: bool,
}

impl ShadowValue {
    pub fn new(
        offset_x: f64,
        offset_y: f64,
        blur: f64,
        spread: f64,
        color: (f64, f64, f64, f64),
    ) -> Self {
        Self {
            offset_x,
            offset_y,
            blur,
            spread,
            color,
            inset: false,
        }
    }

    pub fn to_css_string(&self) -> String {
        let inset_str = if self.inset { "inset " } else { "" };
        format!(
            "{}{}px {}px {}px {}px rgba({}, {}, {}, {})",
            inset_str,
            self.offset_x.round() as i32,
            self.offset_y.round() as i32,
            self.blur.round() as i32,
            self.spread.round() as i32,
            self.color.0.round() as u8,
            self.color.1.round() as u8,
            self.color.2.round() as u8,
            self.color.3,
        )
    }
}

#[derive(Clone, Debug)]
pub enum LengthUnit {
    Px,
    Percent,
    Vw,
    Vh,
    Em,
    Rem,
}

impl LengthUnit {
    pub fn as_str(&self) -> &str {
        match self {
            LengthUnit::Px => "px",
            LengthUnit::Percent => "%",
            LengthUnit::Vw => "vw",
            LengthUnit::Vh => "vh",
            LengthUnit::Em => "em",
            LengthUnit::Rem => "rem",
        }
    }
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
    pub z: Option<f64>,
    pub scale: Option<f64>,
    pub scale_x: Option<f64>,
    pub scale_y: Option<f64>,
    pub opacity: Option<f64>,
    pub rotate: Option<f64>,
    pub rotate_x: Option<f64>,
    pub rotate_y: Option<f64>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub blur: Option<f64>,
    pub brightness: Option<f64>,
    pub contrast: Option<f64>,
    pub shadow_blur: Option<f64>,
    pub shadow_offset_x: Option<f64>,
    pub shadow_offset_y: Option<f64>,
    pub visibility: Option<String>,  // ✨ NEW
}

#[derive(Deserialize)]
pub struct AnimateConfig {
    // Transform
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
    pub scale: Option<f64>,
    pub scale_x: Option<f64>,
    pub scale_y: Option<f64>,
    pub rotate: Option<f64>,
    pub rotate_x: Option<f64>,
    pub rotate_y: Option<f64>,
    pub rotate_z: Option<f64>,
    pub skew_x: Option<f64>,
    pub skew_y: Option<f64>,

    // Size
    pub width: Option<String>,
    pub height: Option<String>,
    pub min_width: Option<String>,
    pub min_height: Option<String>,
    pub max_width: Option<String>,
    pub max_height: Option<String>,

    // Visual
    pub opacity: Option<f64>,
    pub visibility: Option<String>,  // ✨ NEW
    pub background_color: Option<String>,
    pub color: Option<String>,
    pub border_color: Option<String>,
    pub border_radius: Option<String>,
    pub border_width: Option<String>,

    // Shadows
    pub shadow_offset_x: Option<f64>,
    pub shadow_offset_y: Option<f64>,
    pub shadow_blur: Option<f64>,
    pub shadow_spread: Option<f64>,
    pub shadow_color: Option<String>,

    // Filters
    pub blur: Option<f64>,
    pub brightness: Option<f64>,
    pub contrast: Option<f64>,
    pub saturate: Option<f64>,
    pub hue: Option<f64>,
    pub grayscale: Option<f64>,
    pub invert: Option<f64>,
    pub sepia: Option<f64>,

    // SVG
    pub stroke_dashoffset: Option<f64>,
    pub stroke_width: Option<f64>,
    pub fill_opacity: Option<f64>,
    pub stroke_opacity: Option<f64>,

    // Advanced
    pub transform_origin_x: Option<String>,
    pub transform_origin_y: Option<String>,
    pub transform_origin_z: Option<String>,
    pub perspective: Option<f64>,
    pub perspective_origin_x: Option<String>,
    pub perspective_origin_y: Option<String>,

    pub backface_visibility: Option<f64>,
    pub background_blur: Option<f64>,
    pub inset: Option<f64>,
}

// Helper functions
pub fn interpolate_value(
    start: &AnimatableValue,
    end: &AnimatableValue,
    t: f64,
) -> AnimatableValue {
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
        (AnimatableValue::Shadow(s1), AnimatableValue::Shadow(s2)) => {
            AnimatableValue::Shadow(ShadowValue {
                offset_x: s1.offset_x + (s2.offset_x - s1.offset_x) * t,
                offset_y: s1.offset_y + (s2.offset_y - s1.offset_y) * t,
                blur: s1.blur + (s2.blur - s1.blur) * t,
                spread: s1.spread + (s2.spread - s1.spread) * t,
                color: (
                    s1.color.0 + (s2.color.0 - s1.color.0) * t,
                    s1.color.1 + (s2.color.1 - s1.color.1) * t,
                    s1.color.2 + (s2.color.2 - s1.color.2) * t,
                    s1.color.3 + (s2.color.3 - s1.color.3) * t,
                ),
                inset: s1.inset,
            })
        }
        (AnimatableValue::Visibility(v1), AnimatableValue::Visibility(v2)) => {
            // Interpolate visibility as numbers, then convert back
            let v1_num = v1.to_number();
            let v2_num = v2.to_number();
            let interpolated = v1_num + (v2_num - v1_num) * t;
            AnimatableValue::Visibility(VisibilityValue::from_number(interpolated))
        }
        _ => start.clone(),
    }
}

pub fn extract_number(value: &AnimatableValue) -> f64 {
    match value {
        AnimatableValue::Number(n) => *n,
        AnimatableValue::Length(n, _) => *n,
        AnimatableValue::Shadow(s) => s.offset_x,
        AnimatableValue::Visibility(v) => v.to_number(),
        _ => 0.0,
    }
}

pub fn create_value_with_number(template: &AnimatableValue, num: f64) -> AnimatableValue {
    match template {
        AnimatableValue::Number(_) => AnimatableValue::Number(num),
        AnimatableValue::Length(_, u) => AnimatableValue::Length(num, u.clone()),
        AnimatableValue::Color(_, g, b, a) => AnimatableValue::Color(num, *g, *b, *a),
        AnimatableValue::Shadow(s) => {
            let mut shadow = s.clone();
            shadow.offset_x = num;
            AnimatableValue::Shadow(shadow)
        }
        AnimatableValue::Visibility(_) => AnimatableValue::Visibility(VisibilityValue::from_number(num)),
    }
}

pub fn format_value(value: &AnimatableValue) -> String {
    match value {
        AnimatableValue::Number(n) => n.to_string(),
        AnimatableValue::Length(n, u) => format!("{}{}", n, u.as_str()),
        AnimatableValue::Color(r, g, b, a) => {
            format!(
                "rgba({}, {}, {}, {})",
                r.round() as u8,
                g.round() as u8,
                b.round() as u8,
                a
            )
        }
        AnimatableValue::Shadow(s) => s.to_css_string(),
        AnimatableValue::Visibility(v) => v.as_str().to_string(),
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
    } else if value.ends_with("vw") {
        let num = value[..value.len() - 2]
            .parse::<f64>()
            .map_err(|_| "Invalid vw value".to_string())?;
        Ok((num, LengthUnit::Vw))
    } else if value.ends_with("vh") {
        let num = value[..value.len() - 2]
            .parse::<f64>()
            .map_err(|_| "Invalid vh value".to_string())?;
        Ok((num, LengthUnit::Vh))
    } else if value.ends_with("em") {
        let num = value[..value.len() - 2]
            .parse::<f64>()
            .map_err(|_| "Invalid em value".to_string())?;
        Ok((num, LengthUnit::Em))
    } else if value.ends_with("rem") {
        let num = value[..value.len() - 3]
            .parse::<f64>()
            .map_err(|_| "Invalid rem value".to_string())?;
        Ok((num, LengthUnit::Rem))
    } else {
        let num = value
            .parse::<f64>()
            .map_err(|_| "Invalid number".to_string())?;
        Ok((num, LengthUnit::Px))
    }
}

pub fn parse_css_color(value: &str) -> Result<(f64, f64, f64, f64), String> {
    let value = value.trim().to_lowercase();

    if value.starts_with('#') {
        let hex = &value[1..];
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f64;
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f64;
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f64;
            return Ok((r, g, b, 1.0));
        } else if hex.len() == 3 {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).unwrap_or(0) as f64;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).unwrap_or(0) as f64;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).unwrap_or(0) as f64;
            return Ok((r, g, b, 1.0));
        }
    } else if value.starts_with("rgb") {
        return parse_rgb_color(&value);
    }

    // Named colors
    match value.as_str() {
        "red" => Ok((255.0, 0.0, 0.0, 1.0)),
        "green" => Ok((0.0, 128.0, 0.0, 1.0)),
        "blue" => Ok((0.0, 0.0, 255.0, 1.0)),
        "white" => Ok((255.0, 255.0, 255.0, 1.0)),
        "black" => Ok((0.0, 0.0, 0.0, 1.0)),
        "transparent" => Ok((0.0, 0.0, 0.0, 0.0)),
        _ => Ok((0.0, 0.0, 0.0, 1.0)),
    }
}

fn parse_rgb_color(value: &str) -> Result<(f64, f64, f64, f64), String> {
    let start = value.find('(').ok_or("Invalid rgb format")?;
    let end = value.find(')').ok_or("Invalid rgb format")?;
    let content = &value[start + 1..end];
    let parts: Vec<&str> = content.split(',').collect();

    if parts.len() < 3 {
        return Err("RGB requires at least 3 values".to_string());
    }

    let r = parts[0].trim().parse::<f64>().unwrap_or(0.0);
    let g = parts[1].trim().parse::<f64>().unwrap_or(0.0);
    let b = parts[2].trim().parse::<f64>().unwrap_or(0.0);
    let a = if parts.len() > 3 {
        parts[3].trim().parse::<f64>().unwrap_or(1.0)
    } else {
        1.0
    };

    Ok((r, g, b, a))
}