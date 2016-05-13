# [Limpet](http://limpet.io/)
[![Build Status](https://travis-ci.org/bacoboy/limpet.svg?branch=master)](https://travis-ci.org/bacoboy/limpet)

A [liquid template](http://liquidmarkup.org/) command line tool.

## Usage

## Compile
To compile for ubuntu/amazon linux:

```
% docker run --rm -v `pwd`:/source -ti schickling/rust cargo build --release
[snip]
% file target/release/limpet
target/release/limpet: ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked (uses shared libs), for GNU/Linux 2.6.26, not stripped
```

The binary will be `target/release/limpet` and is currently under around 2MB

TODO: publish via Travis-CI