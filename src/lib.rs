use anyhow::{Result, Error};
use autocxx::include_cpp;
use cxx::UniquePtr;

mod config;
pub mod proto;
pub mod route;

pub use config::Config;
pub use route::{RoutingOptions, RoutingOutput};
//use proto::Api;

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
    pub fn route(&self, request: &RoutingOptions) -> Result<RoutingOutput> {
        let request_string = serde_json::to_string(request)?;
        cxx::let_cxx_string!(request_cxx_string = request_string);

        let rs: String = self.inner.route(&request_cxx_string);
        println!("route: {}", &rs);
        serde_json::from_str::<RoutingOutput>(&rs).map_err(Error::from)
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
        route::{CostingModels, Location, RoutingOptions, Units},
        Actor,
    };

    #[test]
    fn test_route() {
        let actor = Actor::new("valhalla.json");

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
            costing: Some(CostingModels::Auto),
            units: Some(Units::Kilometers),
            id: Some("kotti_to_hermannplatz".into()),
            ..Default::default()
        };

        let routing = actor.route(&request).unwrap();

        println!("{:?}", routing);
        assert!(false);
    }
}
