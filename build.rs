fn cmake_build() -> String {
    let build_type = if Ok("release".to_owned()) == std::env::var("PROFILE") {
        "Release"
    } else {
        "Debug"
    };

    let mut conf = cmake::Config::new("valhalla");
    if cfg!(target_os = "macos") {
        conf.cxxflag("-DGEOS_INLINE");
    }

    conf.cxxflag("-pthread");

    let dst = conf
        .define("ENABLE_TESTS", "OFF")
        .define("ENABLE_BENCHMARKS", "OFF")
        .define("ENABLE_TOOLS", "OFF")
        .define("ENABLE_DATA_TOOLS", "OFF")
        .define("ENABLE_HTTP", "OFF")
        .define("ENABLE_SERVICES", "OFF")
        .define("ENABLE_PYTHON_BINDINGS", "OFF")
        .define("ENABLE_THREAD_SAFE_TILE_REF_COUNT", "ON")
        // .define("BUILD_SHARED_LIBS", "OFF") // turning this on throws a linker error
        // .define("ENABLE_STATIC_LIBRARY_MODULES", "ON") // turning this on throws a linker error
        .define("CMAKE_BUILD_TYPE", build_type)
        .build();

    dst.display().to_string()
}

fn generate_autocxx_bindings(out_dir: String) {
    let includes = vec![
        "src".to_string(),
        "include".to_string(),
        format!("{}/include", out_dir),
        "valhalla/third_party/date/include".to_string(),
        "valhalla/third_party/rapidjson/include".to_string(),
    ];

    let mut b = autocxx_build::Builder::new("src/lib.rs", &includes)
        .extra_clang_args(&["-std=c++14"])
        .expect_build();

    b.opt_level(2)
        .cpp(true)
        .flag_if_supported("-std=c++14")
        .flag_if_supported("/std:c++14")
        .file("src/valhalla.cc")
        .compile("valhalla-wrapper");

    println!("cargo:rerun-if-changed=src/lib.rs");

    println!("cargo:rustc-link-search={}/lib", out_dir);
    println!("cargo:rustc-link-lib=static=valhalla");

    // println!("cargo:rustc-link-lib=protoc"); // TODO - check whether we can leave this out
    println!("cargo:rustc-link-lib=protobuf");
    // println!("cargo:rustc-link-lib=zmq"); // TODO - check whether we can leave this out
    println!("cargo:rustc-link-lib=z");
    // println!("cargo:rustc-link-lib=boost_program_options"); // TODO - check whether we can leave this out
    // println!("cargo:rustc-link-lib=spatialite"); // TODO - check whether we can leave this out
    // println!("cargo:rustc-link-lib=sqlite3"); // TODO - check whether we can leave this out
    // println!("cargo:rustc-link-lib=luajit-5.1"); // TODO - check whether we can leave this out
    // println!("cargo:rustc-link-lib=geos"); // TODO - check whether we can leave this out
    // println!("cargo:rustc-link-lib=gssapi_krb5"); // TODO - check whether we can leave this out
    // println!("cargo:rustc-link-lib=m"); // TODO - check whether we can leave this out

    // // println!("cargo:rustc-link-lib=stdc++"); // TODO - check whether we can leave this out
}

fn compile_protos() {
    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");

    if cfg!(feature = "async-graphql-annotation") {
        // TODO - Get the selection by path right instead of just matching by global names.
        // https://docs.rs/prost-build/latest/prost_build/struct.Config.html#examples-3
        config.type_attribute("Type", "#[derive(async_graphql::Enum)]");
        config.type_attribute("SideOfStreet", "#[derive(async_graphql::Enum)]");
        config.type_attribute("PreferredSide", "#[derive(async_graphql::Enum)]");
        config.type_attribute("State", "#[derive(async_graphql::Enum)]");
        config.type_attribute("RoadClass", "#[derive(async_graphql::Enum)]");
        config.type_attribute("Units", "#[derive(async_graphql::Enum)]");
        config.type_attribute("Format", "#[derive(async_graphql::Enum)]");
        config.type_attribute("Action", "#[derive(async_graphql::Enum)]");
        config.type_attribute("DateTimeType", "#[derive(async_graphql::Enum)]");
        config.type_attribute("ShapeMatch", "#[derive(async_graphql::Enum)]");
        config.type_attribute("FilterAction", "#[derive(async_graphql::Enum)]");
        config.type_attribute("DirectionsType", "#[derive(async_graphql::Enum)]");
        config.type_attribute("ShapeFormat", "#[derive(async_graphql::Enum)]");
        config.type_attribute("Costing", "#[derive(async_graphql::Enum)]");
        config.type_attribute("Impact", "#[derive(async_graphql::Enum)]");
        config.type_attribute("Traversability", "#[derive(async_graphql::Enum)]");
        config.type_attribute("Use", "#[derive(async_graphql::Enum)]");
        config.type_attribute("Surface", "#[derive(async_graphql::Enum)]");
        config.type_attribute("TravelMode", "#[derive(async_graphql::Enum)]");
        config.type_attribute("VehicleType", "#[derive(async_graphql::Enum)]");
        config.type_attribute("PedestrianType", "#[derive(async_graphql::Enum)]");
        config.type_attribute("BicycleType", "#[derive(async_graphql::Enum)]");
        config.type_attribute("TransitType", "#[derive(async_graphql::Enum)]");
        config.type_attribute("CycleLane", "#[derive(async_graphql::Enum)]");
        config.type_attribute("SacScale", "#[derive(async_graphql::Enum)]");
        config.type_attribute("Sidewalk", "#[derive(async_graphql::Enum)]");
        config.type_attribute("CardinalDirection", "#[derive(async_graphql::Enum)]");
        config.type_attribute("BssManeuverType", "#[derive(async_graphql::Enum)]");
        config.type_attribute("StatisticType", "#[derive(async_graphql::Enum)]");
    }

    config
        .compile_protos(&["valhalla/proto/api.proto"], &["valhalla/proto/"])
        .expect("compiling protos");
}

fn main() {
    let out_dir = cmake_build();
    println!("OUT_DIR: {}", out_dir);
    generate_autocxx_bindings(out_dir);
    compile_protos();
}
