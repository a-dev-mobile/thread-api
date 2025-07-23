#[derive(Debug)]
pub struct ModelPipeOtherDimensions {
    pub p: f64,      // Pitch
    pub a_c: f64,    // Thread profile angle
    pub h4_h3: f64,  // Thread depth difference
    pub h1: f64,     // Basic profile height
    pub r1_max: f64, // Maximum fillet radius (crest)
    pub r2_max: f64, // Maximum fillet radius (root)
    pub z: f64,
}
