// text_options.rs

use crate::services::svg::enums::{FontFamily, FontWeight, TextAnchor};

use super::models::SvgTextOptions;

/// Trait defining various text styling options for SVG text elements
pub trait TextOptionsGenerator {
    fn create_default_text_options(&self) -> SvgTextOptions;
    fn text_option_40_0_center_bold(&self) -> SvgTextOptions;
    fn text_option_40_0_start_bold(&self) -> SvgTextOptions;
    fn text_option_40_0_end_bold(&self) -> SvgTextOptions;
    fn text_option_40_90_center_bold(&self) -> SvgTextOptions;
    fn create_right_aligned_text_options(&self) -> SvgTextOptions;
    fn create_vertical_text_options(&self) -> SvgTextOptions;
    fn create_monospace_text_options(&self) -> SvgTextOptions;
    fn create_light_text_options(&self) -> SvgTextOptions;
    fn create_emphasis_text_options(&self) -> SvgTextOptions;
    fn create_custom_text_options(
        &self,
        font_size: f64,
        rotation_angle: f64,
        text_anchor: TextAnchor,
        font_weight: FontWeight,
        font_family: FontFamily,
    ) -> SvgTextOptions;
}

impl TextOptionsGenerator for super::svg_service::SvgService {
    fn create_default_text_options(&self) -> SvgTextOptions {
        SvgTextOptions {
            font_size: 14.0,
            rotation_angle: 0.0,
            text_anchor: Some(TextAnchor::default()),
            font_weight: Some(FontWeight::default()),
            font_family: Some(FontFamily::default()),
        }
    }

    fn text_option_40_0_center_bold(&self) -> SvgTextOptions {
        self.create_default_text_options().with(|o| {
            o.font_size = 40.0;
            o.text_anchor = Some(TextAnchor::Middle);
            o.font_weight = Some(FontWeight::Bold);
            o.font_family = Some(FontFamily::Arial);
        })
    }
    fn create_right_aligned_text_options(&self) -> SvgTextOptions {
        self.create_default_text_options().with(|o| {
            o.text_anchor = Some(TextAnchor::End);
        })
    }
    fn text_option_40_0_start_bold(&self) -> SvgTextOptions {
        self.text_option_40_0_center_bold().with(|o| {
            o.text_anchor = Some(TextAnchor::Start);
        })
    }
    fn text_option_40_0_end_bold(&self) -> SvgTextOptions {
        self.text_option_40_0_center_bold().with(|o| {
            o.text_anchor = Some(TextAnchor::End);
        })
    }
    fn text_option_40_90_center_bold(&self) -> SvgTextOptions {
        self.text_option_40_0_center_bold().with(|o| {
            o.rotation_angle = -90.0;
        })
    }

    fn create_vertical_text_options(&self) -> SvgTextOptions {
        self.create_default_text_options().with(|o| {
            o.font_size = 12.0;
            o.rotation_angle = -90.0;
            o.text_anchor = Some(TextAnchor::Middle);
        })
    }

    fn create_monospace_text_options(&self) -> SvgTextOptions {
        self.create_default_text_options().with(|o| {
            o.font_size = 13.0;
            o.text_anchor = Some(TextAnchor::Start);
            o.font_weight = Some(FontWeight::W500);
            o.font_family = Some(FontFamily::Courier);
        })
    }

    fn create_light_text_options(&self) -> SvgTextOptions {
        self.create_default_text_options().with(|o| {
            o.font_size = 12.0;
            o.font_weight = Some(FontWeight::W300);
        })
    }

    fn create_emphasis_text_options(&self) -> SvgTextOptions {
        self.create_default_text_options().with(|o| {
            o.font_size = 16.0;
            o.text_anchor = Some(TextAnchor::Middle);
            o.font_weight = Some(FontWeight::W700);
        })
    }

    fn create_custom_text_options(
        &self,
        font_size: f64,
        rotation_angle: f64,
        text_anchor: TextAnchor,
        font_weight: FontWeight,
        font_family: FontFamily,
    ) -> SvgTextOptions {
        self.create_default_text_options().with(|o| {
            o.font_size = font_size;
            o.rotation_angle = rotation_angle;
            o.text_anchor = Some(text_anchor);
            o.font_weight = Some(font_weight);
            o.font_family = Some(font_family);
        })
    }
}
