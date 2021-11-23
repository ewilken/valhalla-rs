use anyhow::Result;
use autocxx::include_cpp;
use cxx::UniquePtr;

mod config;
pub mod proto;
pub mod route_inputs;

pub use config::Config;
pub use route_inputs::RoutingOptions;
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
    pub fn route(&mut self, request: &RoutingOptions) -> Result<String> {
        let request_string = serde_json::to_string(request)?;
        cxx::let_cxx_string!(request_cxx_string = request_string);

        let actor = self.inner.as_mut().unwrap();

        let response = actor.route(&request_cxx_string);

        Ok(response) // TODO - don't allocate here
    }
}

#[cfg(test)]
mod tests {
    use crate::route_inputs::{CostingModels, CostingOptions, Location, RoutingOptions};
    use crate::Actor;

    #[test]
    fn test_route() {
        let mut actor = Actor::new("valhalla/valhalla.json");

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
}
