use valhalla::{
    proto::{Api, CostingOptions, LatLng, Location, Options},
    ProtobufActor,
};

#[test]
fn test_proto_route() {
    let actor = ProtobufActor::new("tests/valhalla.json");
    let api = Api {
        options: Some(Options {
            id: Some("kotti_to_hermannplatz".into()),
            units: Some(0),
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
                    name: Some("Kottbusser Tor".into()),
                    ..Default::default()
                },
            ],
            costing: Some(0),
            costing_options: vec![CostingOptions {
                transport_type: Some("car".into()),
                alley_factor: Some(1.0),
                use_highways: Some(0.5),
                use_tolls: Some(0.5),
                use_distance: Some(0.),
                height: Some(1.6),
                width: Some(1.9),
                shortest: Some(true),
                costing: Some(0),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    };

    let r = actor.route(&api).unwrap();
    println!("{:?}", r);
}
