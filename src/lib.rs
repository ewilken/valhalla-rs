use anyhow::{Error, Result};
use autocxx::include_cpp;
use cxx::UniquePtr;

mod config;
pub mod data;
pub mod proto;

pub use config::Config;
use data::{HeightRequest, HeightResponse, IsochroneInput, IsochroneOutput, MatrixInput, MatrixOutput};
pub use data::{RequestOptions, RoutingOutput};
//use proto::Api;

include_cpp! {
    #include "valhalla.h"

    safety!(unsafe)

    generate!("ValhallaClient")
    generate!("new_valhalla_client")
}

pub struct ActorNative {
    inner: UniquePtr<ffi::ValhallaClient>,
}

impl ActorNative {
    pub fn new(config: &str) -> Self {
        cxx::let_cxx_string!(config_cxx_string = config);
        let inner = ffi::new_valhalla_client(&config_cxx_string);

        Self { inner }
    }

    // Calculates a route.
    pub fn route(&self, request: &str) -> String {
        cxx::let_cxx_string!(rq_str = request);
        self.inner.route(&rq_str)
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

pub struct Actor {
    inner: ActorNative,
}

impl Actor {
    pub fn new(config: &str) -> Self {
        Self {
            inner: ActorNative::new(config),
        }
    }

    // Calculates a route.
    pub fn route(&self, request: &RequestOptions) -> Result<RoutingOutput> {
        let req = serde_json::to_string(request)?;
        let res = self.inner.route(&req);

        serde_json::from_str(&res).map_err(Error::from)
    }

    pub fn locate(&self, request: &RequestOptions) -> Result<RoutingOutput> {
        let req = serde_json::to_string(request)?;
        let res = self.inner.route(&req);

        serde_json::from_str(&res).map_err(Error::from)
    }

    pub fn optimized_route(&self, request: &RequestOptions) -> Result<RoutingOutput> {
        let req = serde_json::to_string(request)?;
        let res = self.inner.optimized_route(&req);

        serde_json::from_str(&res).map_err(Error::from)
    }

    pub fn matrix(&self, request: &MatrixInput) -> Result<MatrixOutput> {
        let req = serde_json::to_string(request)?;
        let res = self.inner.matrix(&req);

        serde_json::from_str(&res).map_err(Error::from)
    }

    pub fn isochrone(&self, request: &IsochroneInput) -> Result<IsochroneOutput> {
        let req = serde_json::to_string(request)?;
        let res = self.inner.isochrone(&req);

        serde_json::from_str(&res).map_err(Error::from)
    }

    pub fn trace_route(&self, request: &str) -> String { self.inner.trace_route(&request) }

    pub fn trace_attributes(&self, request: &str) -> String { self.inner.trace_attributes(&request) }

    /// The height method of the [elevation service][elevation-service].
    ///
    /// [elevation-service]: https://valhalla.readthedocs.io/en/latest/api/elevation/api-reference/#elevation-service-api-reference
    pub fn height(&self, request: &HeightRequest) -> Result<HeightResponse> {
        let req = serde_json::to_string(request)?;
        let res = self.inner.height(&req);

        serde_json::from_str(&res).map_err(Error::from)
    }

    pub fn transit_available(&self, request: &str) -> String { self.inner.transit_available(&request) }
}

#[cfg(test)]
mod tests {
    use crate::{
        data::{Contour, CostingModels, HeightRequest, IsochroneInput, Location, MatrixInput, RequestOptions, Units},
        Actor,
    };

    #[test]
    fn test_route() {
        let actor = Actor::new("valhalla.json");

        let request = RequestOptions {
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

        let r = actor.route(&request).unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_locate() {
        let actor = Actor::new("valhalla.json");

        let request = RequestOptions {
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

        let r = actor.locate(&request).unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_optimazed_route() {
        let actor = Actor::new("valhalla.json");

        let request = RequestOptions {
            locations: vec![
                Location {
                    lat: Some(52.499078),
                    lon: Some(13.418230),
                    name: Some("Kottbusser Tor".into()),
                    ..Default::default()
                },
                Location {
                    lat: Some(52.4929306),
                    lon: Some(13.4211985),
                    name: Some("Bürknerstraße".into()),
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

        let r = actor.optimized_route(&request).unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_matrix() {
        let actor = Actor::new("valhalla.json");

        let request = MatrixInput {
            sources: vec![Location {
                lat: Some(52.499078),
                lon: Some(13.418230),
                ..Default::default()
            }],
            targets: vec![
                Location {
                    lat: Some(52.4929306),
                    lon: Some(13.4211985),
                    ..Default::default()
                },
                Location {
                    lat: Some(52.487331),
                    lon: Some(13.425042),
                    ..Default::default()
                },
            ],
            costing: Some(CostingModels::Auto),
            units: Some(Units::Miles),
            ..Default::default()
        };

        let r = actor.matrix(&request).unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_isochrone() {
        let actor = Actor::new("valhalla.json");

        let request = IsochroneInput {
            locations: vec![Location {
                lat: Some(52.499078),
                lon: Some(13.418230),
                ..Default::default()
            }],
            costing: Some(CostingModels::Auto),
            contours: vec![Contour {
                time: 15.0,
                color: None,
            }],
            id: None,
        };

        let r = actor.isochrone(&request).unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_height() {
        let actor = Actor::new("valhalla.json");
        let mut req = HeightRequest {
            range: Some(true),
            shape: vec![
                Location {
                    lat: Some(52.499078),
                    lon: Some(13.41823),
                    ..Default::default()
                },
                Location {
                    lat: Some(52.4929306),
                    lon: Some(13.4211985),
                    ..Default::default()
                },
                Location {
                    lat: Some(52.487331),
                    lon: Some(13.425042),
                    ..Default::default()
                },
            ],
            id: None,
        };

        let res = actor.height(&req).unwrap();
        assert!(res.range_height.is_some());
        assert!(res.height.is_none());

        req.range = None;
        let res = actor.height(&req).unwrap();
        assert!(res.range_height.is_none());
        assert!(res.height.is_some());
    }
}
