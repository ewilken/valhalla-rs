use serde::{Deserialize, Serialize};

// derived from valhalla/scripts/valhalla_build_config

#[derive(Debug, Default, Serialize, Deserialize)]
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

impl Default for MjolnirConfig {
    fn default() -> Self {
        Self {
            max_cache_size: 1000000000,
            id_table_size: 1300000000,
            use_lru_mem_cache: false,
            lru_mem_cache_hard_control: false,
            use_simple_mem_cache: false,
            user_agent: None,
            tile_url: None,
            tile_url_gz: None,
            concurrency: None,
            tile_dir: "/data/valhalla".to_string(),
            tile_extract: "/data/valhalla/tiles.tar".to_string(),
            traffic_extract: "/data/valhalla/traffic.tar".to_string(),
            incident_dir: None,
            incident_log: None,
            shortcut_caching: None,
            admin: "/data/valhalla/admin.sqlite".to_string(),
            timezone: "/data/valhalla/tz_world.sqlite".to_string(),
            transit_dir: "/data/valhalla/transit".to_string(),
            transit_bounding_box: None,
            hierarchy: true,
            shortcuts: true,
            include_driveways: true,
            include_bicycle: true,
            include_pedestrian: true,
            include_driving: true,
            import_bike_share_stations: false,
            global_synchronized_cache: false,
            max_concurrent_reader_users: 1,
            reclassify_links: true,
            default_speeds_config: None,
            data_processing: MjolnirDataProcessingConfig::default(),
            logging: LoggingConfig {
                r#type: "std_out".to_string(),
                color: true,
                file_name: "path_to_some_file.log".to_string(),
                long_request: None,
            },
        }
    }
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

impl Default for MjolnirDataProcessingConfig {
    fn default() -> Self {
        Self {
            infer_internal_intersections: true,
            infer_turn_channels: true,
            apply_country_overrides: true,
            use_admin_db: true,
            use_direction_on_ways: false,
            allow_alt_name: false,
            use_urban_tag: false,
            use_rest_area: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LokiConfig {
    pub actions: Vec<String>,
    pub use_connectivity: bool,
    pub service_defaults: LokiServiceDefaultsConfig,
    pub logging: LoggingConfig,
    pub service: ServiceConfig,
}

impl Default for LokiConfig {
    fn default() -> Self {
        Self {
            actions: vec![
                "locate".to_string(),
                "route".to_string(),
                "height".to_string(),
                "sources_to_targets".to_string(),
                "optimized_route".to_string(),
                "isochrone".to_string(),
                "trace_route".to_string(),
                "trace_attributes".to_string(),
                "transit_available".to_string(),
                "expansion".to_string(),
                "centroid".to_string(),
                "status".to_string(),
            ],
            use_connectivity: true,
            service_defaults: LokiServiceDefaultsConfig::default(),
            logging: LoggingConfig {
                r#type: "std_out".to_string(),
                color: true,
                file_name: "path_to_some_file.log".to_string(),
                long_request: Some(100.0),
            },
            service: ServiceConfig {
                proxy: "ipc:///tmp/loki".to_string(),
            },
        }
    }
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

impl Default for LokiServiceDefaultsConfig {
    fn default() -> Self {
        Self {
            radius: 0,
            minimum_reachability: 50,
            search_cutoff: 35000,
            node_snap_tolerance: 5,
            street_side_tolerance: 5,
            street_side_max_distance: 1000,
            heading_tolerance: 60,
        }
    }
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

impl Default for ThorConfig {
    fn default() -> Self {
        Self {
            logging: LoggingConfig {
                r#type: "std_out".to_string(),
                color: true,
                file_name: "path_to_some_file.log".to_string(),
                long_request: Some(110.0),
            },
            source_to_target_algorithm: "select_optimal".to_string(),
            service: ServiceConfig {
                proxy: "ipc:///tmp/thor".to_string(),
            },
            max_reserved_labels_count: 1000000,
            extended_search: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OdinConfig {
    pub logging: LoggingConfig,
    pub service: ServiceConfig,
}

impl Default for OdinConfig {
    fn default() -> Self {
        Self {
            logging: LoggingConfig {
                r#type: "std_out".to_string(),
                color: true,
                file_name: "path_to_some_file.log".to_string(),
                long_request: None,
            },
            service: ServiceConfig {
                proxy: "ipc:///tmp/odin".to_string(),
            },
        }
    }
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

impl Default for MeiliConfig {
    fn default() -> Self {
        Self {
            mode: "auto".to_string(),
            customizable: vec![
                "mode".to_string(),
                "search_radius".to_string(),
                "turn_penalty_factor".to_string(),
                "gps_accuracy".to_string(),
                "interpolation_distance".to_string(),
                "sigma_z".to_string(),
                "beta".to_string(),
                "max_route_distance_factor".to_string(),
                "max_route_time_factor".to_string(),
            ],
            verbose: false,
            default: MeiliModeConfig {
                sigma_z: Some(4.07),
                gps_accuracy: Some(5.0),
                beta: Some(3),
                max_route_distance_factor: Some(5),
                max_route_time_factor: Some(5),
                max_search_radius: Some(100),
                breakage_distance: Some(2000),
                interpolation_distance: Some(10),
                search_radius: Some(50),
                geometry: Some(false),
                route: Some(true),
                turn_penalty_factor: Some(0),
            },
            auto: MeiliModeConfig {
                turn_penalty_factor: Some(200),
                search_radius: Some(50),
                ..Default::default()
            },
            pedestrian: MeiliModeConfig {
                turn_penalty_factor: Some(100),
                search_radius: Some(50),
                ..Default::default()
            },
            bicycle: MeiliModeConfig {
                turn_penalty_factor: Some(140),
                ..Default::default()
            },
            multimodal: MeiliModeConfig {
                turn_penalty_factor: Some(70),
                ..Default::default()
            },
            logging: LoggingConfig {
                r#type: "std_out".to_string(),
                color: true,
                file_name: "path_to_some_file.log".to_string(),
                long_request: None,
            },
            service: ServiceConfig {
                proxy: "ipc:///tmp/meili".to_string(),
            },
            grid: GridConfig::default(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            size: 500,
            cache_size: 100240,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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

impl Default for HttpdServiceConfig {
    fn default() -> Self {
        Self {
            listen: "tcp://*:8002".to_string(),
            loopback: "ipc:///tmp/loopback".to_string(),
            interrupt: "ipc:///tmp/interrupt".to_string(),
            drain_seconds: 28,
            shutdown_seconds: 1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsdConfig {
    pub host: Option<String>,
    pub port: u16,
    pub prefix: String,
    pub batch_size: Option<i32>,
    pub tags: Option<Vec<String>>,
}

impl Default for StatsdConfig {
    fn default() -> Self {
        Self {
            host: None,
            port: 8125,
            prefix: "valhalla".to_string(),
            batch_size: None,
            tags: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalDataConfig {
    pub elevation: String,
}

impl Default for AdditionalDataConfig {
    fn default() -> Self {
        Self {
            elevation: "/data/valhalla/elevation/".to_string(),
        }
    }
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

impl Default for ServiceLimitsConfig {
    fn default() -> Self {
        Self {
            max_exclude_locations: 50,
            max_reachability: 100,
            max_radius: 200,
            max_timedep_distance: 500000,
            max_alternates: 2,
            max_exclude_polygons_length: 10000,

            auto: ServiceLimitConfig {
                max_distance: Some(5000000.0),
                max_locations: Some(20),
                max_matrix_distance: Some(400000.0),
                max_matrix_locations: Some(50),
                ..Default::default()
            },
            auto_shorter: ServiceLimitConfig {
                max_distance: Some(5000000.0),
                max_locations: Some(20),
                max_matrix_distance: Some(400000.0),
                max_matrix_locations: Some(50),
                ..Default::default()
            },
            bus: ServiceLimitConfig {
                max_distance: Some(5000000.0),
                max_locations: Some(50),
                max_matrix_distance: Some(400000.0),
                max_matrix_locations: Some(50),
                ..Default::default()
            },
            hov: ServiceLimitConfig {
                max_distance: Some(5000000.0),
                max_locations: Some(20),
                max_matrix_distance: Some(400000.0),
                max_matrix_locations: Some(50),
                ..Default::default()
            },
            taxi: ServiceLimitConfig {
                max_distance: Some(5000000.0),
                max_locations: Some(20),
                max_matrix_distance: Some(400000.0),
                max_matrix_locations: Some(50),
                ..Default::default()
            },
            pedestrian: ServiceLimitConfig {
                max_distance: Some(250000.0),
                max_locations: Some(50),
                max_matrix_distance: Some(200000.0),
                max_matrix_locations: Some(50),
                min_transit_walking_distance: Some(1),
                max_transit_walking_distance: Some(10000),
                ..Default::default()
            },
            motor_scooter: ServiceLimitConfig {
                max_distance: Some(500000.0),
                max_locations: Some(50),
                max_matrix_distance: Some(200000.0),
                max_matrix_locations: Some(50),
                ..Default::default()
            },
            motorcycle: ServiceLimitConfig {
                max_distance: Some(500000.0),
                max_locations: Some(50),
                max_matrix_distance: Some(200000.0),
                max_matrix_locations: Some(50),
                ..Default::default()
            },
            bicycle: ServiceLimitConfig {
                max_distance: Some(500000.0),
                max_locations: Some(50),
                max_matrix_distance: Some(200000.0),
                max_matrix_locations: Some(50),
                ..Default::default()
            },
            multimodal: ServiceLimitConfig {
                max_distance: Some(500000.0),
                max_locations: Some(50),
                max_matrix_distance: Some(0.0),
                max_matrix_locations: Some(0),
                ..Default::default()
            },
            transit: ServiceLimitConfig {
                max_distance: Some(500000.0),
                max_locations: Some(50),
                max_matrix_distance: Some(200000.0),
                max_matrix_locations: Some(50),
                ..Default::default()
            },
            truck: ServiceLimitConfig {
                max_distance: Some(5000000.0),
                max_locations: Some(20),
                max_matrix_distance: Some(400000.0),
                max_matrix_locations: Some(50),
                ..Default::default()
            },
            skadi: ServiceLimitConfig {
                max_shape: Some(750000),
                min_resample: Some(10.0),
                ..Default::default()
            },
            isochrone: ServiceLimitConfig {
                max_contours: Some(4),
                max_time_contour: Some(120),
                max_distance: Some(25000.0),
                max_locations: Some(1),
                max_distance_contour: Some(200),
                ..Default::default()
            },
            trace: ServiceLimitConfig {
                max_distance: Some(200000.0),
                max_gps_accuracy: Some(100.0),
                max_search_radius: Some(100.0),
                max_shape: Some(16000),
                max_best_paths: Some(4),
                max_best_paths_shape: Some(100),
                ..Default::default()
            },
            bikeshare: ServiceLimitConfig {
                max_distance: Some(500000.0),
                max_locations: Some(50),
                max_matrix_distance: Some(200000.0),
                max_matrix_locations: Some(50),
                ..Default::default()
            },
            centroid: ServiceLimitConfig {
                max_distance: Some(200000.0),
                max_locations: Some(5),
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
