use anyhow::Result;
use std::{fs::File, io::BufReader};
use valhalla::{
    proto::{options::Units, Api, Costing, CostingOptions, LatLng, Location, Options},
    Actor,
    Config,
};

fn main() -> Result<()> {
    let config_file = File::open("valhalla.json")?;
    let config_reader = BufReader::new(config_file);

    let config: Config = serde_json::from_reader(config_reader)?;

    let mut actor = Actor::new(&config);

    let request = Api {
        options: Some(Options {
            locations: vec![
                Location {
                    ll: Some(LatLng {
                        lat: Some(52.499078),
                        lng: Some(13.418230),
                    }),
                    name: Some("Kottbusser Tor".into()),
                    ..Default::default()
                },
                Location {
                    ll: Some(LatLng {
                        lat: Some(52.487331),
                        lng: Some(13.425042),
                    }),
                    name: Some("Hermannplatz".into()),
                    ..Default::default()
                },
            ],
            costing: Some(Costing::Auto as i32),
            costing_options: vec![CostingOptions {
                country_crossing_penalty: Some(2000.0),
                ..Default::default()
            }],
            units: Some(Units::Kilometers as i32),
            id: Some("kotti_to_hermannplatz".into()),
            ..Default::default()
        }),
        ..Default::default()
    };

    let response = actor.route(&request)?;

    println!("{}", response);

    Ok(())
}
