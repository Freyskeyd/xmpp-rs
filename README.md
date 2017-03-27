# xmpp-rs

`xmpp-rs` is an XMPP client library (for now). Under active development.

[![Build Status](https://travis-ci.org/Freyskeyd/xmpp-rs.svg?branch=master)](https://travis-ci.org/Freyskeyd/xmpp-rs)

## Install

*Cargo.toml*

```toml
xmpp-rs = "0.1"
```

```
extern crate xmpp;

```
## Build

To build xmpp-rs:

`cargo build --release`

## Discuss && Get help

## ROADMAP

- [ ] Root implementation
    - [x] TCP Connection
    - [x] TLS Connection and negociation
    - [ ] PLAIN authentication
    - [ ] Ping IQ client
    - [ ] Send First presence

## XEP

- [ ] [XEP-0199: XMPP Ping](https://xmpp.org/extensions/xep-0199.html)

## License

xmpp-rs is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0), with portions covered by various
BSD-like licenses.

See LICENSE-APACHE, and LICENSE-MIT for details.
