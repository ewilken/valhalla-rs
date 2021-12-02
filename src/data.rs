// Routing inputs with respect to: https://valhalla.readthedocs.io/en/latest/api/turn-by-turn/api-reference/
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct RequestOptions {
    pub locations: Vec<Location>,
    pub costing: Option<CostingModels>,
    pub costing_options: Option<CostingOptions>,
    pub units: Option<Units>,
    pub language: Option<String>,
    #[serde(default)]
    pub directions_type: DirectionsType,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum DirectionsType {
    None,
    Maneuvers,
    Instructions,
}

impl Default for DirectionsType {
    fn default() -> Self { DirectionsType::Instructions }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Units {
    Miles,
    Kilometers,
}

#[derive(Serialize, Deserialize, Default, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum LocationType {
    Break,
    Through,
    Via,
    BreakThrough,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PreferredSide {
    Same,
    Opposite,
    Either,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CostingModels {
    Auto,
    Bicycle,
    Bus,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CostingOptions {
    pub auto: Option<AutomobileOptions>,
    pub bicycle: Option<BicycleOptions>,
    pub bus: Option<AutomobileOptions>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
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

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VehicleOptions {
    pub height: Option<f32>,
    pub width: Option<f32>,
    pub exclude_unpaved: Option<u8>,
    pub exclude_cash_only_tolls: Option<bool>,
    pub include_hov2: Option<bool>,
    pub include_hov3: Option<bool>,
    pub include_hot: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BicyleType {
    Hybrid,
    Road,
    City,
    Cross,
    Mountain,
}

impl Default for BicyleType {
    fn default() -> Self { BicyleType::Hybrid }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ManeuverType {
    KNone = 0,
    KStart = 1,
    KStartRight = 2,
    KStartLeft = 3,
    KDestination = 4,
    KDestinationRight = 5,
    KDestinationLeft = 6,
    KBecomes = 7,
    KContinue = 8,
    KSlightRight = 9,
    KRight = 10,
    KSharpRight = 11,
    KUturnRight = 12,
    KUturnLeft = 13,
    KSharpLeft = 14,
    KLeft = 15,
    KSlightLeft = 16,
    KRampStraight = 17,
    KRampRight = 18,
    KRampLeft = 19,
    KExitRight = 20,
    KExitLeft = 21,
    KStayStraight = 22,
    KStayRight = 23,
    KStayLeft = 24,
    KMerge = 25,
    KRoundaboutEnter = 26,
    KRoundaboutExit = 27,
    KFerryEnter = 28,
    KFerryExit = 29,
    KTransit = 30,
    KTransitTransfer = 31,
    KTransitRemainOn = 32,
    KTransitConnectionStart = 33,
    KTransitConnectionTransfer = 34,
    KTransitConnectionDestination = 35,
    KPostTransitConnectionDestination = 36,
    KMergeRight = 37,
    KMergeLeft = 38,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TravelMode {
    Drive,
    Pedestrian,
    Bicycle,
    Transit,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TravelType {
    Car,
    Foot,
    Road,
    Tram,
    Metro,
    Rail,
    Bus,
    Ferry,
    Cable,
    Gondola,
    Funicular,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Maneuver {
    pub r#type: ManeuverType,
    pub instruction: String,
    pub verbal_transition_alert_instruction: Option<String>,
    pub verbal_pre_transition_instruction: Option<String>,
    pub verbal_post_transition_instruction: Option<String>,
    pub street_names: Option<Vec<String>>,
    pub time: f64,
    pub length: f64,
    pub begin_shape_index: u32,
    pub end_shape_index: u32,
    pub travel_mode: TravelMode,
    pub travel_type: TravelType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Leg {
    pub maneuvers: Vec<Maneuver>,
    pub summary: Summary,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Summary {
    pub min_lat: f64,
    pub min_lon: f64,
    pub max_lat: f64,
    pub max_lon: f64,
    pub time: f64,
    pub length: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trip {
    pub locations: Vec<Location>,
    pub legs: Vec<Leg>,
    pub summary: Summary,
    pub status_message: String,
    pub status: u32,
    pub units: Units,
    pub language: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoutingOutput {
    pub trip: Trip,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MatrixInput {
    pub sources: Vec<Location>,
    pub targets: Vec<Location>,
    pub costing: Option<CostingModels>,
    pub id: Option<String>,
    pub units: Option<Units>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MatrixDistance {
    pub distance: f64,
    pub time: f64,
    pub to_index: usize,
    pub from_index: usize,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MatrixOutput {
    pub sources: Vec<Vec<Location>>,
    pub targets: Vec<Vec<Location>>,
    pub sources_to_targets: Vec<Vec<MatrixDistance>>,
    pub units: Option<Units>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contour {
    pub time: f64,
    pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IsochroneInput {
    pub locations: Vec<Location>,
    pub costing: Option<CostingModels>,
    pub contours: Vec<Contour>,
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Geometry {
    pub coordinates: Vec<[f64; 2]>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feature {
    pub geometry: Geometry,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IsochroneOutput {
    pub features: Vec<Feature>,
}
