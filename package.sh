#!/bin/bash

# finish this upload part later...
#if [ -z ${BINTRAY_API_KEY} ]; then
#    echo "BINTRAY_API_KEY environment not set."
#    exit 1
#fi

VERSION=`grep '^version' Cargo.toml | cut -d '"' -f 2`
echo "Packaging version ${VERSION}..."
cargo clean
docker run --rm -v `pwd`:/source -ti schickling/rust cargo build --release
zip -j limpet_${VERSION}_linux_amd64.zip  target/release/limpet

#curl -T limpet_${VERSION}_linux_amd64.zip -ubacoboy:${BINTRAY_API_KEY} -H "X-Bintray-Package:limpet" -H "X-Bintray-Version:${VERSION}" https://api.bintray.com/content/bacoboy/limpet/limpet
