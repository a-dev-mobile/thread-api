pub fn generate_svg_text(
    x: f64,
    y: f64,
    value: &str,
    theme: &str,
    font_size: f64,
    rotation_angle: f64,
    text_anchor: Option<&str>,
) -> String {
    let fill_color = if theme.eq_ignore_ascii_case("light") {
        "black"
    } else {
        "white"
    };
    let text_anchor = text_anchor.unwrap_or("middle");

    format!(
        r#"<text x="{x}" y="{y}" font-size="{font_size}" fill="{fill_color}" text-anchor="{text_anchor}" font-family="Arial" transform="rotate({rotation_angle} {x} {y})">{value}</text>"#,
        x = x,
        y = y,
        font_size = font_size,
        fill_color = fill_color,
        text_anchor = text_anchor,
        rotation_angle = rotation_angle,
        value = value,
    )
}
