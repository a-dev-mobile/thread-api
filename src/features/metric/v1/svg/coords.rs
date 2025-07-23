/// Функция для наполнения координат текстовых элементов SVG на основе типа
pub fn initialize(type_: &str) -> SvgTextCoordinates {
    let is_female = type_ == "female";

    SvgTextCoordinates {
        major_diam_x: 1047.0 - 15.0,
        major_diam_y: 470.5,
        major_diam_avg_x: 1047.0 - 5.0,
        major_diam_avg_y: 470.5,
        pitch_diam_x: 966.5 - 15.0,
        pitch_diam_y: 559.5,
        pitch_diam_avg_x: 966.5 - 5.0,
        pitch_diam_avg_y: 559.5,
        minor_diam_x: 883.5 - 15.0,
        minor_diam_y: 651.0,
        minor_diam_avg_x: 883.5 - 5.0,
        minor_diam_avg_y: 651.0,

        d3_x: 800.0 - 15.0,
        d3_y: 679.0,
        d3_x_avg: 800.0 - 5.0,
        d3_y_avg: 679.0,

        pitch_x: 600.0,
        pitch_y: 67.5 - 5.0,
        angle_x: 267.5 + 2.0,
        angle_y: 466.0 - 5.0,
        thread_depth_x: 709.5 - 5.0,
        thread_depth_y: 360.0,
        h_x: 488.0 - 5.0,
        h_y: 381.0,
        h_div_4_x: 656.0 - 5.0,
        h_div_4_y: 546.0 + 7.0,
        h_div_8_x: 562.5 + 3.0,
        h_div_8_y: 156.0 - 2.0,
        three_h_div_8_x: 651.5 - 5.0,
        three_h_div_8_y: 281.0,
        pitch_div_4_x: 673.0 - 5.0,
        pitch_div_4_y: 596.0 + 7.0,

        pitch_div_2_x: 489.0,
        pitch_div_2_y: 104.5 - 5.0,

        pitch_div_8_x: 489.0,
        pitch_div_8_y: 138.9 - 7.0,

        coord_female_thread_x: 111.5,
        coord_female_thread_y: 67.5,

        coord_male_thread_x: 117.0,
        coord_male_thread_y: 683.5,

        // Условные поля на основе типа
        r_x: if is_female { 455.0 } else { 227.5 },
        r_y: if is_female { 173.0 + 2.0 } else { 534.5 + 2.0 },
        c_x: if is_female { 531.0 } else { 304.5 },
        c_y: if is_female { 218.0 + 2.0 } else { 555.0 + 2.0 },
    }
}

#[derive(Debug)]
pub struct SvgTextCoordinates {
    pub coord_male_thread_x: f64,
    pub coord_male_thread_y: f64,
    pub coord_female_thread_x: f64,
    pub coord_female_thread_y: f64,
    pub major_diam_x: f64,
    pub major_diam_y: f64,

    pub major_diam_avg_x: f64,
    pub major_diam_avg_y: f64,
    pub pitch_diam_x: f64,
    pub pitch_diam_y: f64,
    pub pitch_diam_avg_x: f64,
    pub pitch_diam_avg_y: f64,
    pub minor_diam_x: f64,
    pub minor_diam_y: f64,
    pub minor_diam_avg_x: f64,
    pub minor_diam_avg_y: f64,
    pub d3_x: f64,
    pub d3_y: f64,
    pub r_x: f64,
    pub r_y: f64,

    pub c_x: f64,
    pub c_y: f64,
    pub d3_x_avg: f64,
    pub d3_y_avg: f64,
    pub pitch_x: f64,
    pub pitch_y: f64,
    pub angle_x: f64,
    pub angle_y: f64,
    pub thread_depth_x: f64,
    pub thread_depth_y: f64,
    pub h_x: f64,
    pub h_y: f64,
    pub h_div_4_x: f64,
    pub h_div_4_y: f64,
    pub three_h_div_8_x: f64,
    pub three_h_div_8_y: f64,
    pub h_div_8_x: f64,
    pub h_div_8_y: f64,
    pub pitch_div_8_x: f64,
    pub pitch_div_8_y: f64,
    pub pitch_div_2_x: f64,
    pub pitch_div_2_y: f64,
    pub pitch_div_4_x: f64,
    pub pitch_div_4_y: f64,
}
