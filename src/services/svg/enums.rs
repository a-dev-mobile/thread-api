use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, Display, EnumString, Default)] // Added Copy trait
#[strum(serialize_all = "lowercase")]
pub enum TextAnchor {
    Start,
    #[default]
    Middle,
    End,
}

#[derive(Debug, Clone, Copy, Display, EnumString, Default)] // Added Copy trait
pub enum FontWeight {
    #[strum(serialize = "normal")]
    #[default]
    Normal,
    #[strum(serialize = "bold")]
    Bold,
    #[strum(serialize = "lighter")]
    Lighter,
    #[strum(serialize = "bolder")]
    Bolder,
    #[strum(serialize = "100")]
    W100,
    #[strum(serialize = "200")]
    W200,
    #[strum(serialize = "300")]
    W300,
    #[strum(serialize = "400")]
    W400,
    #[strum(serialize = "500")]
    W500,
    #[strum(serialize = "600")]
    W600,
    #[strum(serialize = "700")]
    W700,
    #[strum(serialize = "800")]
    W800,
    #[strum(serialize = "900")]
    W900,
}

#[derive(Debug, Clone, Display, Default, Copy)]
pub enum FontFamily {
    #[strum(serialize = "Arial")]
    #[default]
    Arial,
    #[strum(serialize = "Helvetica")]
    Helvetica,
    #[strum(serialize = "Times New Roman")]
    TimesNewRoman,
    #[strum(serialize = "Verdana")]
    Verdana,
    #[strum(serialize = "Georgia")]
    Georgia,
    #[strum(serialize = "Courier")]
    Courier,
    #[strum(serialize = "Comic Sans MS")]
    ComicSans,
    #[strum(serialize = "Impact")]
    Impact,
}
