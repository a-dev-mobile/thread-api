use crate::services::svg::enums::{FontFamily, FontWeight, TextAnchor};



#[derive(Debug, Clone, Copy)]
pub struct SvgTextOptions {
    pub font_size: f64,
    pub rotation_angle: f64,
    pub text_anchor: Option<TextAnchor>,
    pub font_weight: Option<FontWeight>,
    pub font_family: Option<FontFamily>,
}

impl SvgTextOptions {
    pub fn with(&self, updates: impl FnOnce(&mut SvgTextOptions)) -> Self {
        let mut options = *self;
        updates(&mut options);
        options
    }
}

#[derive(Debug, Clone)]
pub struct SvgText {
    pub x: f64,
    pub y: f64,
    pub value: String,
}

impl SvgText {
    pub fn new<S: Into<String>>(x: f64, y: f64, value: S) -> Self {
        Self {
            x,
            y,
            value: value.into(),
        }
    }
}
