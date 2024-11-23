// Crop yield and harvest tracking system backend

use serde::{Deserialize, Serialize};
use candid::CandidType;
use ic_cdk_macros::{update, query};
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]  // Add Serialize and Deserialize traits
pub struct Crop {
    id: u64,
    crop_type: String,
    variety: String,
    field_location: String,
    planting_date: String,
    weather_conditions: String,
    expected_yield: f64,
    actual_yield: f64,
    infection_monitoring: HashMap<String, String>,
    pest_and_disease_details: HashMap<String, String>,
    soil_quality: HashMap<String, String>,
    ph_level: f64,
    expected_rainfall: f64,
}

// Implement the Crop struct
impl Crop {
    pub fn new(
        id: u64,
        crop_type: String,
        variety: String,
        field_location: String,
        planting_date: String,
        weather_conditions: String,
        expected_yield: f64,
        actual_yield: f64,
        infection_monitoring: HashMap<String, String>,
        pest_and_disease_details: HashMap<String, String>,
        soil_quality: HashMap<String, String>,
        ph_level: f64,
        expected_rainfall: f64,
    ) -> Self {
        Crop {
            id,
            crop_type,
            variety,
            field_location,
            planting_date,
            weather_conditions,
            expected_yield,
            actual_yield,
            infection_monitoring,
            pest_and_disease_details,
            soil_quality,
            ph_level,
            expected_rainfall,
        }
    }
}

// Global variable to store the crop details
thread_local! {
    static CROPS: RefCell<HashMap<u64, Crop>> = RefCell::new(HashMap::new());
}

// Create a new Crop instance
#[update]
pub fn create_crop(
    id: u64,
    crop_type: String,
    variety: String,
    field_location: String,
    planting_date: String,
    weather_conditions: String,
    expected_yield: f64,
    actual_yield: f64,
    infection_monitoring: HashMap<String, String>,
    pest_and_disease_details: HashMap<String, String>,
    soil_quality: HashMap<String, String>,
    ph_level: f64,
    expected_rainfall: f64,
) -> Crop {
    let crop = Crop::new(
        id,
        crop_type,
        variety,
        field_location,
        planting_date,
        weather_conditions,
        expected_yield,
        actual_yield,
        infection_monitoring,
        pest_and_disease_details,
        soil_quality,
        ph_level,
        expected_rainfall,
    );
    CROPS.with(|crops|{
        crops.borrow_mut().insert(id, crop.clone());
    });
    crop
}

// Get the crop details by ID
#[query]
pub fn get_crop_details(id: u64) -> Option<Crop> {
    CROPS.with(|crops| {
        crops.borrow().get(&id).cloned()
    })
}

// Update the crop details
#[update]
pub fn update_crop_details(
    crop_id: u64,
    actual_yield: f64,
    infection_monitoring: HashMap<String, String>,
    pest_and_disease_details: HashMap<String, String>,
    soil_quality: HashMap<String, String>,
    ph_level: f64,
    expected_rainfall: f64,
) -> Option<Crop> {
    CROPS.with(|crops| {
        let mut crops = crops.borrow_mut();
        if let Some(crop) = crops.get_mut(&crop_id) {
            crop.actual_yield = actual_yield;
            crop.infection_monitoring = infection_monitoring;
            crop.pest_and_disease_details = pest_and_disease_details;
            crop.soil_quality = soil_quality;
            crop.ph_level = ph_level;
            crop.expected_rainfall = expected_rainfall;
            Some(crop.clone())
        } else {
            None
        }
    })
}

// Delete the crop details by ID
#[update]
pub fn delete_crop(id: u64) -> Option<Crop> {
    CROPS.with(|crops| {
        crops.borrow_mut().remove(&id)
    })
}

// Export the candid functions

ic_cdk::export_candid!(); 
