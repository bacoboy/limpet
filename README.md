# [Limpet](http://limpet.io/)
[![Build Status](https://travis-ci.org/bacoboy/limpet.svg?branch=master)](https://travis-ci.org/bacoboy/limpet)

A [liquid template](http://liquidmarkup.org/) command line tool.

## Usage

## Compile
To compile for ubuntu/amazon linux:
```
docker run -v `pwd`:/source -ti schickling/rust cargo build --release
```
The binary will be `target/release/limpet`

TODO: publish via Travis-CI