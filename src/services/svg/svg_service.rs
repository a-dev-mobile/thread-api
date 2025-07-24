use axum::{
    body::Body,
    http::{header::CONTENT_TYPE, HeaderMap, HeaderValue, Response, StatusCode},
    response::IntoResponse,
};
use std::path::PathBuf;
use tokio::fs;

use crate::{
    shared::enums::{Theme, ThreadStandard, ThreadType},
    shared::error::AppError,
};

use super::models::{SvgText, SvgTextOptions};

pub struct SvgService {
    base_path: PathBuf,
}

impl SvgService {
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }

    /// Loads SVG template based on thread type and theme
    pub async fn load_template(
        &self,
        thread_standard: ThreadStandard,
        thread_type: ThreadType,
        theme: Theme,
    ) -> Result<String, AppError> {
        let file_name = self.generate_file_name(thread_type, theme, thread_standard);
        self.read_svg_file(&file_name).await
    }

    /// Generates file name based on parameters
    fn generate_file_name(
        &self,
        enum_type_thread: ThreadType,
        enum_theme: Theme,
        enum_thread: ThreadStandard,
    ) -> String {
        format!("{}-thread-{}-{}.svg", enum_thread, enum_type_thread, enum_theme)
    }

    /// Reads SVG file from filesystem
    async fn read_svg_file(&self, file_name: &str) -> Result<String, AppError> {
        let file_path = self.base_path.join(file_name);

        fs::read_to_string(file_path)
            .await
            .map_err(|e| AppError::FileSystemError(e.to_string()))
    }

    /// Generates SVG text element with specified parameters
    pub fn generate_svg_text(&self, item: &SvgText, theme: &Theme, options: &SvgTextOptions) -> String {
        let fill_color = match theme {
            Theme::Light => "black",
            Theme::Dark => "white",
        };

        let text_anchor = options.text_anchor.unwrap_or_default().to_string();
        let font_weight = options.font_weight.unwrap_or_default().to_string();
        let font_family = options.font_family.unwrap_or_default().to_string();

        format!(
            r#"<text
                x="{x}"
                y="{y}"
                font-size="{font_size}"
                font-weight="{font_weight}"
                fill="{fill_color}"
                text-anchor="{text_anchor}"
                font-family="{font_family}"
                transform="rotate({rotation_angle} {x} {y})"
            >{value}</text>"#,
            x = item.x,
            y = item.y,
            font_size = options.font_size,
            fill_color = fill_color,
            text_anchor = text_anchor,
            font_weight = font_weight,
            font_family = font_family,
            rotation_angle = options.rotation_angle,
            value = item.value,
        )
    }
    // Add new text elements to existing SVG content
    pub fn append_text_elements(
        &self,
        content: String,
        items: Vec<(SvgText, SvgTextOptions)>,
        theme: &Theme,
    ) -> String {
        // Find the closing tag of the SVG
        if let Some(last_tag_pos) = content.rfind("</svg>") {
            let (base_content, _) = content.split_at(last_tag_pos);

            // Generate all text elements
            let text_elements: String = items
                .iter()
                .map(|(item, options)| self.generate_svg_text(item, theme, options))
                .collect();

            // Combine original content, new text elements, and closing tag
            format!("{}{}</svg>", base_content, text_elements)
        } else {
            content // Return original content if no </svg> tag found
        }
    }

    // Convenience method to append a single text element
    pub fn append_text_element(
        &self,
        content: String,
        item: SvgText,
        options: SvgTextOptions,
        theme: &Theme,
    ) -> String {
        self.append_text_elements(content, vec![(item, options)], theme)
    }
    /// Creates HTTP response with SVG content
    pub fn create_svg_response(&self, content: String) -> Response<Body> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("image/svg+xml; charset=utf-8"));

        (StatusCode::OK, headers, content.into_bytes()).into_response()
    }
}
