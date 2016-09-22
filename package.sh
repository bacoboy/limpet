#!/bin/bash

VERSION=`grep '^version' Cargo.toml | cut -d '"' -f 2`
echo "Packaging version ${VERSION}..."
cargo clean
docker run --rm -v `pwd`:/source -ti schickling/rust cargo build --release
zip -j limpet_${VERSION}_linux_amd64.zip  target/release/limpet

