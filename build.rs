fn compile() -> String {
    let build_type = if Ok("release".to_owned()) == std::env::var("PROFILE") {
        "Release"
    } else {
        "Debug"
    };

    let mut conf = cmake::Config::new("valhalla");
    if cfg!(target_os = "macos") {
        conf.cxxflag("-DGEOS_INLINE");
    }

    let dst = conf
        .define("ENABLE_TESTS", "OFF")
        .define("ENABLE_BENCHMARKS", "OFF")
        .define("ENABLE_HTTP", "OFF")
        .define("ENABLE_SERVICES", "OFF")
        .define("ENABLE_PYTHON_BINDINGS", "OFF")
        .define("CMAKE_BUILD_TYPE", build_type)
        .build();

    dst.display().to_string()
}

fn generate_bindings(out_dir: String) {
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
    println!("cargo:rustc-link-lib=valhalla");

    //println!("cargo:rustc-link-lib=prime_server");
    println!("cargo:rustc-link-lib=protoc");
    println!("cargo:rustc-link-lib=protobuf");
    println!("cargo:rustc-link-lib=zmq");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=boost_program_options");
    println!("cargo:rustc-link-lib=curl");
    println!("cargo:rustc-link-lib=spatialite");
    println!("cargo:rustc-link-lib=sqlite3");
    println!("cargo:rustc-link-lib=luajit-5.1");
    println!("cargo:rustc-link-lib=geos");
    println!("cargo:rustc-link-lib=gssapi_krb5");
}

fn compile_protos() {
    tonic_build::configure()
        .build_client(false)
        .build_server(false)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .format(false)
        .compile(&["valhalla/proto/api.proto"], &["valhalla/proto/"])
        .expect("compiling protos");
}

fn main() {
    let out_dir = compile();
    println!("OUT_DIR: {}", out_dir);
    generate_bindings(out_dir);
    compile_protos();
}
