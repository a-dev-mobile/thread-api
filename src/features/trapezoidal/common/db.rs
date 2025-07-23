// src/routes/v1/trapezoidal/core/db.rs

use sqlx::{PgPool, Row};
use tracing::{debug, error};

use crate::{
   shared::enums::ThreadType,
    features::trapezoidal::common::models::{
        ModelTrapezoidalDiameterBasic, ModelTrapezoidalOtherDimensions, ModelTrapezoidalTolerance,
    },
};

pub struct ThreadDataService {
    pool: PgPool,
}

#[derive(Debug)]
pub struct ThreadData {
    pub basic_diameters: ModelTrapezoidalDiameterBasic,
    pub tolerances: ModelTrapezoidalTolerance,
    pub other_dimensions: ModelTrapezoidalOtherDimensions,
}

impl ThreadDataService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn fetch_thread_data(
        &self,
        diameter: i32,
        pitch: f64,
        thread_type: ThreadType,
        tolerance: &str,
    ) -> Result<ThreadData, sqlx::Error> {
        let main_query = "SELECT * FROM trapezoidal.main WHERE diameter = $1::integer AND pitch = $2::double precision";
        let basic_dim_query =
            "SELECT * FROM trapezoidal.basic_dimensions WHERE p = $1::double precision";

        debug!(
            "Executing queries with params: diameter={}, pitch={}",
            diameter, pitch
        );

        let main_row = sqlx::query(main_query)
            .bind(diameter)
            .bind(pitch)
            .fetch_one(&self.pool)
            .await?;

        let basic_dim_row = sqlx::query(basic_dim_query)
            .bind(pitch)
            .fetch_one(&self.pool)
            .await?;

        let type_suffix = match thread_type {
            ThreadType::Male => "_m",
            ThreadType::Female => "_f",
        };
        let tolerance = tolerance.to_lowercase();

        Ok(ThreadData {
            basic_diameters: self.extract_basic_diameters(&main_row, diameter)?,
            tolerances: self.extract_tolerances(&main_row, &tolerance, type_suffix)?,
            other_dimensions: self.extract_basic_dimensions(&basic_dim_row)?,
        })
    }

    fn extract_basic_diameters(
        &self,
        row: &sqlx::postgres::PgRow,
        diameter: i32,
    ) -> Result<ModelTrapezoidalDiameterBasic, sqlx::Error> {
        Ok(ModelTrapezoidalDiameterBasic {
            d: diameter as f64,
            d1: self.get_f64(row, "d1")?,
            d2: self.get_f64(row, "d2_d2")?,
            d3: self.get_f64(row, "d3")?,
            d4: self.get_f64(row, "d4")?,
        })
    }

    fn extract_tolerances(
        &self,
        row: &sqlx::postgres::PgRow,
        tolerance: &str,
        type_suffix: &str,
    ) -> Result<ModelTrapezoidalTolerance, sqlx::Error> {
        // For female threads (_f), some tolerances should be 0.0
        let is_female = type_suffix == "_f";
        let is_male = type_suffix == "_m";

        Ok(ModelTrapezoidalTolerance {
            // For female threads, es_d and ei_d are not used
            es_d: if is_female {
                0.0
            } else {
                self.get_tolerance(row, &format!("es_d_{}{}", tolerance, type_suffix))?
            },
            ei_d: if is_female {
                0.0
            } else {
                self.get_tolerance(row, &format!("ei_d_{}{}", tolerance, type_suffix))?
            },

            es_d1: if is_male {
                0.0
            } else {
                self.get_tolerance(row, &format!("es_d1_{}{}", tolerance, type_suffix))?
            },

            ei_d1: if is_male {
                0.0
            } else {
                self.get_tolerance(row, &format!("ei_d1_{}{}", tolerance, type_suffix))?
            },

            es_d2: self.get_tolerance(row, &format!("es_d2_{}{}", tolerance, type_suffix))?,
            ei_d2: self.get_tolerance(row, &format!("ei_d2_{}{}", tolerance, type_suffix))?,
            // For female threads, es_d3 and ei_d3 are not used
            es_d3: if is_female {
                0.0
            } else {
                self.get_tolerance(row, &format!("es_d3_{}{}", tolerance, type_suffix))?
            },
            ei_d3: if is_female {
                0.0
            } else {
                self.get_tolerance(row, &format!("ei_d3_{}{}", tolerance, type_suffix))?
            },
            ei_d4: if is_female {
                self.get_tolerance(row, &format!("ei_d4_{}{}", tolerance, type_suffix))?
            } else {
                0.0
            },
        })
    }

    fn extract_basic_dimensions(
        &self,
        row: &sqlx::postgres::PgRow,
    ) -> Result<ModelTrapezoidalOtherDimensions, sqlx::Error> {
        Ok(ModelTrapezoidalOtherDimensions {
            p: self.get_f64(row, "p")?,
            a_c: self.get_f64(row, "a_c")?,
            h4_h3: self.get_f64(row, "h4_h3")?,
            h1: self.get_f64(row, "h1")?,
            r1_max: self.get_f64(row, "r1_max")?,
            r2_max: self.get_f64(row, "r2_max")?,
            z: self.get_f64(row, "z")?,
        })
    }

    fn get_f64(&self, row: &sqlx::postgres::PgRow, name: &str) -> Result<f64, sqlx::Error> {
        match row.try_get::<f64, _>(name) {
            Ok(v) => Ok(v),
            Err(_) => match row.try_get::<i32, _>(name) {
                Ok(v) => Ok(v as f64),
                Err(e) => {
                    error!("Error getting value for {}: {}", name, e);
                    Err(e)
                }
            },
        }
    }

    fn get_tolerance(&self, row: &sqlx::postgres::PgRow, name: &str) -> Result<f64, sqlx::Error> {
        match row.try_get::<i32, _>(name) {
            Ok(v) => Ok((v as f64) / 1000.0),
            Err(e) => {
                error!("Error getting tolerance for {}: {}", name, e);
                Err(e)
            }
        }
    }
}
