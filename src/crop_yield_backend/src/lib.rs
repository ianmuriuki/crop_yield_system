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
    weather_conditions: WeatherConditions,
    expected_yield: f64,
    actual_yield: f64,
    infection_monitoring: HashMap<String, String>,
    pest_and_disease_details: HashMap<String, String>,
    soil_quality: HashMap<String, String>,
    ph_level: f64,
    expected_rainfall: f64,
    growth_stage: GrowthStage,
    fertilizer_application: Vec<Application>,
    pesticides_application: Vec<Application>,
}

// Pesticides and Fertilizer Application tracking struct
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct Application {
    date: String,
    product_name: String,
    quantity: f64,
}

// Crop growth stage tracking struct
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub enum GrowthStage {
    Planting, // Initial stage
    Germination, // Second stage
    Vegetative, //  Third stage
    Flowering, // Fourth stage
    Fruiting, // Fifth stage
    Ripening, // Sixth stage
    Harvesting, // Seventh stage
}

// Weather conditions struct
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct WeatherConditions {
    conditions: String,
    rainfall: f64,
    temperature: f64,
}


// Implement the Crop struct
impl Crop {
    pub fn new(
        id: u64,
        crop_type: String,
        variety: String,
        field_location: String,
        planting_date: String,
        weather_conditions: WeatherConditions,
        expected_yield: f64,
        actual_yield: f64,
        infection_monitoring: HashMap<String, String>,
        pest_and_disease_details: HashMap<String, String>,
        soil_quality: HashMap<String, String>,
        ph_level: f64,
        expected_rainfall: f64,
        growth_stage: GrowthStage,
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
            growth_stage,
            fertilizer_application: Vec::new(),
            pesticides_application: Vec::new(),
        }
    }
}

// Global variable to store the crop details
thread_local! {
    static CROPS: RefCell<HashMap<u64, Crop>> = RefCell::new(HashMap::new());
    static COUNTER: RefCell<u64> = RefCell::new(0);
}

// Create a new Crop instance
#[update]
pub fn create_crop(
    crop_type: String,
    variety: String,
    field_location: String,
    planting_date: String,
    weather_conditions: WeatherConditions,  
    expected_yield: f64,
    actual_yield: f64,
    infection_monitoring: HashMap<String, String>,
    pest_and_disease_details: HashMap<String, String>,
    soil_quality: HashMap<String, String>,
    ph_level: f64,
    expected_rainfall: f64,
) -> Crop {

    let id = COUNTER.with(|counter| {
        let current = *counter.borrow();
        *counter.borrow_mut() = current + 1;
        current
    });
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
        GrowthStage::Planting,
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

// A function to update crop growth stage
#[update]
pub fn update_growth_stage(crop_id: u64, growth_stage: GrowthStage) -> Option<Crop> {
    CROPS.with(|crops| {
        let mut crops = crops.borrow_mut();
        if let Some(crop) = crops.get_mut(&crop_id) {
            crop.growth_stage = growth_stage;
            Some(crop.clone())
        } else {
            None
        }
    })
}

// Query to get growth stage of a crop
#[query]
pub fn get_growth_stage(crop_id: u64) -> Option<GrowthStage> {
    CROPS.with(|crops| {
        crops.borrow().get(&crop_id).map(|crop| crop.growth_stage.clone())
    })
}

// Get all the crop details function
#[query]
pub fn get_all_crops() -> Vec<Crop> {
    CROPS.with(|crops| {
        crops.borrow().values().cloned().collect()
    })
}


// Fertilizer application and tracking function
#[update]
pub fn log_fertilizer_application(crop_id: u64, application: Application) -> Option<Crop> {
    CROPS.with(|crops| {
        let mut crops = crops.borrow_mut();
        if let Some(crop) = crops.get_mut(&crop_id) {
            crop.fertilizer_application.push(application);
            Some(crop.clone())
        } else {
            None
        }
    })
}


// Query to get the fertilizer application details
#[query]
pub fn get_fertilizer_application(crop_id: u64) -> Option<Vec<Application>> {
    CROPS.with(|crops| {
        crops.borrow().get(&crop_id).map(|crop| crop.fertilizer_application.clone())
    })
}


// A function for logging pesticides application
#[update]
pub fn log_pesticides_application(crop_id: u64, application: Application) -> Option<Crop> {
    CROPS.with(|crops| {
        let mut crops = crops.borrow_mut();
        if let Some(crop) = crops.get_mut(&crop_id) {
            crop.pesticides_application.push(application);
            Some(crop.clone())
        } else {
            None
        }
    })
}

// Query to get the pesticides application details
#[query]
pub fn get_pesticides_application(crop_id: u64) -> Option<Vec<Application>> {
    CROPS.with(|crops| {
        crops.borrow().get(&crop_id).map(|crop| crop.pesticides_application.clone())
    })
}


// Yiel Prediction model to predict yield based on weather
#[query]
pub fn predict_yield(crop_id: u64) -> Option<f64> {
    CROPS.with(|crops| {
        crops.borrow().get(&crop_id).map(|crop| {
            // Base yield (expected yield)
            let base_yield = crop.expected_yield;

            // Weather factor (e.g., rainfall impact)
            let weather_factor = if crop.weather_conditions.rainfall > 100.0 { 
                1.2 // More rainfall increases yield
            } else {
                1.0 }
            ;

            // Soil factor (e.g., pH level impact)
            let soil_factor = if crop.ph_level >= 6.0 && crop.ph_level <= 7.0 {
                1.1 // Optimal pH range
            } else {
                0.9 // Suboptimal pH range
            };

            // Fertilizer factor (e.g., total fertilizer used)
            let total_fertilizer: f64 = crop.fertilizer_application.iter().map(|app| app.quantity).sum();
            let fertilizer_factor = if total_fertilizer > 0.0 {
                1.0 + (total_fertilizer * 0.01) // Fertilizer increases yield
            } else {
                1.0
            };

            // Pesticide factor (e.g., total pesticide used)
            let total_pesticide: f64 = crop.pesticides_application.iter().map(|app| app.quantity).sum();
            let pesticide_factor = if total_pesticide > 0.0 {
                1.0 - (total_pesticide * 0.005) // Pesticide reduces yield slightly
            } else {
                1.0
            };

            // Final predicted yield
            base_yield * weather_factor * soil_factor * fertilizer_factor * pesticide_factor
        })
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
