// Routing inputs with respect to: https://valhalla.readthedocs.io/en/latest/api/turn-by-turn/api-reference/

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct RoutingOptions {
    pub locations: Vec<Location>,
    pub costing: Option<CostingModels>,
    pub costing_options: Option<CostingOptions>,
    pub units: Option<Units>,
    pub language: Option<String>,
    pub directions_type: Option<DirectionsType>,
    // DEPRECATED: Should use directions_type
    pub narrative: Option<String>,
    //pub exclude_locations: Option<String>,
    //pub exclude_polygons: Option<String>,
    // ISO 8601 format <YYYY-MM-DDThh:mm>
    pub date_time: Option<String>,
    pub out_format: Option<String>,
    pub id: Option<String>,
    pub linear_references: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DirectionsType {
    NONE,
    MANEUVERS,
    // this is the default if not specified
    INSTRUCTIONS,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Units {
    #[serde(rename = "mi")]
    MILES,
    #[serde(rename = "km")]
    KILOMETERS,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Location {
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    // if no type is provided, the type is assumed to be a break
    pub r#type: Option<LocationType>,
    pub heading: Option<f32>,
    pub heading_tolerance: Option<f32>,
    pub street: Option<String>,
    pub way_id: Option<String>,
    pub minimum_reachability: Option<u32>,
    pub radius: Option<f32>,
    pub rank_candidates: Option<bool>,
    pub preferred_side: Option<PreferredSide>,
    pub display_lat: Option<f64>,
    pub display_lon: Option<f64>,
    pub search_cutoff: Option<String>,
    pub node_snap_tolerance: Option<f32>,
    pub street_side_tolerance: Option<f32>,
    pub street_side_max_distance: Option<f32>,
    // TO-DO: check again
    pub search_filter: Option<String>,
    pub name: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub phone: Option<String>,
    pub url: Option<String>,
    pub side_of_street: Option<String>,
    // ISO 8601 format <YYYY-MM-DDThh:mm>
    pub date_time: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LocationType {
    BREAK,
    THROUGH,
    VIA,
    BREAK_THROUGH,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PreferredSide {
    #[serde(rename = "same")]
    SAME,
    #[serde(rename = "opposite")]
    OPPOSITE,
    #[serde(rename = "either")]
    EITHER,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CostingModels {
    AUTO,
    BICYCLE,
    BUS,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct CostingOptions {
    pub auto: Option<AutomobileOptions>,
    pub bicycle: Option<BicycleOptions>,
    pub bus: Option<AutomobileOptions>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct AutomobileOptions {
    pub maneuver_penalty: Option<u32>,
    pub gate_cost: Option<u32>,
    pub gate_penalty: Option<u32>,
    pub private_access_penalty: Option<u32>,
    pub toll_booth_cost: Option<u32>,
    pub toll_booth_penalty: Option<u32>,
    //  this is a range of values between 0 and 1, default value is 0.5 - for the following 4
    pub ferry_cost: Option<f32>,
    pub use_highways: Option<f32>,
    pub use_tolls: Option<f32>,
    pub use_living_streets: Option<f32>,
    pub use_tracks: Option<f32>,
    pub service_penalty: Option<u8>,
    pub service_factor: Option<f32>,
    pub country_crossing_cost: Option<f32>,
    pub country_crossing_penalty: Option<f32>,
    pub shortest: Option<bool>,
    pub top_speed: Option<u8>,
    pub ignore_closures: Option<bool>,
    pub closure_factor: Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct VehicleOptions {
    pub height: Option<f32>,
    pub width: Option<f32>,
    pub exclude_unpaved: Option<u8>,
    pub exclude_cash_only_tolls: Option<bool>,
    pub include_hov2: Option<bool>,
    pub include_hov3: Option<bool>,
    pub include_hot: Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct BicycleOptions {
    pub bicycle_type: Option<BicyleType>,
    pub cycling_speed: Option<f32>,
    pub use_roads: Option<f32>,
    pub use_hills: Option<f32>,
    pub use_ferry: Option<f32>,
    pub use_living_streets: Option<f32>,
    pub avoid_bad_surfaces: Option<f32>,
    pub bss_return_cost: Option<u16>,
    pub bss_return_penalty: Option<f32>,
    pub shortest: Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BicyleType {
    // is default type
    HYBRID,
    ROAD,
    CITY,
    CROSS,
    MOUNTAIN,
}
