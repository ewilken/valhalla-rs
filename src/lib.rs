use anyhow::Result;
use autocxx::include_cpp;
use cxx::UniquePtr;

mod config;
pub mod proto;
pub mod route_inputs;

pub use config::Config;
use prost::Message;
use proto::{options::Units, Api, Costing, CostingOptions, LatLng, Location, Options};
pub use route_inputs::RoutingOptions;

include_cpp! {
    #include "valhalla.h"

    safety!(unsafe)

    generate!("ValhallaClient")
    generate!("new_valhalla_client")
}

pub struct Actor {
    inner: UniquePtr<ffi::ValhallaClient>,
}

impl Actor {
    pub fn new(config: &str) -> Self {
        cxx::let_cxx_string!(config_cxx_string = config);
        let inner = ffi::new_valhalla_client(&config_cxx_string);

        Self { inner }
    }

    // Calculates a route.
    pub fn route_proto(&self, request: &Api) -> Result<String> {
        let request = request.encode_to_vec();

        let actor = self.inner.as_mut().unwrap();

        let request_pointer = request.as_ptr();
        let request_size = request.len() as std::os::raw::c_ulong;

        let response = unsafe { actor.route_proto(request_pointer, autocxx::c_ulong(request_size)) };

        Ok(response) // TODO - don't allocate here
    }

    // Calculates a route.
    pub fn route(&self, request: &RoutingOptions) -> Result<String> {
        let request_string = serde_json::to_string(request)?;
        cxx::let_cxx_string!(request_cxx_string = request_string);

        Ok(self.inner.route(&request_cxx_string))
    }

    pub fn locate(&self, request: &str) -> String {
        cxx::let_cxx_string!(rq_str = request);
        self.inner.locate(&rq_str)
    }

    pub fn optimized_route(&self, request: &str) -> String {
        cxx::let_cxx_string!(rq_str = request);
        self.inner.optimized_route(&rq_str)
    }

    pub fn matrix(&self, request: &str) -> String {
        cxx::let_cxx_string!(rq_str = request);
        self.inner.matrix(&rq_str)
    }

    pub fn isochrone(&self, request: &str) -> String {
        cxx::let_cxx_string!(rq_str = request);
        self.inner.isochrone(&rq_str)
    }

    pub fn trace_route(&self, request: &str) -> String {
        cxx::let_cxx_string!(rq_str = request);
        self.inner.trace_route(&rq_str)
    }

    pub fn trace_attributes(&self, request: &str) -> String {
        cxx::let_cxx_string!(rq_str = request);
        self.inner.trace_attributes(&rq_str)
    }

    pub fn height(&self, request: &str) -> String {
        cxx::let_cxx_string!(rq_str = request);
        self.inner.height(&rq_str)
    }

    pub fn transit_available(&self, request: &str) -> String {
        cxx::let_cxx_string!(rq_str = request);
        self.inner.transit_available(&rq_str)
    }

    pub fn expansion(&self, request: &str) -> String {
        cxx::let_cxx_string!(rq_str = request);
        self.inner.expansion(&rq_str)
    }

    pub fn centroid(&self, request: &str) -> String {
        cxx::let_cxx_string!(rq_str = request);
        self.inner.centroid(&rq_str)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        proto::{
            options::Units,
            Api,
            Costing,
            CostingOptions as ProtoCostingOptions,
            LatLng,
            Location as ProtoLocation,
            Options,
        },
        route_inputs::{CostingModels, CostingOptions, Location, RoutingOptions},
        Actor,
    };

    #[test]
    fn test_route() {
        let actor = Actor::new("valhalla/valhalla.json");

        let request = RoutingOptions {
            locations: vec![
                Location {
                    lat: Some(52.499078),
                    lon: Some(13.418230),
                    name: Some("Kottbusser Tor".into()),
                    ..Default::default()
                },
                Location {
                    lat: Some(52.487331),
                    lon: Some(13.425042),
                    name: Some("Hermannplatz".into()),
                    ..Default::default()
                },
            ],
            costing: Some(CostingModels::AUTO.as_string()),
            units: Some("km".to_string()),
            id: Some("kotti_to_hermannplatz".into()),
            ..Default::default()
        };

        let response = actor.route(&request).unwrap();

        println!("{}", response);
    }

    #[test]
    fn test_route_proto() {
        let request = Api {
            options: Some(Options {
                locations: vec![
                    ProtoLocation {
                        ll: Some(LatLng {
                            lat: Some(52.499078),
                            lng: Some(13.418230),
                        }),
                        name: Some("Kottbusser Tor".into()),
                        ..Default::default()
                    },
                    ProtoLocation {
                        ll: Some(LatLng {
                            lat: Some(52.487331),
                            lng: Some(13.425042),
                        }),
                        name: Some("Hermannplatz".into()),
                        ..Default::default()
                    },
                ],
                costing: Some(0),
                costing_options: vec![ProtoCostingOptions {
                    country_crossing_penalty: Some(2000.0),
                    ..Default::default()
                }],
                units: Some(Units::Kilometers as i32),
                id: Some("kotti_to_hermannplatz".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut actor = Actor::new("valhalla/valhalla.json");
        let response = actor.route_proto(&request).unwrap();

        println!("{}", response);
    }
}
