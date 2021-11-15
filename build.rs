fn compile() -> String {
    let dst = cmake::Config::new("valhalla")
        // .define("ENABLE_PYTHON_BINDINGS", "ON")
        // .cxxflag("-DGEOS_INLINE")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("ENABLE_PYTHON_BINDINGS", "OFF")
        .define("ENABLE_TOOLS", "OFF")
        .define("ENABLE_HTTP", "OFF")
        .define("ENABLE_SERVICES", "OFF")
        .define("ENABLE_DATA_TOOLS", "OFF")
        .define("ENABLE_STATIC_LIBRARY_MODULES", "ON")
        .define("BUILD_SHARED_LIBS", "OFF")
        .cxxflag("-fPIC")
        .build();

    dst.display().to_string()
}

fn generate_bindings(out_dir: String) {
    let includes = vec![
            "src".to_string(),
            format!("{}/include", out_dir),
            "valhalla/third_party/date/include".to_string(),
            "valhalla/third_party/rapidjson/include".to_string(),
        ];
        
    let mut b = autocxx_build::Builder::new("src/lib.rs", &includes)
		.extra_clang_args(&["-std=c++14"])
		.expect_build();
    
    println!("cargo:rustc-link-search=native={}/build/src", out_dir);
    println!("cargo:rustc-link-lib=static=valhalla");

    println!("cargo:rustc-link-search=native={}/build/src/baldr", out_dir);
    println!("cargo:rustc-link-lib=static=valhalla-baldr");


    println!("cargo:rustc-link-search=native={}/build/src/meili", out_dir);
    println!("cargo:rustc-link-lib=static=valhalla-meili");


    println!("cargo:rustc-link-search=native={}/build/src/midgard", out_dir);
    println!("cargo:rustc-link-lib=static=valhalla-midgard");


    println!("cargo:rustc-link-search=native={}/build/src/odin", out_dir);
    println!("cargo:rustc-link-lib=static=valhalla-odin");


    println!("cargo:rustc-link-search=native={}/build/src/sif", out_dir);
    println!("cargo:rustc-link-lib=static=valhalla-sif");


    println!("cargo:rustc-link-search=native={}/build/src/skadi", out_dir);
    println!("cargo:rustc-link-lib=static=valhalla-skadi");


    println!("cargo:rustc-link-search=native={}/build/src/tyr", out_dir);
    println!("cargo:rustc-link-lib=static=valhalla-tyr");


    println!("cargo:rustc-link-search=native={}/build/src/thor", out_dir);
    println!("cargo:rustc-link-lib=static=valhalla-thor");

    println!("cargo:rustc-link-search=native={}/build/src/valhalla/proto", out_dir);
    println!("cargo:rustc-link-lib=static=valhalla-proto");

    b.opt_level(2)
    .cpp(true)
    .flag_if_supported("-std=c++14")
    .flag_if_supported("/std:c++14")
    .file("src/actor_wrapper.cc")
    .compile("valhalla-wrapper");

    println!("cargo:rerun-if-changed=src/lib.rs");

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=valhalla-wrapper");
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
    generate_bindings(out_dir);
    compile_protos();
}
