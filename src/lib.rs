use std::{fmt, error};

use anyhow::{Error, Result};
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

#[derive(Debug)]
pub struct NotDefined;

impl fmt::Display for NotDefined {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "valhalla not defined")
    }
}

impl error::Error for NotDefined {}

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

        self.inner.as_mut()
            .ok_or(Error::new(NotDefined))
            .map(|a| a.route(&request_cxx_string))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        route_inputs::{CostingModels, Location, RoutingOptions},
        Actor,
    };

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
