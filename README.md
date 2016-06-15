# [Limpet](http://limpet.io/)
[![Build Status](https://travis-ci.org/bacoboy/limpet.svg?branch=master)](https://travis-ci.org/bacoboy/limpet)

A [liquid template](http://liquidmarkup.org/) command line tool.

Keep an eye on [this github issue](https://github.com/cobalt-org/liquid-rust/issues/11) for details
on which filters are implements or not.  The list isn't as complete as I thought when I started this.

## Download
There is a linux 64 bit pre-compiled binary available at bintray

[ ![Download](https://api.bintray.com/packages/bacoboy/limpet/limpet/images/download.svg) ](https://bintray.com/bacoboy/limpet/limpet/_latestVersion)

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