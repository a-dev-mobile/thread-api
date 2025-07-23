use crate::features::pipe::v1::common::models::model_pipe_db::ModelPipeDB;

use super::models::{ModelPipeDiameter, ResponsePipeDiameters};

impl From<Vec<ModelPipeDB>> for ResponsePipeDiameters {
    fn from(records: Vec<ModelPipeDB>) -> Self {
        let mut male_diameters = Vec::new();
        let mut female_diameters = Vec::new();

        for db in records {
            let is_male = db.class_name.is_some();
            let decimal_diam = if is_male {
                db.ex_major_dia_max.unwrap_or_default()
            } else {
                db.in_major_dia_min.unwrap_or_default()
            };

            let diameter = ModelPipeDiameter {
                id: db.id,
                fractional: format!("G {} - {}", db.designation_2, db.thread_per),
                decimal: format!("G {} x {}", decimal_diam, db.thread_pitch),
                tolerance: db.class_name.clone(),
            };
            if is_male {
                male_diameters.push(diameter);
            } else {
                female_diameters.push(diameter);
            }
        }

        ResponsePipeDiameters {
            male: male_diameters,
            female: female_diameters,
        }
    }
}
