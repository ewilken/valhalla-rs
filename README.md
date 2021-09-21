# valhalla-rs (WIP)

[![license: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/ewilken/valhalla-rs#license)

Rust wrapper for the [Valhalla](https://github.com/valhalla/valhalla) routing engine.

This aims to replicate the API exposed for [the in-tree Python bindings](https://github.com/valhalla/valhalla/tree/master/src/bindings/python) as Rust-familiar typing. For data exchange with the C++ signature we're probably just gonna string-convert back and forth for now.

## Development

### macOS

    brew install automake cmake libtool protobuf-c boost-python libspatialite pkg-config sqlite3 jq curl wget czmq lz4 spatialite-tools unzip luajit bash coreutils binutils
    export PATH="/usr/local/opt/binutils/bin:/usr/local/opt/coreutils/libexec/gnubin:$PATH"

    # install prime_server
    git clone https://github.com/kevinkreiser/prime_server.git
    cd prime_server
    git checkout 0.7.0
    git submodule update --init --recursive
    aclocal -I m4
    autoheader --warnings=no-portability
    autoconf --warnings=no-portability
    automake --force-missing --add-missing
    ./configure
    make test -j8
    sudo make install
    cd .. && rm -r prime_server

    git submodule update --init --recursive

    cargo build -vv

## License

`valhalla-rs` is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
