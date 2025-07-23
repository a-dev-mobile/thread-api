use crate::{features::metric::models::ThreadInfo, shared::utils::svg::generate_svg_text,};

use super::coords::SvgTextCoordinates;

pub fn generate_svg_texts(
    thread_info: &ThreadInfo,
    type_: &str,
    coords: &SvgTextCoordinates,
    theme: &str,
    language: &str,
    show_dimensions: bool,
) -> String {
    let major_diam_label = if type_.eq_ignore_ascii_case("female") {
        "D"
    } else {
        "d"
    };

    let pitch_diam_label = if type_.eq_ignore_ascii_case("female") {
        "D2"
    } else {
        "d2"
    };

    let minor_diam_label = if type_.eq_ignore_ascii_case("female") {
        "D1"
    } else {
        "d1"
    };

    let avg_label = if language.eq_ignore_ascii_case("ru") {
        "сред."
    } else {
        "avg."
    };
    let female_thread_label = if language.eq_ignore_ascii_case("ru") {
        "Внутренняя резьба"
    } else {
        "Internal thread"
    };
    let male_thread_label = if language.eq_ignore_ascii_case("ru") {
        "Наружная резьба"
    } else {
        "External thread"
    };

    let svg_text_major_diam = generate_svg_text(
        coords.major_diam_x,
        coords.major_diam_y,
        &if show_dimensions {
            format!(
                "ø{}-{}",
                thread_info.major_diam_min, thread_info.major_diam_max
            )
        } else {
            major_diam_label.to_string()
        },
        theme,
        20.0,
        -90.0,
        None,
    );

    let svg_text_major_diam_avg = if show_dimensions {
        generate_svg_text(
            coords.major_diam_avg_x,
            coords.major_diam_avg_y,
            &format!("({} ø{})", avg_label, thread_info.major_diam_avg),
            theme,
            10.0,
            -90.0,
            None,
        )
    } else {
        String::new()
    };

    let svg_text_pitch_tolerance = generate_svg_text(
        coords.pitch_diam_x,
        coords.pitch_diam_y,
        &if show_dimensions {
            format!(
                "ø{}-{}",
                thread_info.pitch_diam_min, thread_info.pitch_diam_max
            )
        } else {
            pitch_diam_label.to_string()
        },
        theme,
        20.0,
        -90.0,
        None,
    );

    let svg_text_pitch_tolerance_avg = if show_dimensions {
        generate_svg_text(
            coords.pitch_diam_avg_x,
            coords.pitch_diam_avg_y,
            &format!("({} ø{})", avg_label, thread_info.pitch_diam_avg),
            theme,
            10.0,
            -90.0,
            None,
        )
    } else {
        String::new()
    };

    let svg_text_minor_tolerance = generate_svg_text(
        coords.minor_diam_x,
        coords.minor_diam_y,
        &if show_dimensions {
            format!(
                "ø{}-{}",
                thread_info.minor_diam_min, thread_info.minor_diam_max
            )
        } else {
            minor_diam_label.to_string()
        },
        theme,
        20.0,
        -90.0,
        None,
    );

    let svg_text_minor_tolerance_avg = if show_dimensions {
        generate_svg_text(
            coords.minor_diam_avg_x,
            coords.minor_diam_avg_y,
            &format!("({} ø{})", avg_label, thread_info.minor_diam_avg),
            theme,
            10.0,
            -90.0,
            None,
        )
    } else {
        String::new()
    };

    let svg_text_pitch = generate_svg_text(
        coords.pitch_x,
        coords.pitch_y,
        &if show_dimensions {
            thread_info.pitch.to_string()
        } else {
            "P".to_string()
        },
        theme,
        20.0,
        0.0,
        None,
    );

    let svg_text_deg = generate_svg_text(
        coords.angle_x,
        coords.angle_y,
        "60°",
        theme,
        20.0,
        0.0,
        None,
    );

    let svg_text_thread_depth = generate_svg_text(
        coords.thread_depth_x,
        coords.thread_depth_y,
        &if show_dimensions {
            thread_info.thread_depth.to_string()
        } else if type_.eq_ignore_ascii_case("male") {
            "(h3) 17H/24".to_string()
        } else {
            "5H/8".to_string()
        },
        theme,
        20.0,
        -90.0,
        None,
    );

    let svg_text_h = generate_svg_text(
        coords.h_x,
        coords.h_y,
        &if show_dimensions {
            thread_info.h.to_string()
        } else {
            "H".to_string()
        },
        theme,
        20.0,
        -90.0,
        None,
    );

    let svg_text_h_div_4 = generate_svg_text(
        coords.h_div_4_x,
        coords.h_div_4_y,
        &if show_dimensions {
            thread_info.h_div_4.to_string()
        } else {
            "H/4".to_string()
        },
        theme,
        20.0,
        0.0,
        Some("end"),
    );

    let svg_text_h_div_8 = generate_svg_text(
        coords.h_div_8_x,
        coords.h_div_8_y,
        &if show_dimensions {
            thread_info.h_div_8.to_string()
        } else {
            "H/8".to_string()
        },
        theme,
        if show_dimensions { 10.0 } else { 15.0 },
        -90.0,
        Some("start"),
    );

    let svg_text_pitch_div_4 = generate_svg_text(
        coords.pitch_div_4_x,
        coords.pitch_div_4_y,
        &if show_dimensions {
            thread_info.pitch_div_4.to_string()
        } else {
            "P/4".to_string()
        },
        theme,
        20.0,
        0.0,
        Some("end"),
    );

    let svg_text_r = generate_svg_text(
        coords.r_x,
        coords.r_y,
        &if show_dimensions {
            format!("R{}-{}", thread_info.rmin, thread_info.rmax)
        } else {
            "R".to_string()
        },
        theme,
        if show_dimensions { 10.0 } else { 20.0 },
        0.0,
        Some("end"),
    );

    let svg_text_c = generate_svg_text(
        coords.c_x,
        coords.c_y,
        &if show_dimensions {
            format!("{}-{}", thread_info.cmin, thread_info.cmax)
        } else {
            "C".to_string()
        },
        theme,
        if show_dimensions { 10.0 } else { 20.0 },
        0.0,
        Some("start"),
    );

    let svg_text_pitch_div_2 = generate_svg_text(
        coords.pitch_div_2_x,
        coords.pitch_div_2_y,
        &if show_dimensions {
            thread_info.pitch_div_2.to_string()
        } else {
            "P/2".to_string()
        },
        theme,
        20.0,
        0.0,
        None,
    );

    let svg_text_pitch_div_8 = generate_svg_text(
        coords.pitch_div_8_x,
        coords.pitch_div_8_y,
        &if show_dimensions {
            thread_info.pitch_div_8.to_string()
        } else {
            "P/8".to_string()
        },
        theme,
        if show_dimensions { 15.0 } else { 20.0 },
        0.0,
        None,
    );

    let svg_text_coord_female_thread = generate_svg_text(
        coords.coord_female_thread_x,
        coords.coord_female_thread_y,
        female_thread_label,
        theme,
        20.0,
        0.0,
        Some("start"),
    );

    let svg_text_coord_male_thread = generate_svg_text(
        coords.coord_male_thread_x,
        coords.coord_male_thread_y,
        male_thread_label,
        theme,
        20.0,
        0.0,
        Some("start"),
    );

    let svg_text_three_h_div_8 = generate_svg_text(
        coords.three_h_div_8_x,
        coords.three_h_div_8_y,
        &if show_dimensions {
            thread_info.three_h_div_8.to_string()
        } else {
            "3H/8".to_string()
        },
        theme,
        20.0,
        -90.0,
        None,
    );

    let mut svg_texts_vec = vec![
        svg_text_coord_female_thread,
        svg_text_coord_male_thread,
        svg_text_three_h_div_8,
        svg_text_h_div_8,
        svg_text_pitch_div_8,
        svg_text_pitch_div_2,
        svg_text_pitch_div_4,
        svg_text_h_div_4,
        svg_text_r,
        svg_text_c,
        svg_text_h,
        svg_text_major_diam,
        svg_text_pitch_tolerance,
        svg_text_minor_tolerance,
        svg_text_major_diam_avg,
        svg_text_pitch,
        svg_text_deg,
        svg_text_thread_depth,
    ];

    if show_dimensions {
        svg_texts_vec.push(svg_text_pitch_tolerance_avg);
        svg_texts_vec.push(svg_text_minor_tolerance_avg);
    }

    if type_.eq_ignore_ascii_case("male") {
        let svg_text_d3 = generate_svg_text(
            coords.d3_x,
            coords.d3_y,
            &if show_dimensions {
                format!(
                    "ø{}-{}",
                    thread_info.minor_diam_min_d3.unwrap_or(0.0),
                    thread_info.minor_diam_max_d3.unwrap_or(0.0)
                )
            } else {
                "d3".to_string()
            },
            theme,
            if show_dimensions { 15.0 } else { 20.0 },
            -90.0,
            None,
        );

        let svg_text_d3_avg = if show_dimensions {
            generate_svg_text(
                coords.d3_x_avg,
                coords.d3_y_avg,
                &format!(
                    "({} ø{:?})",
                    avg_label,
                    thread_info.minor_diam_avg_d3.unwrap()
                ),
                theme,
                10.0,
                -90.0,
                None,
            )
        } else {
            String::new()
        };

        svg_texts_vec.push(svg_text_d3);
        svg_texts_vec.push(svg_text_d3_avg);
    }

    svg_texts_vec.join("")
}
