# valhalla-rs (WIP)

[![license: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/ewilken/valhalla-rs#license)

Rust wrapper around the [Valhalla](https://github.com/valhalla/valhalla) routing engine.

This aims to replicate the API exposed for [the in-tree Python bindings](https://github.com/valhalla/valhalla/tree/master/src/bindings/python) as Rust-familiar typing. For data exchange with the C++ signature we're probably just gonna string-convert back and forth for now.

## OSRM Data Setup

    git submodule update --init --recursive

    # install Valhalla CLIs
    cd valhalla
    mkdir build
    cd build
    cmake .. -DCMAKE_BUILD_TYPE=Release
    make -j$(nproc) # for macos, use: make -j$(sysctl -n hw.physicalcpu)
    sudo make install
    cd ../..

    # download some data and make tiles out of it
    # NOTE: you can feed multiple extracts into pbfgraphbuilder
    cd mapdata
    wget http://download.geofabrik.de/europe/germany-latest.osm.pbf http://download.geofabrik.de/europe/netherlands-latest.osm.pbf

    # get the config and setup
    mkdir -p valhalla_tiles
    valhalla_build_config --mjolnir-tile-dir ${PWD}/valhalla_tiles --mjolnir-tile-extract ${PWD}/valhalla_tiles.tar --mjolnir-timezone ${PWD}/valhalla_tiles/timezones.sqlite --mjolnir-admin ${PWD}/valhalla_tiles/admins.sqlite > valhalla.json

    # build routing tiles
    valhalla_build_tiles -c valhalla.json germany-latest.osm.pbf netherlands-latest.osm.pbf

    # tar it up for running the server
    find valhalla_tiles | sort -n | tar cf valhalla_tiles.tar --no-recursion -T -

## Development

### Testing

    cargo test -vvv -- --nocapture


### macOS

    brew install automake cmake libtool protobuf-c boost-python libspatialite pkg-config sqlite3 jq curl wget czmq lz4 spatialite-tools unzip luajit bash coreutils binutils
    export PATH="/usr/local/opt/binutils/bin:/usr/local/opt/coreutils/libexec/gnubin:$PATH"

    # install prime_server
    git clone https://github.com/kevinkreiser/prime_server.git
    cd prime_server
    git checkout 0.7.0
    git submodule update --init --recursive
    ./autogen.sh
    aclocal -I m4
    autoheader --warnings=no-portability
    autoconf --warnings=no-portability
    automake --force-missing --add-missing
    ./configure
    make test -j8
    sudo make install
    cd .. && rm -rf prime_server

    git submodule update --init --recursive

    cargo build -vv

#### libgeos linking
If your version of geos is below 3.10.0 you have to remove the line with 
    .cxxflag("-DGEOS_INLINE")
in the build.rs file. See https://github.com/valhalla/valhalla/issues/3388#issuecomment-961934388 .

### Linux (`.deb` capable distros)

```bash
sudo apt install cmake make libtool pkg-config llvm-dev libclang-dev clang curl unzip jq lcov protobuf-compiler \
	vim-common locales libboost-all-dev libcurl4-openssl-dev zlib1g-dev liblz4-dev libprotobuf-dev libgeos-dev \
	libgeos++-dev libluajit-5.1-dev libspatialite-dev libsqlite3-dev wget sqlite3 spatialite-bin
```

You need to switch to clang if you have gcc installed
```bash
sudo update-alternatives --config cc
sudo update-alternatives --config cc++
```

## License

`valhalla-rs` is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
