#[derive(Debug)]
pub struct ModelPipeTolerance {
    pub es_d: f64,  // Upper deviation for major diameter
    pub ei_d: f64,  // Lower deviation for major diameter
    pub es_d1: f64, // Upper deviation for minor diameter (female)
    pub ei_d1: f64, // Lower deviation for minor diameter (female)
    pub es_d2: f64, // Upper deviation for pitch diameter
    pub ei_d2: f64, // Lower deviation for pitch diameter
    pub es_d3: f64, // Upper deviation for minor diameter (male)
    pub ei_d3: f64, // Lower deviation for minor diameter (male)
    pub ei_d4: f64, // Lower deviation for Major diameter (female)
}
