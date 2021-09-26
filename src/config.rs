use serde::{Deserialize, Serialize};

// taken from valhalla/scripts/valhalla_build_config

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub mjolnir: MjolnirConfig,
    pub loki: LokiConfig,
    pub thor: ThorConfig,
    pub odin: OdinConfig,
    pub meili: MeiliConfig,
    pub httpd: HttpdConfig,
    pub statsd: StatsdConfig,
    pub additional_data: AdditionalDataConfig,
    pub service_limits: ServiceLimitsConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MjolnirConfig {
    pub max_cache_size: i32,
    pub id_table_size: i32,
    pub use_lru_mem_cache: bool,
    pub lru_mem_cache_hard_control: bool,
    pub use_simple_mem_cache: bool,
    pub user_agent: Option<String>,
    pub tile_url: Option<String>,
    pub tile_url_gz: Option<bool>,
    pub concurrency: Option<i32>,
    pub tile_dir: String,
    pub tile_extract: String,
    pub traffic_extract: String,
    pub incident_dir: Option<String>,
    pub incident_log: Option<String>,
    pub shortcut_caching: Option<bool>,
    pub admin: String,
    pub timezone: String,
    pub transit_dir: String,
    pub transit_bounding_box: Option<String>,
    pub hierarchy: bool,
    pub shortcuts: bool,
    pub include_driveways: bool,
    pub include_bicycle: bool,
    pub include_pedestrian: bool,
    pub include_driving: bool,
    pub import_bike_share_stations: bool,
    pub global_synchronized_cache: bool,
    pub max_concurrent_reader_users: i32,
    pub reclassify_links: bool,
    pub default_speeds_config: Option<String>,
    pub data_processing: MjolnirDataProcessingConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub r#type: String,
    pub color: bool,
    pub file_name: String,
    pub long_request: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MjolnirDataProcessingConfig {
    pub infer_internal_intersections: bool,
    pub infer_turn_channels: bool,
    pub apply_country_overrides: bool,
    pub use_admin_db: bool,
    pub use_direction_on_ways: bool,
    pub allow_alt_name: bool,
    pub use_urban_tag: bool,
    pub use_rest_area: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LokiConfig {
    pub actions: Vec<String>,
    pub use_connectivity: bool,
    pub service_defaults: LokiServiceDefaultsConfig,
    pub logging: LoggingConfig,
    pub service: ServiceConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LokiServiceDefaultsConfig {
    pub radius: i32,
    pub minimum_reachability: i32,
    pub search_cutoff: i32,
    pub node_snap_tolerance: i32,
    pub street_side_tolerance: i32,
    pub street_side_max_distance: i32,
    pub heading_tolerance: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub proxy: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThorConfig {
    pub logging: LoggingConfig,
    pub source_to_target_algorithm: String,
    pub service: ServiceConfig,
    pub max_reserved_labels_count: i32,
    pub extended_search: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OdinConfig {
    pub logging: LoggingConfig,
    pub service: ServiceConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeiliConfig {
    pub mode: String,
    pub customizable: Vec<String>,
    pub verbose: bool,
    pub default: MeiliModeConfig,
    pub auto: MeiliModeConfig,
    pub pedestrian: MeiliModeConfig,
    pub bicycle: MeiliModeConfig,
    pub multimodal: MeiliModeConfig,
    pub logging: LoggingConfig,
    pub service: ServiceConfig,
    pub grid: GridConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeiliModeConfig {
    pub sigma_z: Option<f32>,
    pub gps_accuracy: Option<f32>,
    pub beta: Option<i32>,
    pub max_route_distance_factor: Option<i32>,
    pub max_route_time_factor: Option<i32>,
    pub max_search_radius: Option<i32>,
    pub breakage_distance: Option<i32>,
    pub interpolation_distance: Option<i32>,
    pub search_radius: Option<i32>,
    pub geometry: Option<bool>,
    pub route: Option<bool>,
    pub turn_penalty_factor: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GridConfig {
    pub size: i32,
    pub cache_size: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpdConfig {
    pub service: HttpdServiceConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpdServiceConfig {
    pub listen: String,
    pub loopback: String,
    pub interrupt: String,
    pub drain_seconds: i32,
    pub shutdown_seconds: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsdConfig {
    pub host: Option<String>,
    pub port: u16,
    pub prefix: String,
    pub batch_size: Option<i32>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalDataConfig {
    pub elevation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceLimitsConfig {
    pub max_exclude_locations: i32,
    pub max_reachability: i32,
    pub max_radius: i32,
    pub max_timedep_distance: i32,
    pub max_alternates: i32,
    pub max_exclude_polygons_length: i32,

    pub auto: ServiceLimitConfig,
    pub auto_shorter: ServiceLimitConfig,
    pub bus: ServiceLimitConfig,
    pub hov: ServiceLimitConfig,
    pub taxi: ServiceLimitConfig,
    pub pedestrian: ServiceLimitConfig,
    pub motor_scooter: ServiceLimitConfig,
    pub motorcycle: ServiceLimitConfig,
    pub bicycle: ServiceLimitConfig,
    pub multimodal: ServiceLimitConfig,
    pub transit: ServiceLimitConfig,
    pub truck: ServiceLimitConfig,
    pub skadi: ServiceLimitConfig,
    pub isochrone: ServiceLimitConfig,
    pub trace: ServiceLimitConfig,
    pub bikeshare: ServiceLimitConfig,
    pub centroid: ServiceLimitConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceLimitConfig {
    pub max_distance: Option<f32>,
    pub max_locations: Option<i32>,
    pub max_matrix_distance: Option<f32>,
    pub max_matrix_locations: Option<i32>,
    pub min_transit_walking_distance: Option<i32>,
    pub max_transit_walking_distance: Option<i32>,
    pub max_shape: Option<i32>,
    pub min_resample: Option<f32>,
    pub max_contours: Option<i32>,
    pub max_time_contour: Option<i32>,
    pub max_distance_contour: Option<i32>,
    pub max_gps_accuracy: Option<f32>,
    pub max_search_radius: Option<f32>,
    pub max_best_paths: Option<i32>,
    pub max_best_paths_shape: Option<i32>,
}
