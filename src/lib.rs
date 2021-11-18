use anyhow::Result;
use autocxx::include_cpp;
use cxx::UniquePtr;

mod config;
pub mod proto;

pub use config::Config;
use proto::Api;

include_cpp! {
    #include "valhalla.h"

    safety!(unsafe)

    generate!("ValhallaClient")
    generate!("new_valhalla_client")
}

#[cfg(test)]
mod tests {
    use crate::ffi;
    use cxx::let_cxx_string;

    #[test]
    fn test_route() {
        use crate::Config;
        use std::{fs::File, io::BufReader};

        cxx::let_cxx_string!(config_cxx_string = "valhalla.json");
        let client = ffi::new_valhalla_client(&config_cxx_string);

        let request_raw = r#"
        	{
           "locations":[
              {
                 "lat":52.499078,
                 "lon":13.418230,
                 "name":"Kottbusser Tor",
                 "type":"break"
              },
              {
                 "lat":52.487331,
                 "lon":13.425042,
                 "name":"Hermannplatz",
                 "type":"break"
              }
           ],
           "costing":"auto",
           "costing_options":{
              "auto":{
                 "country_crossing_penalty":2000.0
              }
           },
           "units":"miles",
           "id":"my_work_route"
            }"#;

        cxx::let_cxx_string!(request_cxx = request_raw);
        let response = client.as_ref().unwrap().route(&request_cxx);
        println!("route: {:?}", response);
    }
}
