use anyhow::{Error, Result};
use autocxx::include_cpp;
use cxx::UniquePtr;

use prost::Message;

pub mod proto;
use proto::Api;

include_cpp! {
    #include "valhalla.h"

    safety!(unsafe)

    generate!("ValhallaJsonClient")
    generate!("new_valhalla_json_client")

    generate!("ValhallaProtobufClient")
    generate!("new_valhalla_protobuf_client")
}

unsafe impl Send for ffi::ValhallaJsonClient {}
unsafe impl Sync for ffi::ValhallaJsonClient {}

unsafe impl Send for ffi::ValhallaProtobufClient {}
unsafe impl Sync for ffi::ValhallaProtobufClient {}

pub struct JsonActor {
    inner: UniquePtr<ffi::ValhallaJsonClient>,
}

impl JsonActor {
    pub fn new(config: &str) -> Self {
        cxx::let_cxx_string!(config_cxx_string = config);
        let inner = ffi::new_valhalla_json_client(&config_cxx_string);

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

pub struct ProtobufActor {
    inner: UniquePtr<ffi::ValhallaProtobufClient>,
}

impl ProtobufActor {
    pub fn new(config: &str) -> Self {
        cxx::let_cxx_string!(config_cxx_string = config);
        let inner = ffi::new_valhalla_protobuf_client(&config_cxx_string);

        Self { inner }
    }

    pub fn route(&self, request: &Api) -> Result<Api> {
        let mut buf = vec![];
        request.encode(&mut buf)?;

        cxx::let_cxx_string!(api_str = buf);
        let response = self.inner.route(&api_str);

        Message::decode(response.as_bytes()).map_err(Error::from)
    }

    pub fn locate(&self, request: &Api) -> Result<Api> {
        let mut buf = vec![];
        request.encode(&mut buf)?;

        cxx::let_cxx_string!(api_str = buf);
        let response = self.inner.locate(&api_str);

        Message::decode(response.as_bytes()).map_err(Error::from)
    }

    pub fn optimized_route(&self, request: &Api) -> Result<Api> {
        let mut buf = vec![];
        request.encode(&mut buf)?;

        cxx::let_cxx_string!(api_str = buf);
        let response = self.inner.optimized_route(&api_str);

        Message::decode(response.as_bytes()).map_err(Error::from)
    }

    pub fn matrix(&self, request: &Api) -> Result<Api> {
        let mut buf = vec![];
        request.encode(&mut buf)?;

        cxx::let_cxx_string!(api_str = buf);
        let response = self.inner.matrix(&api_str);

        Message::decode(response.as_bytes()).map_err(Error::from)
    }

    pub fn isochrone(&self, request: &Api) -> Result<Api> {
        let mut buf = vec![];
        request.encode(&mut buf)?;

        cxx::let_cxx_string!(api_str = buf);
        let response = self.inner.isochrone(&api_str);

        Message::decode(response.as_bytes()).map_err(Error::from)
    }

    pub fn trace_route(&self, request: &Api) -> Result<Api> {
        let mut buf = vec![];
        request.encode(&mut buf)?;

        cxx::let_cxx_string!(api_str = buf);
        let response = self.inner.trace_route(&api_str);

        Message::decode(response.as_bytes()).map_err(Error::from)
    }

    pub fn trace_attributes(&self, request: &Api) -> Result<Api> {
        let mut buf = vec![];
        request.encode(&mut buf)?;

        cxx::let_cxx_string!(api_str = buf);
        let response = self.inner.trace_attributes(&api_str);

        Message::decode(response.as_bytes()).map_err(Error::from)
    }

    pub fn height(&self, request: &Api) -> Result<Api> {
        let mut buf = vec![];
        request.encode(&mut buf)?;

        cxx::let_cxx_string!(api_str = buf);
        let response = self.inner.height(&api_str);

        Message::decode(response.as_bytes()).map_err(Error::from)
    }

    pub fn transit_available(&self, request: &Api) -> Result<Api> {
        let mut buf = vec![];
        request.encode(&mut buf)?;

        cxx::let_cxx_string!(api_str = buf);
        let response = self.inner.transit_available(&api_str);

        Message::decode(response.as_bytes()).map_err(Error::from)
    }

    pub fn expansion(&self, request: &Api) -> Result<Api> {
        let mut buf = vec![];
        request.encode(&mut buf)?;

        cxx::let_cxx_string!(api_str = buf);
        let response = self.inner.expansion(&api_str);

        Message::decode(response.as_bytes()).map_err(Error::from)
    }

    pub fn centroid(&self, request: &Api) -> Result<Api> {
        let mut buf = vec![];
        request.encode(&mut buf)?;

        cxx::let_cxx_string!(api_str = buf);
        let response = self.inner.centroid(&api_str);

        Message::decode(response.as_bytes()).map_err(Error::from)
    }
}
