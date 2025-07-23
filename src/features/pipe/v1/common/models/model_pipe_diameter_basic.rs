// Calculation models
#[derive(Debug)]
pub struct ModelPipeDiameterBasic {
    pub d: f64,  // Major diameter
    pub d1: f64, // Minor diameter (female)
    pub d2: f64, // Pitch diameter
    pub d3: f64, // Minor diameter (male)
    pub d4: f64, // Minimum Major Diameter (female)
}
